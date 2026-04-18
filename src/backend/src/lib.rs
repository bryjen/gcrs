pub mod models;
pub mod colors;

#[cfg(feature = "ssr")]
pub mod services;

#[cfg(feature = "ssr")]
pub use services::{DbPool, RepoService, GitService, UserService};

pub use models::user::User;
pub use colors::get_language_color;
