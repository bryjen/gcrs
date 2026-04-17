use leptos::prelude::*;

#[derive(Clone)]
struct RepoTab {
    label: &'static str,
    count: Option<u32>,
}

#[component]
pub fn RepoTabStrip() -> impl IntoView {
    let tabs = vec![
        RepoTab {
            label: "Code",
            count: None,
        },
        RepoTab {
            label: "Issues",
            count: Some(3),
        },
        RepoTab {
            label: "Pull requests",
            count: Some(2),
        },
        RepoTab {
            label: "Actions",
            count: None,
        },
        RepoTab {
            label: "Projects",
            count: None,
        },
        RepoTab {
            label: "Wiki",
            count: None,
        },
        RepoTab {
            label: "Security",
            count: None,
        },
        RepoTab {
            label: "Insights",
            count: None,
        },
        RepoTab {
            label: "Settings",
            count: None,
        },
    ];

    view! {
        <nav class="flex items-end gap-0 overflow-x-auto">
            {tabs.into_iter().map(|tab| view! {
                <a
                    href="/"
                    class="flex items-center gap-1.5 border-b-2 border-transparent px-3 py-2 font-bold text-sm text-muted-foreground hover:border-border hover:text-foreground whitespace-nowrap"
                >
                    {tab.label}
                    {tab.count.map(|n| view! {
                        <span class="rounded-full bg-card px-1.5 py-0.5 text-[11px] leading-none text-primary">
                            {n}
                        </span>
                    })}
                </a>
            }).collect_view()}
        </nav>
    }
}
