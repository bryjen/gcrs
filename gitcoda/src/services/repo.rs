use crate::services::db::DbPool;
use shaku::{Component, Interface};
use std::sync::Arc;

pub trait RepoServiceInterface: Interface {}

#[derive(Component)]
#[shaku(interface = RepoServiceInterface)]
pub struct RepoService {
    #[shaku(inject)]
    pub db: Arc<DbPool>,
}

impl RepoServiceInterface for RepoService {}

impl RepoService {
    pub async fn list(&self) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar::<_, String>("SELECT name FROM repositories")
            .fetch_all(self.db.inner())
            .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar::<_, String>("SELECT name FROM repositories WHERE id = ?")
            .bind(id)
            .fetch_optional(self.db.inner())
            .await
    }
}
