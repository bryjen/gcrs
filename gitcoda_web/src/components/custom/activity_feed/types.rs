#[derive(Clone, Debug)]
pub enum ActivityEvent {
    CommitsCreated {
        total_commits: u32,
        total_repos: u32,
        repos: Vec<CommitRepo>,
    },
    RepositoriesCreated {
        repos: Vec<RepoCreation>,
    },
    PullRequestOpened {
        repo: String,
        branch: String,
        merge_count: u32,
        date: String,
    },
    IssuesOpened {
        repo: String,
        issues: Vec<Issue>,
    },
}

#[derive(Clone, Debug)]
pub struct CommitRepo {
    pub name: String,
    pub owner: String,
    pub commit_count: u32,
    pub bar_width_percent: u32,
    pub opacity: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct RepoCreation {
    pub name: String,
    pub owner: String,
    pub is_private: bool,
    pub language: String,
    pub language_color_class: String,
    pub date: String,
}

#[derive(Clone, Debug)]
pub struct Issue {
    pub title: String,
    pub url: String,
    pub status: IssueStatus,
    pub count: u32,
    pub date: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IssueStatus {
    Open,
    Closed,
}
