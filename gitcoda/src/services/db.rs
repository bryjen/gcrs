use shaku::{Component, Interface};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;

pub trait DbPoolInterface: Interface {
    fn pool(&self) -> &sqlx::SqlitePool;
}

#[derive(Component, Clone)]
#[shaku(interface = DbPoolInterface)]
pub struct DbPool {
    pool: Arc<sqlx::SqlitePool>,
}

impl DbPoolInterface for DbPool {
    fn pool(&self) -> &sqlx::SqlitePool {
        &self.pool
    }
}

impl DbPool {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        DbPool {
            pool: Arc::new(pool),
        }
    }

    pub fn inner(&self) -> &sqlx::SqlitePool {
        &self.pool
    }

    pub async fn from_url(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        Ok(Self::new(pool))
    }
}
