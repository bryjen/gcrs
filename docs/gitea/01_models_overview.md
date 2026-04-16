# Gitea Models Overview (High Level)

This is a high-level map of the model areas under:

- `context/gitea/models`

It is meant as a quick reference when porting or re-modeling server-side logic into `gitcoda` (domain/models/services) while keeping the Leptos UI in `gitcoda_web`.

## Model Areas (Broad)

Gitea's models are organized mostly by domain subpackages. The main "anchor" entities you tend to care about first:

- `models/user`: `User` (accounts; also used as the backing row for orgs)
- `models/repo`: `Repository` and repository-adjacent tables (units/features, stars/watches, collaborators, releases, wiki, mirrors, uploads/attachments, etc.)
- `models/issues`: `Issue` and `PullRequest` (PR is an issue with extra PR-specific state), plus comments/labels/milestones/reviews/reactions/time tracking
- `models/git`: git-adjacent database state like branches, protected branches/tags, commit statuses (also contains LFS, but you can ignore that if not needed)

Other high-level buckets you'll run into as you expand server-side logic:

- `models/organization`: orgs + teams (org is essentially a `User` with `Type=org`)
- `models/auth` + `models/asymkey`: access tokens, OAuth apps, sessions, 2FA, WebAuthn, SSH keys, deploy keys, GPG keys
- `models/webhook`: webhooks + delivery tasks
- `models/actions`: "Actions" runs/jobs/runners/schedules/artifacts/variables
- `models/packages`: package registry (container/debian/rpm/nuget/etc.)
- `models/activities`: activity actions + notifications + user heatmap/statistics
- plumbing/infrastructure: `models/db`, `models/migrations`, `models/perm`, `models/system`, `models/secret`, `models/unittest`, `models/fixtures`

## Repository-Centric View (Repo + Git-Facing + Issues/PRs)

The repository is the hub. The most common linkage patterns:

- `Repository.OwnerID` points to `User.ID` (repo owner can be a user or an org)
- `Issue.RepoID` points to `Repository.ID`
- `Issue.PosterID` points to `User.ID` (author)
- A pull request is an `Issue` with `Issue.IsPull=true`, with additional PR-only state in `PullRequest`
- Git-facing policy/state is repo-scoped: branches, protected branches/tags, commit statuses typically carry `RepoID`
- Many "user signals" and "access" things are join tables keyed by `(RepoID, UserID)` (stars, watches, collaboration, etc.)

## Diagram (Mermaid)

Paste the following into the Mermaid Live Editor to render:

- https://mermaid.live

```mermaid
classDiagram
direction LR

class User {
  +int64 ID
  +string Name
  +string LowerName
  +UserType Type  // user|org|bot|remote
}

class Repository {
  +int64 ID
  +int64 OwnerID
  +string OwnerName
  +string Name
  +string LowerName
  +string DefaultBranch
  +bool IsPrivate
  +bool IsFork
  +int64 ForkID
}

class Issue {
  +int64 ID
  +int64 RepoID
  +int64 Index        // per-repo number
  +int64 PosterID
  +string Title
  +bool IsPull
  +bool IsClosed
}

class PullRequest {
  +int64 ID
  +int64 IssueID      // (conceptually) 1:1 with Issue when IsPull=true
  +string HeadBranch
  +string BaseBranch
  +bool HasMerged
}

class Comment {
  +int64 ID
  +int64 IssueID
  +int64 PosterID
  +string Content
}

class Label {
  +int64 ID
  +int64 RepoID
  +string Name
  +string Color
}

class Milestone {
  +int64 ID
  +int64 RepoID
  +string Name
  +bool IsClosed
}

class Review {
  +int64 ID
  +int64 IssueID      // PR review attached to the PR's Issue
  +int64 ReviewerID
  +ReviewState State
}

class Branch {
  +int64 RepoID
  +string Name
  +string CommitID
}

class ProtectedBranch {
  +int64 RepoID
  +string BranchNamePattern
  +bool EnablePush
  +bool EnableMergeWhitelist
}

class ProtectedTag {
  +int64 RepoID
  +string TagNamePattern
}

class CommitStatus {
  +int64 RepoID
  +string SHA
  +string Context
  +StatusState State
  +int64 CreatorID
}

class Collaboration {
  +int64 ID
  +int64 RepoID
  +int64 UserID
  +AccessMode Mode
}

class Watch {
  +int64 RepoID
  +int64 UserID
  +bool IsWatching
}

class Star {
  +int64 RepoID
  +int64 UserID
}

%% Core ownership
User "1" --> "many" Repository : owns (OwnerID)
Repository "many" --> "1" User : owner (OwnerID)

%% Issues / PRs are repo-scoped and user-authored
Repository "1" --> "many" Issue : contains (RepoID)
User "1" --> "many" Issue : posts (PosterID)

%% PullRequest is PR-specific state layered on an Issue
Issue "1" --> "0..1" PullRequest : if IsPull=true

%% Discussion and planning
Issue "1" --> "many" Comment : has
User "1" --> "many" Comment : writes (PosterID)
Repository "1" --> "many" Label : defines
Repository "1" --> "many" Milestone : defines
Issue "many" --> "many" Label : tagged
Issue "many" --> "0..1" Milestone : in

%% PR workflow
Issue "1" --> "many" Review : has
User "1" --> "many" Review : performs (ReviewerID)

%% Git-facing (repo-scoped policy + status)
Repository "1" --> "many" Branch : has
Repository "1" --> "many" ProtectedBranch : policy
Repository "1" --> "many" ProtectedTag : policy
Repository "1" --> "many" CommitStatus : status checks
User "1" --> "many" CommitStatus : sets (CreatorID)

%% Access + signals (repo<->user join tables)
Repository "1" --> "many" Collaboration : collaborators
User "1" --> "many" Collaboration : on repos
Repository "1" --> "many" Watch : watch records
User "1" --> "many" Watch : watches repos
Repository "1" --> "many" Star : stars
User "1" --> "many" Star : stars repos
```

## Diagram (PlantUML)

Paste the following into PlantText to render:

- https://www.planttext.com/

```plantuml
@startuml
skinparam classAttributeIconSize 0
left to right direction

class User {
  +ID : int64
  +Name : string
  +LowerName : string
  +Type : UserType
}

class Repository {
  +ID : int64
  +OwnerID : int64
  +OwnerName : string
  +Name : string
  +LowerName : string
  +DefaultBranch : string
  +IsPrivate : bool
  +IsFork : bool
  +ForkID : int64
}

class Issue {
  +ID : int64
  +RepoID : int64
  +Index : int64
  +PosterID : int64
  +Title : string
  +IsPull : bool
  +IsClosed : bool
}

class PullRequest {
  +ID : int64
  +IssueID : int64
  +HeadBranch : string
  +BaseBranch : string
  +HasMerged : bool
}

class Comment {
  +ID : int64
  +IssueID : int64
  +PosterID : int64
}

class Label {
  +ID : int64
  +RepoID : int64
  +Name : string
}

class Milestone {
  +ID : int64
  +RepoID : int64
  +Name : string
  +IsClosed : bool
}

class Review {
  +ID : int64
  +IssueID : int64
  +ReviewerID : int64
  +State : ReviewState
}

class Branch {
  +RepoID : int64
  +Name : string
  +CommitID : string
}

class ProtectedBranch {
  +RepoID : int64
  +BranchNamePattern : string
}

class ProtectedTag {
  +RepoID : int64
  +TagNamePattern : string
}

class CommitStatus {
  +RepoID : int64
  +SHA : string
  +Context : string
  +State : StatusState
  +CreatorID : int64
}

class Collaboration {
  +RepoID : int64
  +UserID : int64
  +Mode : AccessMode
}

class Watch {
  +RepoID : int64
  +UserID : int64
  +IsWatching : bool
}

class Star {
  +RepoID : int64
  +UserID : int64
}

User "1" -- "0..*" Repository : owns (OwnerID)
Repository "1" -- "0..*" Issue : contains (RepoID)
User "1" -- "0..*" Issue : posts (PosterID)
Issue "1" -- "0..1" PullRequest : PR overlay

Issue "1" -- "0..*" Comment
User "1" -- "0..*" Comment

Repository "1" -- "0..*" Label
Repository "1" -- "0..*" Milestone
Issue "0..*" -- "0..*" Label
Issue "0..*" -- "0..1" Milestone

Issue "1" -- "0..*" Review
User "1" -- "0..*" Review

Repository "1" -- "0..*" Branch
Repository "1" -- "0..*" ProtectedBranch
Repository "1" -- "0..*" ProtectedTag
Repository "1" -- "0..*" CommitStatus
User "1" -- "0..*" CommitStatus

Repository "1" -- "0..*" Collaboration
User "1" -- "0..*" Collaboration
Repository "1" -- "0..*" Watch
User "1" -- "0..*" Watch
Repository "1" -- "0..*" Star
User "1" -- "0..*" Star
@enduml
```

