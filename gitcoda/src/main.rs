//  SSR binary entry — compiled only when the `ssr` feature is active.
//  The WASM hydrate entry lives below, compiled only for wasm32.

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use gitcoda::app::{App, shell};
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use tower_http::compression::CompressionLayer;

    let conf = get_configuration(None).unwrap();
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&options, routes, {
            let options = options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(CompressionLayer::new())
        .with_state(options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log::info!("listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}

// Satisfy the compiler when neither feature is active (e.g. `cargo check`).
#[cfg(not(any(feature = "ssr", feature = "hydrate")))]
fn main() {}
