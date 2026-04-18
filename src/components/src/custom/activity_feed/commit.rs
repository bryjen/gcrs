use crate::custom::activity_feed::types::CommitRepo;
use crate::custom::activity_feed::shared::{CommitIcon, ToggleButton};
use leptos::prelude::*;

#[component]
pub fn CommitEventRow(
    total_commits: u32,
    total_repos: u32,
    repos: Vec<CommitRepo>,
) -> impl IntoView {
    view! {
        <div class="mb-10 pl-8 relative">
            <CommitIcon />

            <div class="flex items-center justify-between text-foreground mb-3">
                <h3 class="text-base font-medium">
                    "Created " {total_commits} " commits in " {total_repos} " repositories"
                </h3>
                <ToggleButton />
            </div>

            <div class="space-y-2 mt-2">
                {repos.into_iter().map(|repo| {
                    view! {
                        <CommitRepoRow repo=repo />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
pub fn CommitRepoRow(repo: CommitRepo) -> impl IntoView {
    let bar_width = format!("{}%", repo.bar_width_percent);
    let opacity_class = match repo.opacity {
        Some(0.8) => "opacity-80",
        _ => "",
    };

    view! {
        <div class="flex items-center justify-between group">
            <div class="flex items-center gap-3">
                <a href="#" class="text-primary hover:underline text-sm font-medium">
                    {repo.owner} "/" {repo.name}
                </a>
                <span class="text-xs text-muted-foreground">{repo.commit_count} " commits"</span>
            </div>
            <div class="w-32 flex justify-start">
                <div
                    class=format!("h-2 bg-primary rounded-full {}", opacity_class)
                    style:width=bar_width
                ></div>
            </div>
        </div>
    }
}
