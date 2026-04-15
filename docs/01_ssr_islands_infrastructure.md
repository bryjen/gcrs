# SSR + Islands Infrastructure

This document outlines the infrastructure present in gitcoda that enables Server-Side Rendering (SSR) with Islands architecture using Leptos and cargo-leptos.

## Overview

gitcoda uses **SSR + Islands** instead of traditional Client-Side Rendering (CSR). This means:

- The server (Axum) renders the full page to HTML on each request
- The browser receives pre-rendered HTML instantly
- Only interactive "island" components hydrate in the browser as WASM
- Static content remains as HTML — no client-side code needed

## Key Infrastructure Components

### 1. **Cargo.toml Workspace Configuration**

The workspace metadata in `Cargo.toml` configures cargo-leptos:

```toml
[[workspace.metadata.leptos]]
name               = "gitcoda"
output-name        = "gitcoda"
bin-package        = "gitcoda"
lib-package        = "gitcoda"
site-root          = "target/site"
site-pkg-dir       = "pkg"
tailwind-input-file = "static/css/tailwind.css"
assets-dir         = "static"
site-addr          = "127.0.0.1:3000"
reload-port        = 3001
env                = "DEV"
bin-features       = ["ssr"]
bin-default-features = false
lib-features       = ["hydrate"]
lib-default-features = false
```

**What this does:**
- `bin-features = ["ssr"]` — builds the server binary with SSR support
- `lib-features = ["hydrate"]` — builds the WASM library with hydration support
- `tailwind-input-file` — tells cargo-leptos where to find and compile Tailwind CSS
- `site-root` and `site-pkg-dir` — output staging directory for compiled assets

### 2. **Dual Feature Flags**

The codebase compiles twice with different feature sets:

```toml
[features]
default = []
hydrate = ["leptos/hydrate"]
ssr = ["leptos/ssr", "leptos_router/ssr", "dep:leptos_axum", "dep:axum", "dep:tokio", ...]
```

**Why:**
- Server needs `tokio`, `axum`, `leptos_axum` — unavailable/unused on the client
- Client WASM needs hydration runtime — unavailable/unused on the server
- Features prevent unnecessary dependencies from bloating each compilation target

### 3. **Conditional Compilation Gates**

Code is gated based on the active feature:

```rust
// Server-only
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() { ... }

// Client-only
#[cfg(feature = "hydrate")]
pub fn hydrate() { ... }

// Client-only (alternative syntax)
#[cfg(not(feature = "ssr"))]
{
    Effect::new(|| { ... });
}
```

This ensures:
- Server code never appears in WASM
- Client code never appears in the binary
- Effects (browser initialization) only run on the client

### 4. **Two Compilation Targets**

cargo-leptos orchestrates **two parallel compilations**:

```bash
cargo leptos watch
```

Internally runs:
1. **Native binary** — `cargo build --features ssr`
   - Outputs: `target/debug/gitcoda` (Axum server)
   - Includes: Server routing, rendering, business logic

2. **WASM library** — `cargo build --target wasm32-unknown-unknown --features hydrate`
   - Outputs: `target/site/pkg/gitcoda_bg.wasm` (islands only)
   - Includes: Only `#[island]` components and their logic

### 5. **Asset Pipeline**

cargo-leptos manages the asset build:

```
static/                          (source assets)
├── css/tailwind.css             (Tailwind input)
├── favicon.svg
└── ...

target/site/                      (staged for Axum to serve)
├── pkg/
│   ├── gitcoda.css              ← compiled from tailwind.css
│   ├── gitcoda.js               ← hydration entry point
│   ├── gitcoda_bg.wasm          ← islands WASM
│   └── gitcoda_bg.wasm.d.ts
├── favicon.svg
└── ...
```

**Tailwind compilation:**
- Input: `static/css/tailwind.css` (with `@import "tailwindcss"`)
- Tool: `tailwindcss` CLI (v4) invoked by cargo-leptos
- Output: `target/site/pkg/gitcoda.css` (fully compiled CSS)

### 6. **Server Entry Point (gitcoda/src/main.rs)**

The SSR server bootstrap:

```rust
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use gitcoda::app::{shell, App};
    
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
    axum::serve(listener, app).await.unwrap();
}
```

**What it does:**
- Starts Axum on `127.0.0.1:3000`
- Registers Leptos routes with `leptos_routes()`
- The `shell()` function (from app.rs) wraps the page in full HTML document
- Serves static assets (including compiled CSS) via `CompressionLayer`

### 7. **Shell Function (gitcoda/src/app.rs)**

The SSR HTML wrapper:

```rust
#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html class="dark" lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="stylesheet" href="/pkg/gitcoda.css" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options islands=true />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
```

**What it does:**
- Wraps the App component in `<!DOCTYPE html>`
- Loads compiled CSS from `/pkg/gitcoda.css`
- Loads hydration scripts via `HydrationScripts` — these mount islands in the browser
- `islands=true` tells Leptos to only ship WASM for island components

### 8. **Hydration Entry Point (gitcoda/src/lib.rs)**

The WASM entrypoint:

```rust
#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
```

**What it does:**
- Exports a `hydrate()` function for WASM
- Calls `hydrate_islands()` — finds all island components in the DOM and attaches interactivity

### 9. **Island Components**

Components marked with `#[island]` are compiled to WASM:

```rust
#[island]
pub fn RepoLanguageBar(languages: Vec<LanguageBar>) -> impl IntoView {
    let open = RwSignal::new(false);
    
    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(|| {
            // Browser-only initialization
        });
    }
    
    view! { /* DOM structure identical on SSR and client */ }
}
```

**What it does:**
- Server-side: renders to HTML string, inserts markers for hydration
- Client-side: hydrates the markers, restores state, attaches event listeners
- Effects are no-ops on the server, but run on the client

### 10. **rust-analyzer Configuration**

`rust-analyzer.toml` ensures LSP doesn't complain about cfg gates:

```toml
[cargo]
features = ["ssr", "hydrate"]
```

Tells rust-analyzer to analyze code assuming both features are active, preventing false positives.

## Build Flow

```
cargo leptos watch
    ├─ Compile server binary (--features ssr)
    │  └─ gitcoda/src/main.rs → target/debug/gitcoda
    │
    ├─ Compile WASM (--target wasm32-unknown-unknown --features hydrate)
    │  └─ gitcoda/src/lib.rs → target/site/pkg/gitcoda_bg.wasm
    │
    ├─ Compile Tailwind
    │  └─ static/css/tailwind.css → target/site/pkg/gitcoda.css
    │
    └─ Copy assets
       └─ static/* → target/site/*
```

## Runtime Flow

```
1. Browser requests http://127.0.0.1:3000/
2. Axum server receives request
3. Server renders App component to HTML string (SSR)
4. shell() wraps it in full document with:
   - <link rel="stylesheet" href="/pkg/gitcoda.css">
   - <script> tags that load hydration code
5. Browser receives pre-rendered HTML
6. CSS loads, page paints
7. Hydration script loads WASM
8. hydrate() runs, finds island markers
9. Islands hydrate — state/listeners restored
10. Page becomes interactive
```

## Summary

The infrastructure consists of:

- **Dual compilation** (SSR binary + WASM) via cargo-leptos
- **Feature flags** to conditionally enable/disable code paths
- **Asset pipeline** that compiles Tailwind and stages output
- **Axum server** that renders the page and serves assets
- **Hydration entry point** that attaches interactivity on the client
- **Island components** that render identically on both server and client
- **LSP configuration** to prevent false positives in the editor

This architecture gives you the performance of SSR (fast initial HTML) + the interactivity of WASM (minimal WASM size, only for islands).
