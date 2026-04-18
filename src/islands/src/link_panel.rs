use leptos::prelude::*;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};

use icondata as i;

use components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};
use gitcoda::get_language_color;
use gitcoda::models::git::Repository;

#[server]
pub async fn fetch_repos() -> Result<Vec<Repository>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::extract::Extension;
        use gitcoda::RepoService;
        use leptos_axum::extract;
        use std::sync::Arc;

        let Extension(svc) = extract::<Extension<Arc<RepoService>>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract RepoService: {}", e)))?;

        svc.list_demo()
            .await
            .map_err(|e| ServerFnError::new(format!("DB error: {}", e)))
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server function only available on SSR"))
    }
}

#[island]
pub fn LinkPanel() -> impl IntoView {
    view! {
        <Tabs default_value="repos" class="w-full">
            <TabsList variant=TabsVariant::Line>
                <TabsTrigger value="repos">
                    <div class="mx-1 flex gap-2">
                        <Icon icon=i::MdiSourceRepositoryMultiple width="20" height="20"/>
                        "Repositories"
                    </div>
                </TabsTrigger>
                <TabsTrigger value="orgs">
                    <div class="mx-1 flex gap-2">
                        <Icon icon=i::OcOrganizationLg width="20" height="20"/>
                        "Organizations"
                    </div>
                </TabsTrigger>
            </TabsList>
            <TabsContent value="repos">
                {render_repos_tab()}
            </TabsContent>
            <TabsContent value="orgs">
                {render_orgs()}
            </TabsContent>
        </Tabs>
    }
}

fn render_repos_tab() -> impl IntoView {
    let repos_resource = Resource::new(|| (), |_| fetch_repos());
    view! {
        <Suspense fallback=|| view! { <p>"Loading repos..."</p> }>
            {move || {
                repos_resource.read().as_ref().map(|result| {
                    match result {
                        Ok(repos) => {
                            let repo_infos = repos.iter().cloned().map(|r| {
                                let color = get_language_color(&r.language);
                                RepoInfo {
                                    name: r.name,
                                    is_private: r.is_private,
                                    language_name: r.language,
                                    language_color: color,
                                }
                            }).collect();
                            render_repos(repo_infos).into_any()
                        }
                        Err(e) => view! { <p>"Error: " {e.to_string()}</p> }.into_any(),
                    }
                })
            }}
        </Suspense>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepoInfo {
    name: String,
    is_private: bool,
    language_name: String,
    language_color: String,
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
                                    <span
                                        class="w-3 h-3 rounded-full transition-all duration-100"
                                        style=format!("background-color: {}", repo.language_color)
                                    ></span>
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
