use crate::components::layout::header::Header;
use crate::pages::home::Home;
use crate::pages::repo::Repo;
use leptos::prelude::*;
use leptos_router::components::ParentRoute;
use leptos_router::nested_router::Outlet;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[allow(non_snake_case)]
fn MainLayout() -> impl IntoView {
    view! {
        <main class="relative min-h-screen" data-vaul-drawer-wrapper>
            <Header />  // see header impl, we add different data per route
            <div class="mx-auto min-h-screen max-w-screen-xl px-4 md:px-8 lg:px-12">
                <Outlet/>
            </div>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found">
                <ParentRoute path=path!("/") view=MainLayout>
                    <Route path=path!("") view=Home />
                </ParentRoute>
                <ParentRoute path=path!("/repo") view=MainLayout>
                    <Route path=path!("") view=Repo />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

// Shell is only needed on the server — it wraps App in the full HTML document.
#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    use leptos_meta::{MetaTags, provide_meta_context};

    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html class="dark" lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap"
                    rel="stylesheet"
                />
                <link rel="stylesheet" href="/pkg/gitcoda.css" />
                <link rel="icon" type="image/png" href="/favicon-96x96.png" sizes="96x96" />
                <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
                <link rel="shortcut icon" href="/favicon.ico" />
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
