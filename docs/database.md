# Database module design

## Purpose

`skywhale_core::db` provides a stable boundary between repositories and SQLx
execution types. Repositories depend on `DbSession`, not directly on
`sqlx::Pool`, `sqlx::Transaction`, or `sqlx::pool::PoolConnection`.

The application/service layer chooses the execution context. This keeps the
choice of pooling, connection affinity, and transaction scope outside the
repository boundary while allowing repository methods to retain one parameter
shape.

```rust
async fn find_user(
    &self,
    session: &mut skywhale_core::db::sqlite::DbSession<'_>,
    id: i64,
) -> skywhale_core::Result<User> {
    sqlx::query_as("select id, name from users where id = ?")
        .bind(id)
        .fetch_one(session.executor())
        .await
        .map_err(/* convert sqlx::Error to the application error */)
}
```

## Execution contexts

`Database<DB>` creates one of the following `DbSession` variants.

| Entry point | Variant | Intended use |
| --- | --- | --- |
| `database.pool()` | `DbSession::Pool` | Ordinary repository work. SQLx selects a pooled connection per operation. |
| `database.conn().await` | `DbSession::Conn` | Work that must retain one checked-out connection. |
| `database.tx().await` | `DbSession::Tx` | Atomic work that must commit or roll back together. |

All variants expose `session.executor()`, which implements `sqlx::Executor`.
Therefore a repository method accepts a single `&mut DbSession` argument and
does not need overloads or a generic parameter for each SQLx concrete type.

## Transaction policy

`DbSession::begin()` starts a transaction from a pool or connection, or a
nested transaction from an existing transaction. Nested transactions rely on
the SQLx driver's savepoint behavior.

`commit(self)` and `rollback(self)` consume the session.

- For `DbSession::Tx`, they issue SQLx commit or rollback operations.
- For `DbSession::Pool` and `DbSession::Conn`, they intentionally return
  `Ok(())` without issuing database work.

This no-op behavior is deliberate: callers may retain the common `DbSession`
boundary without branching on its concrete execution context. Code that
requires transactional atomicity must obtain the context through `tx()` (or
call `begin()`) before invoking repositories.

## SQLx compatibility boundary

The module targets SQLx 0.8. SQLx no longer implements `Executor` directly for
`Transaction` and `PoolConnection`; both dereference to their underlying
connection. `ExecutorImpl` forwards transaction and connection calls through
`&mut **transaction` and `&mut **connection` respectively. Keep this explicit
forwarding when upgrading SQLx so that compatibility is localized to this
module rather than spreading into repositories.

`ExecutorImpl` is public only because it is the return type of the public
`DbSession::executor()` method. Its `handle` field is `pub(crate)`, and callers
should construct executors only through `session.executor()`.

## Drivers and configuration

`DatabaseConfig` currently contains only the connection URL and maximum pool
size. Type aliases are available for SQLite, PostgreSQL, and MySQL under
`skywhale_core::db::{sqlite, postgres, mysql}`.

Pool lifecycle tuning, driver-specific connect options, migrations, and a UOW
convenience API are intentionally outside this module's current scope. Add
them when an application use case requires them; they should not alter the
repository boundary described above.

## Verification contract

The module tests the behavior that defines this boundary:

1. One repository method operates with pool, transaction, and connection
   sessions.
2. A committed transaction persists its change.
3. Rolling back a nested transaction preserves the outer transaction's change.

The SQLite test pool is limited to one connection because `sqlite::memory:`
databases are connection-local.
