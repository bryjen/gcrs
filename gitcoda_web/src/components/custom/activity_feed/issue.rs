use crate::components::custom::activity_feed::types::{Issue, IssueStatus};
use crate::components::custom::activity_feed::shared::{IssueIcon, ToggleButton};
use leptos::prelude::*;

#[component]
pub fn IssueEventRow(repo: String, issues: Vec<Issue>) -> impl IntoView {
    view! {
        <div class="mb-4 pl-8 relative">
            <IssueIcon />

            <div class="flex items-center justify-between text-foreground mb-2">
                <h3 class="text-base font-medium">
                    "Opened " {issues.len()} " issues in 1 repository"
                </h3>
                <ToggleButton />
            </div>

            <div class="mb-3">
                <span class="text-muted-foreground text-sm">{repo}</span>
            </div>

            <div class="space-y-3 mt-2">
                {issues.into_iter().map(|issue| {
                    view! {
                        <IssueRow issue=issue />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
pub fn IssueRow(issue: Issue) -> impl IntoView {
    let status_text = match issue.status {
        IssueStatus::Open => "open",
        IssueStatus::Closed => "closed",
    };

    let (status_bg, status_text_color, status_border, badge_bg) = match issue.status {
        IssueStatus::Open => (
            "bg-[#2ea043]/10",
            "text-primary",
            "border-[#2ea043]/20",
            "bg-primary",
        ),
        IssueStatus::Closed => (
            "bg-[#8957e5]/10",
            "text-accent",
            "border-[#8957e5]/20",
            "bg-accent",
        ),
    };

    view! {
        <div class="flex items-center justify-between text-sm">
            <div class="flex items-center gap-2 truncate pr-4">
                <svg
                    class=format!("w-4 h-4 flex-shrink-0")
                    class=("text-primary", issue.status == IssueStatus::Open)
                    class=("text-accent", issue.status == IssueStatus::Closed)
                    viewBox="0 0 16 16"
                    fill="currentColor"
                >
                    <path d="M8 9.5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3Z"></path>
                    <path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM1.5 8a6.5 6.5 0 1 0 13 0 6.5 6.5 0 0 0-13 0Z"></path>
                </svg>
                <a href="#" class="text-foreground hover:text-primary transition-colors font-medium hover:underline truncate">
                    {issue.title.clone()}
                </a>
            </div>
            <div class="flex items-center gap-3 text-xs flex-shrink-0">
                <span class="text-muted-foreground flex items-center gap-1">
                    <span class=format!("{} {} px-2 py-0.5 rounded-full flex items-center gap-1 font-medium border {}", status_bg, status_text_color, status_border)>
                        <span class=format!("{} rounded-full w-4 h-4 flex items-center justify-center text-white text-[10px]", badge_bg)>
                            {issue.count}
                        </span>
                        {status_text}
                    </span>
                </span>
                <span class="text-muted-foreground">{issue.date}</span>
            </div>
        </div>
    }
}
