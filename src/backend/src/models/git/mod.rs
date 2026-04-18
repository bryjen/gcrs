use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::ids::{
    BranchId, CommitStatusId, ProtectedBranchId, ProtectedTagId, RepoId, UserId,
};

#[cfg(test)]
mod tests;

// Repository
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Repository {
    pub id: RepoId,
    pub owner_id: UserId,
    pub owner_name: String,
    pub name: String,
    pub lower_name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub is_fork: bool,
    pub fork_id: Option<RepoId>,
    pub default_branch: String,
    pub language: String,
    pub language_color_hex: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GitOid(pub String);

impl From<String> for GitOid {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for GitOid {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

// -----------------------------------------------------------------------------
// Branches
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Branch {
    pub id: BranchId,
    pub repo_id: RepoId,
    pub name: String,

    pub commit_id: GitOid,
    pub commit_message: Option<String>,

    pub pusher_id: Option<UserId>,

    pub is_deleted: bool,
    pub deleted_by_id: Option<UserId>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub commit_time: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// -----------------------------------------------------------------------------
// Branch / tag protection
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtectedBranch {
    pub id: ProtectedBranchId,
    pub repo_id: RepoId,

    // Gitea calls this "RuleName" and stores it in the `branch_name` column.
    // It can be a plain branch name or a glob pattern.
    pub rule_name: String,
    pub priority: i64,

    pub can_push: bool,

    pub enable_whitelist: bool,
    pub whitelist_user_ids: Vec<UserId>,
    pub whitelist_team_ids: Vec<i64>,

    pub enable_merge_whitelist: bool,
    pub whitelist_deploy_keys: bool,
    pub merge_whitelist_user_ids: Vec<UserId>,
    pub merge_whitelist_team_ids: Vec<i64>,

    pub can_force_push: bool,
    pub enable_force_push_allowlist: bool,
    pub force_push_allowlist_user_ids: Vec<UserId>,
    pub force_push_allowlist_team_ids: Vec<i64>,
    pub force_push_allowlist_deploy_keys: bool,

    pub enable_status_check: bool,
    pub status_check_contexts: Vec<String>,

    pub enable_approvals_whitelist: bool,
    pub approvals_whitelist_user_ids: Vec<UserId>,
    pub approvals_whitelist_team_ids: Vec<i64>,

    pub required_approvals: i64,
    pub block_on_rejected_reviews: bool,
    pub block_on_official_review_requests: bool,
    pub block_on_outdated_branch: bool,
    pub dismiss_stale_approvals: bool,
    pub ignore_stale_approvals: bool,
    pub require_signed_commits: bool,

    pub protected_file_patterns: Option<String>,
    pub unprotected_file_patterns: Option<String>,

    pub block_admin_merge_override: bool,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtectedTag {
    pub id: ProtectedTagId,
    pub repo_id: RepoId,

    pub name_pattern: String,
    pub allowlist_user_ids: Vec<UserId>,
    pub allowlist_team_ids: Vec<i64>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// -----------------------------------------------------------------------------
// Commit status
// -----------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CommitStatusState {
    Pending,
    Success,
    Error,
    Failure,
    Warning,
    Skipped,
}

impl CommitStatusState {
    /// Combine states using Gitea semantics.
    ///
    /// Mirrors `context/gitea/modules/commitstatus/commit_status.go`:
    /// - `failure` if any context is `error|failure|warning` (warning is failure in Gitea)
    /// - `success` if all contexts are `success|skipped` and there is at least one context
    /// - otherwise `pending` (including an empty list)
    pub fn combine<I>(states: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let mut total = 0usize;
        let mut success_cnt = 0usize;

        for state in states {
            total += 1;
            match state {
                Self::Error | Self::Failure | Self::Warning => return Self::Failure,
                Self::Pending => {}
                Self::Success | Self::Skipped => success_cnt += 1,
            }
        }

        if total > 0 && success_cnt == total {
            Self::Success
        } else {
            Self::Pending
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitStatus {
    pub id: CommitStatusId,
    pub index: i64,
    pub repo_id: RepoId,

    pub state: CommitStatusState,
    pub sha: GitOid,

    pub target_url: Option<String>,
    pub description: Option<String>,

    pub context: String,
    pub context_hash: Option<String>,

    pub creator_id: Option<UserId>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitStatusSummary {
    pub repo_id: RepoId,
    pub sha: GitOid,
    pub state: CommitStatusState,
    pub target_url: Option<String>,
}

pub fn calc_commit_status(statuses: &[CommitStatus]) -> Option<CommitStatusSummary> {
    if statuses.is_empty() {
        return None;
    }

    let mut target_url: Option<String> = None;
    let state = CommitStatusState::combine(statuses.iter().map(|s| {
        if let Some(url) = &s.target_url {
            if !url.is_empty() {
                target_url = Some(url.clone());
            }
        }
        s.state
    }));

    Some(CommitStatusSummary {
        repo_id: statuses[0].repo_id,
        sha: statuses[0].sha.clone(),
        state,
        target_url,
    })
}

// -----------------------------------------------------------------------------
// Optional pattern matching helpers
// -----------------------------------------------------------------------------

#[cfg(feature = "pattern")]
fn has_glob_specials(pat: &str) -> bool {
    // Rough equivalent of Gitea's `glob.IsSpecialByte` check.
    // Good enough for typical branch/tag patterns (`release/*`, `v*`, etc).
    pat.bytes()
        .any(|b| matches!(b, b'*' | b'?' | b'[' | b']' | b'{' | b'}'))
}

#[cfg(feature = "pattern")]
fn compile_glob(pat: &str) -> Result<globset::GlobMatcher, globset::Error> {
    use globset::GlobBuilder;
    Ok(GlobBuilder::new(pat)
        .literal_separator(true)
        .backslash_escape(true)
        .build()?
        .compile_matcher())
}

#[cfg(feature = "pattern")]
impl ProtectedBranch {
    /// Returns true if `branch_name` matches this protection rule.
    ///
    /// If `rule_name` has no glob specials, this is a case-insensitive exact match.
    pub fn matches(&self, branch_name: &str) -> bool {
        if !has_glob_specials(&self.rule_name) {
            return self.rule_name.eq_ignore_ascii_case(branch_name);
        }

        match compile_glob(&self.rule_name) {
            Ok(m) => m.is_match(branch_name),
            Err(_) => false,
        }
    }
}

#[cfg(feature = "pattern")]
impl ProtectedTag {
    /// Returns true if `tag_name` matches `name_pattern` (glob semantics).
    pub fn matches(&self, tag_name: &str) -> bool {
        match compile_glob(&self.name_pattern) {
            Ok(m) => m.is_match(tag_name),
            Err(_) => false,
        }
    }
}
