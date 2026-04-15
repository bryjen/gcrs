use crate::components::layout::header::Header;
use crate::pages::home::Home;
use leptos::prelude::*;
use leptos_router::components::ParentRoute;
use leptos_router::nested_router::Outlet;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

fn MainLayout() -> impl IntoView {
    view! {
        <main class="relative min-h-screen" data-vaul-drawer-wrapper>
            <Header />
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
                    /*
                    <Route path=path!("projects") view=ProjectsPage />
                    */
                </ParentRoute>
            </Routes>
        </Router>
    }
}
