use crate::models::ids::UserId;
use crate::models::user::User;
use crate::services::db::DbPool;
use shaku::{Component, Interface};
use std::sync::Arc;

pub trait UserServiceInterface: Interface {}

#[derive(Component)]
#[shaku(interface = UserServiceInterface)]
pub struct UserService {
    #[shaku(inject)]
    pub db: Arc<DbPool>,
}

impl UserServiceInterface for UserService {}

impl UserService {
    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, sqlx::Error> {
        #[cfg(feature = "ssr")]
        {
            use argon2::{Argon2, PasswordHasher};
            use argon2::password_hash::SaltString;

            let salt = SaltString::generate(&mut rand::thread_rng());
            let hashed = Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|_| sqlx::Error::Configuration("Password hashing failed".into()))?
                .to_string();

            sqlx::query_as::<_, (i64, String, String, String)>(
                "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?) RETURNING id, username, email, created_at"
            )
            .bind(username)
            .bind(email)
            .bind(hashed)
            .fetch_one(self.db.inner())
            .await
            .map(|(id, username, email, _)| User {
                id: UserId(id),
                username,
                email,
                created_at: None,
            })
        }
        #[cfg(not(feature = "ssr"))]
        {
            Err(sqlx::Error::Configuration("Server only".into()))
        }
    }

    pub async fn get_by_email(&self, email: String) -> Result<Option<User>, sqlx::Error> {
        #[cfg(feature = "ssr")]
        {
            sqlx::query_as::<_, (i64, String, String)>(
                "SELECT id, username, email FROM users WHERE email = ? LIMIT 1"
            )
            .bind(email)
            .fetch_optional(self.db.inner())
            .await
            .map(|opt| {
                opt.map(|(id, username, email)| User {
                    id: UserId(id),
                    username,
                    email,
                    created_at: None,
                })
            })
        }
        #[cfg(not(feature = "ssr"))]
        {
            Err(sqlx::Error::Configuration("Server only".into()))
        }
    }

    pub async fn verify_password(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<User>, sqlx::Error> {
        #[cfg(feature = "ssr")]
        {
            use argon2::{Argon2, PasswordHash, PasswordVerifier};

            let row = sqlx::query_as::<_, (i64, String, String, String)>(
                "SELECT id, username, email, password_hash FROM users WHERE email = ? LIMIT 1"
            )
            .bind(&email)
            .fetch_optional(self.db.inner())
            .await?;

            if let Some((id, username, email, hash)) = row {
                let parsed_hash = PasswordHash::new(&hash)
                    .map_err(|_| sqlx::Error::Configuration("Invalid hash".into()))?;

                match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
                    Ok(()) => Ok(Some(User {
                        id: UserId(id),
                        username,
                        email,
                        created_at: None,
                    })),
                    Err(_) => Ok(None),
                }
            } else {
                Ok(None)
            }
        }
        #[cfg(not(feature = "ssr"))]
        {
            Err(sqlx::Error::Configuration("Server only".into()))
        }
    }

    pub async fn create_session(
        &self,
        user_id: UserId,
    ) -> Result<String, sqlx::Error> {
        #[cfg(feature = "ssr")]
        {
            use chrono::Utc;
            use uuid::Uuid;

            let token = Uuid::new_v4().to_string();
            let expires_at = (Utc::now() + chrono::Duration::days(30)).to_rfc3339();

            sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)")
                .bind(&token)
                .bind(user_id.0)
                .bind(expires_at)
                .execute(self.db.inner())
                .await?;

            Ok(token)
        }
        #[cfg(not(feature = "ssr"))]
        {
            Err(sqlx::Error::Configuration("Server only".into()))
        }
    }

    pub async fn get_session_user(&self, token: String) -> Result<Option<User>, sqlx::Error> {
        #[cfg(feature = "ssr")]
        {
            sqlx::query_as::<_, (i64, String, String)>(
                "SELECT u.id, u.username, u.email FROM users u
                 JOIN sessions s ON u.id = s.user_id
                 WHERE s.id = ? AND s.expires_at > datetime('now')
                 LIMIT 1"
            )
            .bind(token)
            .fetch_optional(self.db.inner())
            .await
            .map(|opt| {
                opt.map(|(id, username, email)| User {
                    id: UserId(id),
                    username,
                    email,
                    created_at: None,
                })
            })
        }
        #[cfg(not(feature = "ssr"))]
        {
            Err(sqlx::Error::Configuration("Server only".into()))
        }
    }

    pub async fn delete_session(&self, token: String) -> Result<(), sqlx::Error> {
        #[cfg(feature = "ssr")]
        {
            sqlx::query("DELETE FROM sessions WHERE id = ?")
                .bind(token)
                .execute(self.db.inner())
                .await?;
            Ok(())
        }
        #[cfg(not(feature = "ssr"))]
        {
            Err(sqlx::Error::Configuration("Server only".into()))
        }
    }
}
