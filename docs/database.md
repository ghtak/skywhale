# Database module design

## Purpose

`skywhale_core::db` is the boundary between repositories and SQLx execution
types. A repository accepts `&mut DbSession`, while the application layer
chooses whether the operation uses a pool, a checked-out connection, or a
transaction. Repositories therefore do not depend directly on SQLx's
`Pool`, `Transaction`, or `PoolConnection` types.

## Execution contexts

| Entry point | Variant | Intended use |
| --- | --- | --- |
| `database.pool()` | `DbSession::Pool` | Ordinary repository work; SQLx acquires pooled connections as needed. |
| `database.conn().await` | `DbSession::Conn` | Work that needs one checked-out connection. |
| `database.tx().await` | `DbSession::Tx` | Work that must be committed or rolled back atomically. |

Each context exposes `session.executor()`. It forwards the SQLx `Executor`
operations used by repository queries, giving repositories one parameter
shape regardless of the context selected by their caller.

`begin()` starts a transaction from a pool or connection, or a nested
transaction from an existing transaction. `commit(self)` and `rollback(self)`
perform database work for `Tx` and intentionally return `Ok(())` for `Pool`
and `Conn`, preserving the common boundary type. Code that requires atomicity
must obtain a transaction through `tx()` or `begin()` before calling a
repository.

## SQLx compatibility

The module targets SQLx 0.8. `Transaction` and `PoolConnection` do not
directly implement `Executor`; they dereference to the underlying connection.
`ExecutorImpl` keeps that compatibility detail local by forwarding through
`&mut **transaction` and `&mut **connection`.

`ExecutorImpl` is public only as the return type of `DbSession::executor()`.
Its handle is `pub(crate)`, so callers construct it only through a session.

## Database error classification

SQLx failures are converted to `skywhale_core::Error::Database`, preserving
the original `sqlx::Error` as the source. `DatabaseErrorExt` can be imported
where a technical classification is needed:

```rust
use skywhale_core::db::DatabaseErrorExt;

if error.is_unique_violation() {
    // The service may translate this known constraint conflict to AlreadyExists.
}
```

The trait delegates to SQLx's driver-aware unique-violation classification and
does not parse driver messages or hard-code SQLSTATE values. It should be used
at a repository or service boundary that understands the business meaning of
the violated constraint. Not every unique constraint means the same domain
conflict, so this module does not automatically turn such errors into
`AlreadyExists`.

## Current scope and verification

`DatabaseConfig` currently provides a URL and maximum connection count. SQLite,
PostgreSQL, and MySQL aliases are provided. Driver-specific connection options,
pool lifecycle tuning, migrations, and a UOW convenience API are deferred until
an application requires them.

The test suite verifies that one repository works with all three execution
contexts, commits persist data, nested rollback preserves outer changes, and a
SQLite unique constraint is classified correctly. SQLite in-memory tests use a
one-connection pool because `sqlite::memory:` is connection-local.
