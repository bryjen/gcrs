//! core header delcaration
//! has logic for resolving based on current route/location

mod repo_tab_strip;
use repo_tab_strip::*;

mod core;
use core::*;

use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_location;

fn get_breadcrumb(pathname: &str) -> AnyView {
    match pathname {
        "/" => view! {
            <nav class="flex items-center gap-1 text-sm">
                <a href="/" class="font-semibold text-foreground hover:underline">"Dashboard"</a>
            </nav>
        }.into_any(),
        "/repo" => view! {
            <nav class="flex items-center gap-1 text-sm">
                <a href="/" class="font-semibold text-muted-foreground hover:text-foreground">"username"</a>
                <span class="text-muted-foreground/40">"/"</span>
                <a href="/" class="font-semibold text-foreground hover:underline">"repo-name"</a>
            </nav>
        }.into_any(),
        _ => view! {
        }.into_any(),
    }
}

fn get_snd_row(pathname: &str) -> AnyView {
    match pathname {
        "/repo" => view! {
            <div class="flex items-center gap-4 px-6">
                <RepoTabStrip />
            </div>
        }
        .into_any(),
        _ => view! {}.into_any(),
    }
}

#[component]
pub fn Header(
    #[prop(default = String::new())] username: String,
) -> impl IntoView {
    // might need to clean the route, depends on how we structure the page routes
    let location = use_location();
    let binding = location.pathname.get_untracked().clone();
    let pathname = binding.trim();

    view! {
        <header class="z-50 border-b border-border bg-card">
            <div class="flex items-center gap-4 px-6 pt-4 pb-2">
                <div class="flex items-center gap-6">
                    <NavDrawer />
                    <Icon icon=i::ImGit width="20" height="20"/>
                    {get_breadcrumb(&pathname)}
                </div>
                <div class="ml-auto flex items-center gap-5">
                    <GlobalSearch />
                    <GlobalUserActions username=if username.is_empty() { None } else { Some(username.clone()) } />
                </div>
            </div>

            {get_snd_row(&pathname)}
        </header>
    }
}
