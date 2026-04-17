use crate::components::custom::activity_feed::{
    ActivityEvent, ActivityTimeline, CommitRepo, Issue, IssueStatus, RepoCreation,
};
use crate::components::custom::heatmap::{ActivityHeatmap, HeatmapData};
use leptos::prelude::*;

mod link_panel;
use link_panel::LinkPanel;

#[component]
pub fn Home() -> impl IntoView {
    let heatmap_data = generate_heatmap_data();
    let activity_events = generate_activity_events();

    view! {
        <div class="flex flex-col gap-12 py-12 pt-8 pb-[50vh]">

            <div class="flex gap-8">
                <div class="flex flex-col gap-6 w-min-0 w-fit shirnk-0">
                    <ActivityHeatmap data=heatmap_data />

                    <ActivityTimeline
                        month="March".to_string()
                        year="2026".to_string()
                        events=activity_events />
                </div>

                <div class="flex-1 w-full">
                    <LinkPanel />
                </div>
            </div>

        </div>
    }
}

fn generate_heatmap_data() -> Vec<HeatmapData> {
    let mut data = Vec::new();

    for day in 0..365 {
        let month = (day / 31) + 1;
        let day_of_month = (day % 31) + 1;
        let date = format!("2026-{:02}-{:02}", month, day_of_month);

        // Simple pseudorandom: spread activity across days
        let seed = (day * 73 + 17) as u32;
        let count = ((seed ^ (seed >> 16)) % 300) + (day as u32 % 50);

        if count > 20 {
            data.push(HeatmapData { date, count });
        }
    }

    data
}

fn generate_activity_events() -> Vec<ActivityEvent> {
    vec![
        ActivityEvent::CommitsCreated {
            total_commits: 129,
            total_repos: 5,
            repos: vec![
                CommitRepo {
                    name: "noctua".to_string(),
                    owner: "bryjen".to_string(),
                    commit_count: 59,
                    bar_width_percent: 100,
                    opacity: None,
                },
                CommitRepo {
                    name: "ShadcnBlazor".to_string(),
                    owner: "bryjen".to_string(),
                    commit_count: 49,
                    bar_width_percent: 80,
                    opacity: None,
                },
                CommitRepo {
                    name: "bryjen.github.io".to_string(),
                    owner: "bryjen".to_string(),
                    commit_count: 10,
                    bar_width_percent: 15,
                    opacity: Some(0.8),
                },
                CommitRepo {
                    name: "remote-infra".to_string(),
                    owner: "bryjen".to_string(),
                    commit_count: 6,
                    bar_width_percent: 10,
                    opacity: Some(0.8),
                },
                CommitRepo {
                    name: "nixos-dotfiles".to_string(),
                    owner: "bryjen".to_string(),
                    commit_count: 5,
                    bar_width_percent: 8,
                    opacity: Some(0.8),
                },
            ],
        },
        ActivityEvent::RepositoriesCreated {
            repos: vec![
                RepoCreation {
                    name: "remote-infra".to_string(),
                    owner: "bryjen".to_string(),
                    is_private: true,
                    language: "Shell".to_string(),
                    language_color_class: "bg-primary".to_string(),
                    date: "Mar 16".to_string(),
                },
                RepoCreation {
                    name: "noctua".to_string(),
                    owner: "bryjen".to_string(),
                    is_private: false,
                    language: "C#".to_string(),
                    language_color_class: "bg-accent".to_string(),
                    date: "Mar 11".to_string(),
                },
            ],
        },
        ActivityEvent::PullRequestOpened {
            repo: "bryjen/ShadcnBlazor".to_string(),
            branch: "Dev".to_string(),
            merge_count: 1,
            date: "Mar 25".to_string(),
        },
        ActivityEvent::IssuesOpened {
            repo: "bryjen/ShadcnBlazor".to_string(),
            issues: vec![
                Issue {
                    title: "v0.2.1 roadmap".to_string(),
                    url: "#".to_string(),
                    status: IssueStatus::Open,
                    count: 2,
                    date: "Mar 30".to_string(),
                },
                Issue {
                    title: "experiment: determine viability via RenderTreeBuilder introspection"
                        .to_string(),
                    url: "#".to_string(),
                    status: IssueStatus::Open,
                    count: 1,
                    date: "Mar 27".to_string(),
                },
            ],
        },
    ]
}
