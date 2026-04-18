use shaku::{Component, Interface};
use crate::services::repo::RepoService;
use std::sync::Arc;

pub trait GitServiceInterface: Interface {}

#[derive(Component)]
#[shaku(interface = GitServiceInterface)]
pub struct GitService {
    #[shaku(inject)]
    pub repo_svc: Arc<RepoService>,
}

impl GitServiceInterface for GitService {}

impl GitService {
    pub async fn fetch_commits(&self, repo_id: i64) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar::<_, String>(
            "SELECT hash FROM commits WHERE repo_id = ? ORDER BY created_at DESC"
        )
        .bind(repo_id)
        .fetch_all(self.repo_svc.db.inner())
        .await
    }

    pub async fn get_commit_count(&self, repo_id: i64) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM commits WHERE repo_id = ?"
        )
        .bind(repo_id)
        .fetch_one(self.repo_svc.db.inner())
        .await
    }
}
