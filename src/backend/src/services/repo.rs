use crate::models::git::Repository;
use crate::models::ids::{RepoId, UserId};
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

    pub fn demo_repos() -> Vec<Repository> {
        vec![
            Repository {
                id: RepoId(1),
                owner_id: UserId(1),
                owner_name: "bryjen".to_string(),
                name: "kernel".to_string(),
                lower_name: "kernel".to_string(),
                description: Some("Low-level kernel implementation".to_string()),
                is_private: false,
                is_fork: false,
                fork_id: None,
                default_branch: "main".to_string(),
                language: "C".to_string(),
                language_color_hex: "#555555".to_string(),
            },
            Repository {
                id: RepoId(2),
                owner_id: UserId(1),
                owner_name: "bryjen".to_string(),
                name: "openclaw".to_string(),
                lower_name: "openclaw".to_string(),
                description: Some("Open-source claw framework".to_string()),
                is_private: true,
                is_fork: false,
                fork_id: None,
                default_branch: "main".to_string(),
                language: "Rust".to_string(),
                language_color_hex: "#dea584".to_string(),
            },
            Repository {
                id: RepoId(3),
                owner_id: UserId(1),
                owner_name: "bryjen".to_string(),
                name: "dotfiles".to_string(),
                lower_name: "dotfiles".to_string(),
                description: Some("Configuration files and scripts".to_string()),
                is_private: false,
                is_fork: false,
                fork_id: None,
                default_branch: "main".to_string(),
                language: "Shell".to_string(),
                language_color_hex: "#89e051".to_string(),
            },
            Repository {
                id: RepoId(4),
                owner_id: UserId(1),
                owner_name: "bryjen".to_string(),
                name: "noctua".to_string(),
                lower_name: "noctua".to_string(),
                description: Some("C# ML framework".to_string()),
                is_private: true,
                is_fork: false,
                fork_id: None,
                default_branch: "main".to_string(),
                language: "C#".to_string(),
                language_color_hex: "#178600".to_string(),
            },
            Repository {
                id: RepoId(5),
                owner_id: UserId(1),
                owner_name: "bryjen".to_string(),
                name: "libsignal-rs".to_string(),
                lower_name: "libsignal-rs".to_string(),
                description: Some("Signal protocol bindings for Rust".to_string()),
                is_private: false,
                is_fork: false,
                fork_id: None,
                default_branch: "main".to_string(),
                language: "Rust".to_string(),
                language_color_hex: "#dea584".to_string(),
            },
            Repository {
                id: RepoId(6),
                owner_id: UserId(1),
                owner_name: "bryjen".to_string(),
                name: "infra".to_string(),
                lower_name: "infra".to_string(),
                description: Some("Infrastructure as code with Nix".to_string()),
                is_private: true,
                is_fork: false,
                fork_id: None,
                default_branch: "main".to_string(),
                language: "Nix".to_string(),
                language_color_hex: "#7e7eff".to_string(),
            },
        ]
    }

    pub async fn list_demo(&self) -> Result<Vec<Repository>, sqlx::Error> {
        Ok(Self::demo_repos())
    }

    pub async fn get_demo(&self, id: RepoId) -> Result<Option<Repository>, sqlx::Error> {
        Ok(Self::demo_repos().into_iter().find(|r| r.id == id))
    }
}
