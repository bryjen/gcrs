use crate::components::custom::activity_feed::types::RepoCreation;
use crate::components::custom::activity_feed::shared::{RepoIcon, LockIcon, ToggleButton};
use leptos::prelude::*;

#[component]
pub fn RepoEventRow(repos: Vec<RepoCreation>) -> impl IntoView {
    view! {
        <div class="mb-10 pl-8 relative">
            <RepoIcon />

            <div class="flex items-center justify-between text-foreground mb-3">
                <h3 class="text-base font-medium">
                    "Created " {repos.len()} " repositories"
                </h3>
                <ToggleButton />
            </div>

            <div class="space-y-3 mt-4">
                {repos.into_iter().map(|repo| {
                    view! {
                        <RepoCreationRow repo=repo />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
pub fn RepoCreationRow(repo: RepoCreation) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between text-sm">
            <div class="flex items-center gap-2">
                {repo.is_private.then(|| {
                    view! {
                        <div class="w-4 h-4 bg-card rounded-full flex items-center justify-center">
                            <LockIcon />
                        </div>
                    }
                })}
                <a href="#" class="text-primary hover:underline font-medium">
                    {repo.owner} "/" {repo.name}
                </a>
            </div>
            <div class="flex items-center gap-6 text-xs text-muted-foreground w-32 justify-between">
                <div class="flex items-center gap-1">
                    <span
                        class=format!("w-3 h-3 rounded-full {}", repo.language_color_class)
                    ></span>
                    <span>{repo.language}</span>
                </div>
                <span>{repo.date}</span>
            </div>
        </div>
    }
}
