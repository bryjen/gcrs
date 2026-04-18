use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::ids::UserId;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
}

pub use User as PublicUser;
