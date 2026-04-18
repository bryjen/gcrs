use super::*;
use crate::models::ids::{CommitStatusId, RepoId};

#[test]
fn commit_status_combine_empty_is_pending() {
    assert_eq!(CommitStatusState::combine([]), CommitStatusState::Pending);
}

#[test]
fn commit_status_combine_all_success_is_success() {
    let states = [CommitStatusState::Success, CommitStatusState::Success];
    assert_eq!(CommitStatusState::combine(states), CommitStatusState::Success);
}

#[test]
fn commit_status_combine_success_and_skipped_is_success() {
    let states = [CommitStatusState::Success, CommitStatusState::Skipped];
    assert_eq!(CommitStatusState::combine(states), CommitStatusState::Success);
}

#[test]
fn commit_status_combine_pending_makes_pending() {
    let states = [CommitStatusState::Pending, CommitStatusState::Success];
    assert_eq!(CommitStatusState::combine(states), CommitStatusState::Pending);
}

#[test]
fn commit_status_combine_warning_is_failure() {
    let states = [CommitStatusState::Success, CommitStatusState::Warning];
    assert_eq!(CommitStatusState::combine(states), CommitStatusState::Failure);
}

#[test]
fn commit_status_combine_error_is_failure() {
    let states = [CommitStatusState::Error];
    assert_eq!(CommitStatusState::combine(states), CommitStatusState::Failure);
}

#[test]
fn calc_commit_status_empty_is_none() {
    assert_eq!(calc_commit_status(&[]), None);
}

#[test]
fn calc_commit_status_combines_states_and_selects_a_target_url() {
    let repo_id = RepoId(1);
    let sha = GitOid("abc".to_string());

    let statuses = vec![
        CommitStatus {
            id: CommitStatusId(1),
            index: 1,
            repo_id,
            state: CommitStatusState::Pending,
            sha: sha.clone(),
            target_url: None,
            description: None,
            context: "ci/lint".to_string(),
            context_hash: None,
            creator_id: None,
            created_at: None,
            updated_at: None,
        },
        CommitStatus {
            id: CommitStatusId(2),
            index: 2,
            repo_id,
            state: CommitStatusState::Success,
            sha: sha.clone(),
            target_url: Some("https://example.test/build/2".to_string()),
            description: None,
            context: "ci/test".to_string(),
            context_hash: None,
            creator_id: None,
            created_at: None,
            updated_at: None,
        },
    ];

    let summary = calc_commit_status(&statuses).unwrap();
    assert_eq!(summary.repo_id, repo_id);
    assert_eq!(summary.sha, sha);
    assert_eq!(summary.state, CommitStatusState::Pending);
    assert_eq!(
        summary.target_url.as_deref(),
        Some("https://example.test/build/2")
    );
}

#[test]
fn serde_id_newtypes_serialize_as_numbers() {
    let id = RepoId(42);
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "42");
}

#[cfg(feature = "pattern")]
#[test]
fn protected_branch_matches_plain_name_case_insensitive() {
    use crate::models::ids::ProtectedBranchId;

    let pb = ProtectedBranch {
        id: ProtectedBranchId(1),
        repo_id: RepoId(1),
        rule_name: "Main".to_string(),
        priority: 0,
        can_push: false,
        enable_whitelist: false,
        whitelist_user_ids: vec![],
        whitelist_team_ids: vec![],
        enable_merge_whitelist: false,
        whitelist_deploy_keys: false,
        merge_whitelist_user_ids: vec![],
        merge_whitelist_team_ids: vec![],
        can_force_push: false,
        enable_force_push_allowlist: false,
        force_push_allowlist_user_ids: vec![],
        force_push_allowlist_team_ids: vec![],
        force_push_allowlist_deploy_keys: false,
        enable_status_check: false,
        status_check_contexts: vec![],
        enable_approvals_whitelist: false,
        approvals_whitelist_user_ids: vec![],
        approvals_whitelist_team_ids: vec![],
        required_approvals: 0,
        block_on_rejected_reviews: false,
        block_on_official_review_requests: false,
        block_on_outdated_branch: false,
        dismiss_stale_approvals: false,
        ignore_stale_approvals: false,
        require_signed_commits: false,
        protected_file_patterns: None,
        unprotected_file_patterns: None,
        block_admin_merge_override: false,
        created_at: None,
        updated_at: None,
    };

    assert!(pb.matches("main"));
    assert!(pb.matches("MAIN"));
    assert!(!pb.matches("develop"));
}

#[cfg(feature = "pattern")]
#[test]
fn protected_branch_matches_glob() {
    use crate::models::ids::ProtectedBranchId;

    let pb = ProtectedBranch {
        id: ProtectedBranchId(1),
        repo_id: RepoId(1),
        rule_name: "release/*".to_string(),
        priority: 0,
        can_push: false,
        enable_whitelist: false,
        whitelist_user_ids: vec![],
        whitelist_team_ids: vec![],
        enable_merge_whitelist: false,
        whitelist_deploy_keys: false,
        merge_whitelist_user_ids: vec![],
        merge_whitelist_team_ids: vec![],
        can_force_push: false,
        enable_force_push_allowlist: false,
        force_push_allowlist_user_ids: vec![],
        force_push_allowlist_team_ids: vec![],
        force_push_allowlist_deploy_keys: false,
        enable_status_check: false,
        status_check_contexts: vec![],
        enable_approvals_whitelist: false,
        approvals_whitelist_user_ids: vec![],
        approvals_whitelist_team_ids: vec![],
        required_approvals: 0,
        block_on_rejected_reviews: false,
        block_on_official_review_requests: false,
        block_on_outdated_branch: false,
        dismiss_stale_approvals: false,
        ignore_stale_approvals: false,
        require_signed_commits: false,
        protected_file_patterns: None,
        unprotected_file_patterns: None,
        block_admin_merge_override: false,
        created_at: None,
        updated_at: None,
    };

    assert!(pb.matches("release/v1.2"));
    assert!(!pb.matches("feature/x"));
}

#[cfg(feature = "pattern")]
#[test]
fn protected_tag_matches_glob() {
    use crate::models::ids::ProtectedTagId;

    let pt = ProtectedTag {
        id: ProtectedTagId(1),
        repo_id: RepoId(1),
        name_pattern: "v*".to_string(),
        allowlist_user_ids: vec![],
        allowlist_team_ids: vec![],
        created_at: None,
        updated_at: None,
    };

    assert!(pt.matches("v1.2.3"));
    assert!(!pt.matches("release-1"));
}
