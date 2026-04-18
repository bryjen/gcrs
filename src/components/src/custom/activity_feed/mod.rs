pub mod commit;
pub mod issue;
pub mod pr;
pub mod repo;
pub mod shared;
pub mod types;

pub use commit::CommitEventRow;
pub use issue::IssueEventRow;
pub use pr::PREventRow;
pub use repo::RepoEventRow;
pub use types::{ActivityEvent, CommitRepo, Issue, IssueStatus, RepoCreation};

use leptos::prelude::*;

#[component]
pub fn ActivityTimeline(
    #[prop(default = "March".to_string())] month: String,
    #[prop(default = "2026".to_string())] year: String,
    events: Vec<ActivityEvent>,
) -> impl IntoView {
    view! {
        <div>
            <ActivityHeader month=month year=year />
            <ActivityFeed events=events />
        </div>
    }
}

#[component]
fn ActivityHeader(month: String, year: String) -> impl IntoView {
    view! {
        <div class="flex items-center text-sm font-semibold mb-6">
            <span class="text-foreground">
                {month}
                <span class="text-muted-foreground font-normal">{year}</span>
            </span>
            <div class="h-px bg-border flex-1 ml-4"></div>
        </div>
    }
}

#[component]
fn ActivityFeed(events: Vec<ActivityEvent>) -> impl IntoView {
    view! {
        <div class="relative border-l border-border ml-[15px] pb-4">
            {events
                .into_iter()
                .enumerate()
                .map(|(_idx, event)| {
                    let (is_commit, commits_data) = match &event {
                        ActivityEvent::CommitsCreated {
                            total_commits,
                            total_repos,
                            repos,
                        } => (true, Some((total_commits.clone(), total_repos.clone(), repos.clone()))),
                        _ => (false, None),
                    };

                    let (is_repo, repo_data) = match &event {
                        ActivityEvent::RepositoriesCreated { repos } => (true, Some(repos.clone())),
                        _ => (false, None),
                    };

                    let (is_pr, pr_data) = match &event {
                        ActivityEvent::PullRequestOpened {
                            repo,
                            branch,
                            merge_count,
                            date,
                        } => (true, Some((repo.clone(), branch.clone(), merge_count.clone(), date.clone()))),
                        _ => (false, None),
                    };

                    let (is_issue, issue_data) = match &event {
                        ActivityEvent::IssuesOpened { repo, issues } => (
                            true,
                            Some((repo.clone(), issues.clone())),
                        ),
                        _ => (false, None),
                    };

                    view! {
                        <div>
                            {is_commit.then(|| {
                                let (tc, tr, r) = commits_data.unwrap();
                                view! {
                                    <CommitEventRow
                                        total_commits=tc
                                        total_repos=tr
                                        repos=r
                                    />
                                }
                            })}
                            {is_repo.then(|| {
                                let r = repo_data.unwrap();
                                view! {
                                    <RepoEventRow repos=r />
                                }
                            })}
                            {is_pr.then(|| {
                                let (repo, branch, mc, date) = pr_data.unwrap();
                                view! {
                                    <PREventRow
                                        repo=repo
                                        branch=branch
                                        merge_count=mc
                                        date=date
                                    />
                                }
                            })}
                            {is_issue.then(|| {
                                let (repo, issues) = issue_data.unwrap();
                                view! {
                                    <IssueEventRow repo=repo issues=issues />
                                }
                            })}
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
