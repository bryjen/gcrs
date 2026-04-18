use leptos::prelude::*;
use leptos_icons::Icon;

use icondata as i;

use components::ui::badge::{Badge, BadgeSize};
use components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

#[island]
pub fn LinkPanel() -> impl IntoView {
    let tabs = vec![
        (
            "repos".to_string(),
            i::MdiSourceRepositoryMultiple,
            "Repositories",
        ),
        ("orgs".to_string(), i::OcOrganizationLg, "Organizations"),
    ];
    // two iters required, so we clone
    // small enough, shouldn't matter
    let tabs_clone = tabs.clone();

    view! {
        <Tabs default_value="repos" class="w-full">
            <TabsList variant=TabsVariant::Line>
                {tabs_clone.into_iter().map(|(val, icon, label)| {
                    view! {
                        <TabsTrigger value={val}>
                            <div class="mx-1 flex gap-2">
                                <Icon icon={icon} width="20" height="20"/>
                                {label}
                            </div>
                        </TabsTrigger>
                    }
                }).collect_view()}
            </TabsList>
            {tabs.into_iter().map(|(val, _, _)| {
                let val_clone = val.clone();
                view! {
                    <TabsContent value={val}>
                        {render_tab_content(val_clone)}
                    </TabsContent>
                }
            }).collect_view()}
        </Tabs>
    }
}

fn render_tab_content(tab: String) -> AnyView {
    let repo_info = vec![
        RepoInfo {
            name: "kernel".to_string(),
            is_private: false,
            language_name: "C".to_string(),
            language_color_class: "bg-[#555555]".to_string(),
        },
        RepoInfo {
            name: "openclaw".to_string(),
            is_private: true,
            language_name: "Rust".to_string(),
            language_color_class: "bg-[#dea584]".to_string(),
        },
        RepoInfo {
            name: "dotfiles".to_string(),
            is_private: false,
            language_name: "Shell".to_string(),
            language_color_class: "bg-[#89e051]".to_string(),
        },
        RepoInfo {
            name: "noctua".to_string(),
            is_private: true,
            language_name: "C#".to_string(),
            language_color_class: "bg-[#178600]".to_string(),
        },
        RepoInfo {
            name: "libsignal-rs".to_string(),
            is_private: false,
            language_name: "Rust".to_string(),
            language_color_class: "bg-[#dea584]".to_string(),
        },
        RepoInfo {
            name: "infra".to_string(),
            is_private: true,
            language_name: "Nix".to_string(),
            language_color_class: "bg-[#7e7eff]".to_string(),
        },
    ];

    match tab.as_str() {
        "repos" => render_repos(repo_info).into_any(),
        "orgs" => render_orgs().into_any(),
        _ => view! { <p>"unknown"</p> }.into_any(),
    }
}

pub struct RepoInfo {
    name: String,
    is_private: bool,
    language_name: String,
    language_color_class: String,
}

fn render_repos(repos: Vec<RepoInfo>) -> impl IntoView {
    let var_class = "inline-flex items-center gap-1 font-semibold rounded-md border w-fit border-transparent bg-secondary text-secondary-foreground/50 px-1.5 py-0.5 text-[10px] group-hover:text-secondary-foreground transition-all duration-100";
    view! {
        <div class="mt-2 space-y-3">
            {repos.into_iter().map(|repo| {
                view! {
                    <div class="space-y-2">
                        <a href="/repo" class="group text-base flex items-center justify-between gap-4 text-foreground transition-colors">
                            <div class="flex items-center gap-4">
                                <span class="font-medium group-hover:underline">{repo.name}</span>
                                {repo.is_private.then(|| {
                                    view! {
                                        <span class=var_class>
                                            <Icon icon=i::BiLockAltSolid width="15" height="15" />
                                            "Private"
                                        </span>
                                    }})
                                }
                            </div>

                            <div class="flex items-center gap-6 text-xs text-muted-foreground min-w-12 w-fit justify-between">
                                <div class="flex items-center gap-1">
                                    <span class=format!("w-3 h-3 rounded-full transition-all duration-100 {}", repo.language_color_class)></span>
                                    <span class="group-hover:text-foreground transition-all duration-100">{repo.language_name}</span>
                                </div>
                            </div>
                        </a>
                    </div>
                }}).collect::<Vec<_>>()
            }
        </div>
    }
}

fn render_orgs() -> impl IntoView {
    view! {
       <div class="flex items-center justify-center w-full h-24 text-muted-foreground">
           "Coming Soon"
       </div>
    }
}
