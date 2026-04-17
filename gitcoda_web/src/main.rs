//  SSR binary entry — compiled only when the `ssr` feature is active.
//  The WASM hydrate entry lives below, compiled only for wasm32.

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use gitcoda_web::app::{App, shell};
    use gitcoda::{DbPool, RepoService, GitService};
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use tower_http::compression::CompressionLayer;
    use std::sync::Arc;

    use axum::extract::Extension;

    let conf = get_configuration(None).unwrap();
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);

    // Initialize database
    let db = Arc::new(
        DbPool::from_url("sqlite:gitcoda.db?mode=rwc")
            .await
            .expect("Failed to connect to database")
    );

    // Initialize services
    let repo_service = Arc::new(RepoService {
        db: db.clone(),
    });

    let git_service = Arc::new(GitService {
        repo_svc: repo_service.clone(),
    });

    let app = Router::new()
        .leptos_routes(&options, routes, {
            let options = options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(CompressionLayer::new())
        .layer(Extension(repo_service))
        .layer(Extension(git_service))
        .with_state(options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log::info!("listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}

// Satisfy the compiler when neither feature is active (e.g. `cargo check`).
#[cfg(not(any(feature = "ssr", feature = "hydrate")))]
fn main() {}
