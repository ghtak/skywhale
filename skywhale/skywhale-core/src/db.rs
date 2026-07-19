//! Database execution contexts for repositories.
//!
//! [`DbSession`] is a boundary adapter: repositories receive one executor
//! shape, while the application layer decides whether a query uses the pool,
//! a dedicated connection, or a transaction.

use futures_core::{future::BoxFuture, stream::BoxStream};
use sqlx::Acquire;

use crate::{DatabaseConfig, Result, error::into_db_error};

/// Classifies database-driver errors retained in [`crate::Error::Database`].
///
/// Import this trait only at boundaries that need to translate a technical
/// database failure into a domain error such as `AlreadyExists`.
pub trait DatabaseErrorExt {
    fn is_unique_violation(&self) -> bool;
}

impl DatabaseErrorExt for crate::Error {
    fn is_unique_violation(&self) -> bool {
        let crate::Error::Database(error) = self else {
            return false;
        };

        matches!(
            error.downcast_ref::<sqlx::Error>(),
            Some(sqlx::Error::Database(database_error)) if database_error.is_unique_violation()
        )
    }
}

/// An executor adapter over a [`DbSession`].
#[derive(Debug)]
pub struct ExecutorImpl<'h, 'c, DB>
where
    DB: sqlx::Database,
    for<'e> &'e mut DB::Connection: sqlx::Executor<'e, Database = DB>,
{
    pub(crate) handle: &'h mut DbSession<'c, DB>,
}

/// A database execution context selected by the application layer.
///
/// [`Self::Pool`] executes through the connection pool, [`Self::Conn`] keeps a
/// connection checked out, and [`Self::Tx`] executes within a transaction.
/// `commit` and `rollback` perform database work only for [`Self::Tx`]; they
/// are no-ops for the other contexts so callers can retain one boundary type.
#[derive(Debug)]
pub enum DbSession<'c, DB>
where
    DB: sqlx::Database,
    for<'e> &'e mut DB::Connection: sqlx::Executor<'e, Database = DB>,
{
    Pool(sqlx::Pool<DB>),
    Tx(sqlx::Transaction<'c, DB>),
    Conn(sqlx::pool::PoolConnection<DB>),
}

impl<'c, DB> DbSession<'c, DB>
where
    DB: sqlx::Database,
    for<'e> &'e mut DB::Connection: sqlx::Executor<'e, Database = DB>,
{
    /// Starts a transaction, or a nested transaction when this session is a transaction.
    pub async fn begin(&mut self) -> Result<DbSession<'_, DB>> {
        let transaction = match self {
            Self::Pool(pool) => pool.begin().await,
            Self::Tx(transaction) => transaction.begin().await,
            Self::Conn(connection) => connection.begin().await,
        }
        .map_err(into_db_error)?;

        Ok(DbSession::Tx(transaction))
    }

    /// Commits this transaction. This is a no-op for pool and connection sessions.
    pub async fn commit(self) -> Result<()> {
        match self {
            Self::Pool(_) | Self::Conn(_) => Ok(()),
            Self::Tx(transaction) => transaction.commit().await.map_err(into_db_error),
        }
    }

    /// Rolls back this transaction. This is a no-op for pool and connection sessions.
    pub async fn rollback(self) -> Result<()> {
        match self {
            Self::Pool(_) | Self::Conn(_) => Ok(()),
            Self::Tx(transaction) => transaction.rollback().await.map_err(into_db_error),
        }
    }

    /// Returns the SQLx executor used by repository queries.
    pub fn executor(&mut self) -> ExecutorImpl<'_, 'c, DB> {
        ExecutorImpl { handle: self }
    }
}

impl<'h, 'c, DB> sqlx::Executor<'h> for ExecutorImpl<'h, 'c, DB>
where
    DB: sqlx::Database,
    for<'e> &'e mut DB::Connection: sqlx::Executor<'e, Database = DB>,
{
    type Database = DB;

    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxStream<'e, std::result::Result<sqlx::Either<DB::QueryResult, DB::Row>, sqlx::Error>>
    where
        'c: 'e,
        'h: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        match self.handle {
            DbSession::Pool(pool) => pool.fetch_many(query),
            DbSession::Tx(transaction) => (&mut **transaction).fetch_many(query),
            DbSession::Conn(connection) => (&mut **connection).fetch_many(query),
        }
    }

    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxFuture<'e, std::result::Result<Option<DB::Row>, sqlx::Error>>
    where
        'c: 'e,
        'h: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        match self.handle {
            DbSession::Pool(pool) => pool.fetch_optional(query),
            DbSession::Tx(transaction) => (&mut **transaction).fetch_optional(query),
            DbSession::Conn(connection) => (&mut **connection).fetch_optional(query),
        }
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [DB::TypeInfo],
    ) -> BoxFuture<'e, std::result::Result<DB::Statement<'q>, sqlx::Error>>
    where
        'c: 'e,
        'h: 'e,
    {
        match self.handle {
            DbSession::Pool(pool) => pool.prepare_with(sql, parameters),
            DbSession::Tx(transaction) => (&mut **transaction).prepare_with(sql, parameters),
            DbSession::Conn(connection) => (&mut **connection).prepare_with(sql, parameters),
        }
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, std::result::Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e,
        'h: 'e,
    {
        match self.handle {
            DbSession::Pool(pool) => pool.describe(sql),
            DbSession::Tx(transaction) => (&mut **transaction).describe(sql),
            DbSession::Conn(connection) => (&mut **connection).describe(sql),
        }
    }
}

/// A database pool with methods that create repository execution contexts.
#[derive(Debug)]
pub struct Database<DB: sqlx::Database> {
    pub(crate) inner: sqlx::Pool<DB>,
}

impl<DB: sqlx::Database> Database<DB> {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let inner = sqlx::pool::PoolOptions::<DB>::new()
            .max_connections(config.max_connections)
            .connect(&config.url)
            .await
            .map_err(into_db_error)?;

        Ok(Self { inner })
    }

    /// Creates a pooled execution context.
    pub fn pool(&self) -> DbSession<'_, DB>
    where
        for<'e> &'e mut DB::Connection: sqlx::Executor<'e, Database = DB>,
    {
        DbSession::Pool(self.inner.clone())
    }

    /// Starts a transaction execution context.
    pub async fn tx(&self) -> Result<DbSession<'_, DB>>
    where
        for<'e> &'e mut DB::Connection: sqlx::Executor<'e, Database = DB>,
    {
        self.inner
            .begin()
            .await
            .map(DbSession::Tx)
            .map_err(into_db_error)
    }

    /// Acquires a connection-bound execution context.
    pub async fn conn(&self) -> Result<DbSession<'_, DB>>
    where
        for<'e> &'e mut DB::Connection: sqlx::Executor<'e, Database = DB>,
    {
        self.inner
            .acquire()
            .await
            .map(DbSession::Conn)
            .map_err(into_db_error)
    }
}

pub mod sqlite {
    pub type Database = super::Database<sqlx::Sqlite>;
    pub type DbSession<'a> = super::DbSession<'a, sqlx::Sqlite>;
}

pub mod postgres {
    pub type Database = super::Database<sqlx::Postgres>;
    pub type DbSession<'a> = super::DbSession<'a, sqlx::Postgres>;
}

pub mod mysql {
    pub type Database = super::Database<sqlx::MySql>;
    pub type DbSession<'a> = super::DbSession<'a, sqlx::MySql>;
}

#[cfg(test)]
mod tests {
    use sqlx::Row;

    use super::{Database, DatabaseErrorExt};
    use crate::{DatabaseConfig, Result, error::into_db_error};

    type TestDatabase = Database<sqlx::Sqlite>;
    type TestSession<'c> = super::sqlite::DbSession<'c>;

    struct SampleRepository;

    impl SampleRepository {
        async fn get_id(&self, session: &mut TestSession<'_>) -> Result<i32> {
            let row = sqlx::query("select 1")
                .fetch_one(session.executor())
                .await
                .map_err(into_db_error)?;
            Ok(row.get(0))
        }
    }

    async fn database() -> TestDatabase {
        TestDatabase::new(&DatabaseConfig {
            url: "sqlite::memory:".to_owned(),
            max_connections: 1,
        })
        .await
        .expect("test database must connect")
    }

    async fn execute(session: &mut TestSession<'_>, sql: &str) -> Result<()> {
        sqlx::query(sql)
            .execute(session.executor())
            .await
            .map_err(into_db_error)?;
        Ok(())
    }

    async fn row_count(session: &mut TestSession<'_>) -> Result<i64> {
        let row = sqlx::query("select count(*) from sample")
            .fetch_one(session.executor())
            .await
            .map_err(into_db_error)?;
        Ok(row.get(0))
    }

    #[tokio::test]
    async fn repository_accepts_every_execution_context() {
        let database = database().await;
        let repository = SampleRepository;

        let mut pool = database.pool();
        assert_eq!(repository.get_id(&mut pool).await.unwrap(), 1);

        let mut transaction = database.tx().await.unwrap();
        assert_eq!(repository.get_id(&mut transaction).await.unwrap(), 1);
        transaction.commit().await.unwrap();

        let mut connection = database.conn().await.unwrap();
        assert_eq!(repository.get_id(&mut connection).await.unwrap(), 1);
    }

    #[tokio::test]
    async fn transaction_commit_persists_changes() {
        let database = database().await;
        let mut pool = database.pool();
        execute(
            &mut pool,
            "create table sample (id integer primary key, name text not null)",
        )
        .await
        .unwrap();
        drop(pool);

        let mut transaction = database.tx().await.unwrap();
        execute(
            &mut transaction,
            "insert into sample (id, name) values (1, 'committed')",
        )
        .await
        .unwrap();
        transaction.commit().await.unwrap();

        let mut pool = database.pool();
        assert_eq!(row_count(&mut pool).await.unwrap(), 1);
    }

    #[tokio::test]
    async fn nested_transaction_rollback_preserves_outer_changes() {
        let database = database().await;
        let mut pool = database.pool();
        execute(
            &mut pool,
            "create table sample (id integer primary key, name text not null)",
        )
        .await
        .unwrap();
        drop(pool);

        let mut outer = database.tx().await.unwrap();
        execute(
            &mut outer,
            "insert into sample (id, name) values (1, 'outer')",
        )
        .await
        .unwrap();

        let mut inner = outer.begin().await.unwrap();
        execute(
            &mut inner,
            "insert into sample (id, name) values (2, 'inner')",
        )
        .await
        .unwrap();
        inner.rollback().await.unwrap();

        outer.commit().await.unwrap();

        let mut pool = database.pool();
        assert_eq!(row_count(&mut pool).await.unwrap(), 1);
    }

    #[test]
    fn non_database_errors_are_not_unique_violations() {
        let error = crate::Error::Other(anyhow::anyhow!("unexpected failure"));

        assert!(!error.is_unique_violation());
    }

    #[tokio::test]
    async fn sqlite_unique_constraint_is_classified() {
        let database = database().await;
        let mut session = database.pool();
        execute(
            &mut session,
            "create table sample (id integer primary key, name text unique not null)",
        )
        .await
        .unwrap();
        execute(&mut session, "insert into sample (name) values ('same')")
            .await
            .unwrap();

        let error = sqlx::query("insert into sample (name) values ('same')")
            .execute(session.executor())
            .await
            .map_err(into_db_error)
            .unwrap_err();

        assert!(error.is_unique_violation());
    }
}
