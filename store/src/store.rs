// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use sqlx::database::HasStatement;
use sqlx::pool::PoolConnection;
use sqlx::{Acquire, Database, Describe, Either, Error, Execute, Executor, Pool, Sqlite};

#[derive(Debug, Clone)]
pub struct SqliteStore {
    pool: Pool<Sqlite>,
}

impl SqliteStore {
    pub async fn connect<T: AsRef<str>>(url: T) -> sqlx::Result<Self> {
        let conn = Pool::connect(url.as_ref()).await?;
        Ok(Self { pool: conn })
    }

    pub async fn migrate(&self) -> sqlx::Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn get_connection(&self) -> sqlx::Result<PoolConnection<Sqlite>> {
        self.pool.acquire().await
    }
}

impl<'c> Executor<'c> for &'c SqliteStore {
    type Database = Sqlite;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures::stream::BoxStream<
        'e,
        Result<
            Either<<Self::Database as Database>::QueryResult, <Self::Database as Database>::Row>,
            Error,
        >,
    >
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        self.pool.fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures::future::BoxFuture<'e, Result<Option<<Self::Database as Database>::Row>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        self.pool.fetch_optional(query)
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as Database>::TypeInfo],
    ) -> futures::future::BoxFuture<
        'e,
        Result<<Self::Database as HasStatement<'q>>::Statement, Error>,
    >
    where
        'c: 'e,
    {
        self.pool.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures::future::BoxFuture<'e, Result<Describe<Self::Database>, Error>>
    where
        'c: 'e,
    {
        self.pool.describe(sql)
    }
}
