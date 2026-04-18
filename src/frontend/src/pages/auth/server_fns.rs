use gitcoda::User;
use leptos::prelude::*;

#[server]
pub async fn signup(
    username: String,
    email: String,
    password: String,
) -> Result<User, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::extract::Extension;
        use gitcoda::UserService;
        use leptos_axum::{extract, ResponseOptions};
        use std::sync::Arc;

        if username.is_empty() || email.is_empty() || password.is_empty() {
            return Err(ServerFnError::new("All fields required"));
        }

        if password.len() < 8 {
            return Err(ServerFnError::new("Password must be at least 8 characters"));
        }

        let Extension(user_service) = extract::<Extension<Arc<UserService>>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract service: {}", e)))?;

        let user = user_service
            .create_user(username, email, password.clone())
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to create user: {}", e)))?;

        // Create session
        let token = user_service
            .create_session(user.id)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to create session: {}", e)))?;

        // Set cookie via response options
        let options = use_context::<ResponseOptions>();
        if let Some(opts) = options {
            use http::header::{HeaderName, HeaderValue};
            opts.insert_header(
                HeaderName::from_static("set-cookie"),
                HeaderValue::from_str(&format!(
                    "auth_token={}; Path=/; HttpOnly; SameSite=Strict; Max-Age=2592000",
                    token
                ))
                .unwrap(),
            );
        }

        Ok(user)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server function only available on SSR"))
    }
}

#[server]
pub async fn login(email: String, password: String) -> Result<User, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::extract::Extension;
        use gitcoda::UserService;
        use leptos_axum::{extract, ResponseOptions};
        use std::sync::Arc;

        if email.is_empty() || password.is_empty() {
            return Err(ServerFnError::new("Email and password required"));
        }

        let Extension(user_service) = extract::<Extension<Arc<UserService>>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract service: {}", e)))?;

        let user = user_service
            .verify_password(email, password)
            .await
            .map_err(|e| ServerFnError::new(format!("Password verification failed: {}", e)))?
            .ok_or_else(|| ServerFnError::new("Invalid email or password"))?;

        // Create session
        let token = user_service
            .create_session(user.id)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to create session: {}", e)))?;

        // Set cookie via response options
        let options = use_context::<ResponseOptions>();
        if let Some(opts) = options {
            use http::header::{HeaderName, HeaderValue};
            opts.insert_header(
                HeaderName::from_static("set-cookie"),
                HeaderValue::from_str(&format!(
                    "auth_token={}; Path=/; HttpOnly; SameSite=Strict; Max-Age=2592000",
                    token
                ))
                .unwrap(),
            );
        }

        Ok(user)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server function only available on SSR"))
    }
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::extract::Extension;
        use axum_extra::extract::CookieJar;
        use gitcoda::UserService;
        use leptos_axum::{extract, ResponseOptions};
        use std::sync::Arc;

        let Extension(user_service) = extract::<Extension<Arc<UserService>>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract service: {}", e)))?;

        let jar = extract::<CookieJar>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract cookies: {}", e)))?;

        if let Some(cookie) = jar.get("auth_token") {
            let token = cookie.value().to_string();
            user_service
                .delete_session(token)
                .await
                .map_err(|e| ServerFnError::new(format!("Failed to delete session: {}", e)))?;
        }

        // Clear cookie via response options
        let options = use_context::<ResponseOptions>();
        if let Some(opts) = options {
            use http::header::{HeaderName, HeaderValue};
            opts.insert_header(
                HeaderName::from_static("set-cookie"),
                HeaderValue::from_static("auth_token=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0"),
            );
        }

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server function only available on SSR"))
    }
}

#[server]
pub async fn current_user() -> Result<Option<User>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::extract::Extension;
        use axum_extra::extract::CookieJar;
        use gitcoda::UserService;
        use leptos_axum::extract;
        use std::sync::Arc;

        let Extension(user_service) = extract::<Extension<Arc<UserService>>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract service: {}", e)))?;

        let jar = extract::<CookieJar>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract cookies: {}", e)))?;

        if let Some(cookie) = jar.get("auth_token") {
            let token = cookie.value().to_string();
            user_service
                .get_session_user(token)
                .await
                .map_err(|e| ServerFnError::new(format!("Failed to get user: {}", e)))
        } else {
            Ok(None)
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(None)
    }
}
