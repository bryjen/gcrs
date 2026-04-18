use leptos::prelude::*;
use leptos_icons::Icon;
use icondata as i;

#[component]
pub fn CommitIcon() -> impl IntoView {
    view! {
        <div class="absolute w-8 h-8 bg-card rounded-full flex items-center justify-center -left-4 top-0 text-muted-foreground">
            <Icon icon=i::LuGitCommitHorizontal width="16" height="16" />
        </div>
    }
}

#[component]
pub fn RepoIcon() -> impl IntoView {
    view! {
        <div class="absolute w-8 h-8 bg-card rounded-full flex items-center justify-center -left-4 top-0 text-muted-foreground">
            <Icon icon=i::MdiSourceRepository width="16" height="16" />
        </div>
    }
}

#[component]
pub fn PRIcon() -> impl IntoView {
    view! {
        <div class="absolute w-8 h-8 bg-card rounded-full flex items-center justify-center -left-4 top-0 text-muted-foreground">
            <Icon icon=i::FiGitMerge width="16" height="16" />
        </div>
    }
}

#[component]
pub fn IssueIcon() -> impl IntoView {
    view! {
        <div class="absolute w-8 h-8 bg-card rounded-full flex items-center justify-center -left-4 top-0 text-muted-foreground">
            <Icon icon=i::VsIssues width="16" height="16" />
        </div>
    }
}

#[component]
pub fn LockIcon() -> impl IntoView {
    view! {
        <Icon icon=i::BiLockAltSolid width="16" height="16" />
    }
}

#[component]
pub fn ToggleButton() -> impl IntoView {
    view! {
        <button class="text-muted-foreground hover:text-foreground transition-colors">
            <svg viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
                <path d="M4.22 6.22a.75.75 0 0 1 1.06 0L8 8.94l2.72-2.72a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042l-3.25 3.25a.75.75 0 0 1-1.06 0L4.22 7.28a.75.75 0 0 1 0-1.06Z"></path>
            </svg>
        </button>
    }
}
