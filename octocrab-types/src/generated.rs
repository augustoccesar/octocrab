use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PullRequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,
    pub starred_at: Option<String>,
    pub user_view_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestLabelsItem {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NullableMilestoneState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullableSimpleUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,
    pub starred_at: Option<String>,
    pub user_view_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullableMilestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: i64,
    pub node_id: String,
    /// The number of the milestone.
    pub number: i64,
    /// The state of the milestone.
    pub state: NullableMilestoneState,
    /// The title of the milestone.
    pub title: String,
    pub description: Option<String>,
    /// A GitHub user.
    pub creator: Option<NullableSimpleUser>,
    pub open_issues: i64,
    pub closed_issues: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub due_on: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TeamSimpleType {
    #[serde(rename = "enterprise")]
    Enterprise,
    #[serde(rename = "organization")]
    Organization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamSimple {
    /// Unique identifier of the team
    pub id: i64,
    pub node_id: String,
    /// URL for the team
    pub url: String,
    pub members_url: String,
    /// Name of the team
    pub name: String,
    /// Description of the team
    pub description: Option<String>,
    /// Permission that the team will have for its repositories
    pub permission: String,
    /// The level of privacy this team should have
    pub privacy: Option<String>,
    /// The notification setting the team has set
    pub notification_setting: Option<String>,
    pub html_url: String,
    pub repositories_url: String,
    pub slug: String,
    /// Distinguished Name (DN) that team maps to within LDAP environment
    pub ldap_dn: Option<String>,
    /// The ownership type of the team
    #[serde(rename = "type")]
    pub type_: TeamSimpleType,
    /// Unique identifier of the organization to which this team belongs
    pub organization_id: Option<i64>,
    /// Unique identifier of the enterprise to which this team belongs
    pub enterprise_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullableLicenseSimple {
    pub key: String,
    pub name: String,
    pub url: Option<String>,
    pub spdx_id: Option<String>,
    pub node_id: String,
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryPermissions {
    pub admin: bool,
    pub pull: bool,
    pub triage: Option<bool>,
    pub push: bool,
    pub maintain: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositoryPullRequestCreationPolicy {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "collaborators_only")]
    CollaboratorsOnly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositorySquashMergeCommitTitle {
    #[serde(rename = "PR_TITLE")]
    PrTitle,
    #[serde(rename = "COMMIT_OR_PR_TITLE")]
    CommitOrPrTitle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositorySquashMergeCommitMessage {
    #[serde(rename = "PR_BODY")]
    PrBody,
    #[serde(rename = "COMMIT_MESSAGES")]
    CommitMessages,
    #[serde(rename = "BLANK")]
    Blank,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositoryMergeCommitTitle {
    #[serde(rename = "PR_TITLE")]
    PrTitle,
    #[serde(rename = "MERGE_MESSAGE")]
    MergeMessage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositoryMergeCommitMessage {
    #[serde(rename = "PR_BODY")]
    PrBody,
    #[serde(rename = "PR_TITLE")]
    PrTitle,
    #[serde(rename = "BLANK")]
    Blank,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryCodeSearchIndexStatus {
    pub lexical_search_ok: Option<bool>,
    pub lexical_commit_sha: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    /// Unique identifier of the repository
    pub id: i64,
    pub node_id: String,
    /// The name of the repository.
    pub name: String,
    pub full_name: String,
    /// License Simple
    pub license: Option<NullableLicenseSimple>,
    pub forks: i64,
    pub permissions: Option<RepositoryPermissions>,
    /// A GitHub user.
    pub owner: SimpleUser,
    /// Whether the repository is private or public.
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: String,
    pub mirror_url: Option<String>,
    pub hooks_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    /// The size of the repository, in kilobytes. Size is calculated hourly. When a repository is initially created, the size is 0.
    pub size: i64,
    /// The default branch of the repository.
    pub default_branch: String,
    pub open_issues_count: i64,
    /// Whether this repository acts as a template that can be used to generate new repositories.
    pub is_template: Option<bool>,
    pub topics: Option<Vec<String>>,
    /// Whether issues are enabled.
    pub has_issues: bool,
    /// Whether projects are enabled.
    pub has_projects: bool,
    /// Whether the wiki is enabled.
    pub has_wiki: bool,
    pub has_pages: bool,
    /// Whether downloads are enabled.
    pub has_downloads: bool,
    /// Whether discussions are enabled.
    pub has_discussions: Option<bool>,
    /// Whether pull requests are enabled.
    pub has_pull_requests: Option<bool>,
    /// The policy controlling who can create pull requests: all or collaborators_only.
    pub pull_request_creation_policy: Option<RepositoryPullRequestCreationPolicy>,
    /// Whether the repository is archived.
    pub archived: bool,
    /// Returns whether or not this repository disabled.
    pub disabled: bool,
    /// The repository visibility: public, private, or internal.
    pub visibility: Option<String>,
    pub pushed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    /// Whether to allow rebase merges for pull requests.
    pub allow_rebase_merge: Option<bool>,
    pub temp_clone_token: Option<String>,
    /// Whether to allow squash merges for pull requests.
    pub allow_squash_merge: Option<bool>,
    /// Whether to allow Auto-merge to be used on pull requests.
    pub allow_auto_merge: Option<bool>,
    /// Whether to delete head branches when pull requests are merged
    pub delete_branch_on_merge: Option<bool>,
    /// Whether or not a pull request head branch that is behind its base branch can always be updated even if it is not required to be up to date before merging.
    pub allow_update_branch: Option<bool>,
    /// Whether a squash merge commit can use the pull request title as default. **This property is closing down. Please use `squash_merge_commit_title` instead.
    pub use_squash_pr_title_as_default: Option<bool>,
    /// The default value for a squash merge commit title:
    ///
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
    pub squash_merge_commit_title: Option<RepositorySquashMergeCommitTitle>,
    /// The default value for a squash merge commit message:
    ///
    /// - `PR_BODY` - default to the pull request's body.
    /// - `COMMIT_MESSAGES` - default to the branch's commit messages.
    /// - `BLANK` - default to a blank commit message.
    pub squash_merge_commit_message: Option<RepositorySquashMergeCommitMessage>,
    /// The default value for a merge commit title.
    ///
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name).
    pub merge_commit_title: Option<RepositoryMergeCommitTitle>,
    /// The default value for a merge commit message.
    ///
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `PR_BODY` - default to the pull request's body.
    /// - `BLANK` - default to a blank commit message.
    pub merge_commit_message: Option<RepositoryMergeCommitMessage>,
    /// Whether to allow merge commits for pull requests.
    pub allow_merge_commit: Option<bool>,
    /// Whether to allow forking this repo
    pub allow_forking: Option<bool>,
    /// Whether to require contributors to sign off on web-based commits
    pub web_commit_signoff_required: Option<bool>,
    pub open_issues: i64,
    pub watchers: i64,
    pub master_branch: Option<String>,
    pub starred_at: Option<String>,
    /// Whether anonymous git access is enabled for this repository
    pub anonymous_access_enabled: Option<bool>,
    /// The status of the code search index for this repository
    pub code_search_index_status: Option<RepositoryCodeSearchIndexStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    /// A repository on GitHub.
    pub repo: Repository,
    pub sha: String,
    /// A GitHub user.
    pub user: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    /// A repository on GitHub.
    pub repo: Repository,
    pub sha: String,
    /// A GitHub user.
    pub user: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestLinks {
    /// Hypermedia Link
    pub comments: Link,
    /// Hypermedia Link
    pub commits: Link,
    /// Hypermedia Link
    pub statuses: Link,
    /// Hypermedia Link
    pub html: Link,
    /// Hypermedia Link
    pub issue: Link,
    /// Hypermedia Link
    pub review_comments: Link,
    /// Hypermedia Link
    pub review_comment: Link,
    /// Hypermedia Link
    #[serde(rename = "self")]
    pub self_: Link,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PullRequestAuthorAssociation {
    #[serde(rename = "COLLABORATOR")]
    Collaborator,
    #[serde(rename = "CONTRIBUTOR")]
    Contributor,
    #[serde(rename = "FIRST_TIMER")]
    FirstTimer,
    #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
    FirstTimeContributor,
    #[serde(rename = "MANNEQUIN")]
    Mannequin,
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OWNER")]
    Owner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AutoMergeMergeMethod {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "squash")]
    Squash,
    #[serde(rename = "rebase")]
    Rebase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMerge {
    /// A GitHub user.
    pub enabled_by: SimpleUser,
    /// The merge method to use.
    pub merge_method: AutoMergeMergeMethod,
    /// Title for the merge commit message.
    pub commit_title: String,
    /// Commit message for the merge commit.
    pub commit_message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    /// Number uniquely identifying the pull request within its repository.
    pub number: i64,
    /// State of this Pull Request. Either `open` or `closed`.
    pub state: PullRequestState,
    pub locked: bool,
    /// The title of the pull request.
    pub title: String,
    /// A GitHub user.
    pub user: SimpleUser,
    pub body: Option<String>,
    pub labels: Vec<PullRequestLabelsItem>,
    /// A collection of related issues and pull requests.
    pub milestone: Option<NullableMilestone>,
    pub active_lock_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    /// A GitHub user.
    pub assignee: Option<NullableSimpleUser>,
    pub assignees: Option<Vec<SimpleUser>>,
    pub requested_reviewers: Option<Vec<SimpleUser>>,
    pub requested_teams: Option<Vec<TeamSimple>>,
    pub head: PullRequestHead,
    pub base: PullRequestBase,
    pub _links: PullRequestLinks,
    /// How the author is associated with the repository.
    pub author_association: PullRequestAuthorAssociation,
    /// The status of auto merging a pull request.
    pub auto_merge: Option<AutoMerge>,
    /// Indicates whether or not the pull request is a draft.
    pub draft: Option<bool>,
    pub merged: bool,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: String,
    /// A GitHub user.
    pub merged_by: Option<NullableSimpleUser>,
    pub comments: i64,
    pub review_comments: i64,
    /// Indicates whether maintainers can modify the pull request.
    pub maintainer_can_modify: bool,
    pub commits: i64,
    pub additions: i64,
    pub deletions: i64,
    pub changed_files: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimpleLabelsItem {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub description: String,
    pub color: String,
    pub default: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamPermissions {
    pub pull: bool,
    pub triage: bool,
    pub push: bool,
    pub maintain: bool,
    pub admin: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TeamType {
    #[serde(rename = "enterprise")]
    Enterprise,
    #[serde(rename = "organization")]
    Organization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NullableTeamSimpleType {
    #[serde(rename = "enterprise")]
    Enterprise,
    #[serde(rename = "organization")]
    Organization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullableTeamSimple {
    /// Unique identifier of the team
    pub id: i64,
    pub node_id: String,
    /// URL for the team
    pub url: String,
    pub members_url: String,
    /// Name of the team
    pub name: String,
    /// Description of the team
    pub description: Option<String>,
    /// Permission that the team will have for its repositories
    pub permission: String,
    /// The level of privacy this team should have
    pub privacy: Option<String>,
    /// The notification setting the team has set
    pub notification_setting: Option<String>,
    pub html_url: String,
    pub repositories_url: String,
    pub slug: String,
    /// Distinguished Name (DN) that team maps to within LDAP environment
    pub ldap_dn: Option<String>,
    /// The ownership type of the team
    #[serde(rename = "type")]
    pub type_: NullableTeamSimpleType,
    /// Unique identifier of the organization to which this team belongs
    pub organization_id: Option<i64>,
    /// Unique identifier of the enterprise to which this team belongs
    pub enterprise_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: Option<String>,
    pub notification_setting: Option<String>,
    pub permission: String,
    pub permissions: Option<TeamPermissions>,
    pub url: String,
    pub html_url: String,
    pub members_url: String,
    pub repositories_url: String,
    /// The ownership type of the team
    #[serde(rename = "type")]
    pub type_: TeamType,
    /// Unique identifier of the organization to which this team belongs
    pub organization_id: Option<i64>,
    /// Unique identifier of the enterprise to which this team belongs
    pub enterprise_id: Option<i64>,
    /// Groups of organization members that gives permissions on specified repositories.
    pub parent: Option<NullableTeamSimple>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimpleHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    /// A repository on GitHub.
    pub repo: Repository,
    pub sha: String,
    /// A GitHub user.
    pub user: Option<NullableSimpleUser>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimpleBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    /// A repository on GitHub.
    pub repo: Repository,
    pub sha: String,
    /// A GitHub user.
    pub user: Option<NullableSimpleUser>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimpleLinks {
    /// Hypermedia Link
    pub comments: Link,
    /// Hypermedia Link
    pub commits: Link,
    /// Hypermedia Link
    pub statuses: Link,
    /// Hypermedia Link
    pub html: Link,
    /// Hypermedia Link
    pub issue: Link,
    /// Hypermedia Link
    pub review_comments: Link,
    /// Hypermedia Link
    pub review_comment: Link,
    /// Hypermedia Link
    #[serde(rename = "self")]
    pub self_: Link,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PullRequestSimpleAuthorAssociation {
    #[serde(rename = "COLLABORATOR")]
    Collaborator,
    #[serde(rename = "CONTRIBUTOR")]
    Contributor,
    #[serde(rename = "FIRST_TIMER")]
    FirstTimer,
    #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
    FirstTimeContributor,
    #[serde(rename = "MANNEQUIN")]
    Mannequin,
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OWNER")]
    Owner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimple {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub number: i64,
    pub state: String,
    pub locked: bool,
    pub title: String,
    /// A GitHub user.
    pub user: Option<NullableSimpleUser>,
    pub body: Option<String>,
    pub labels: Vec<PullRequestSimpleLabelsItem>,
    /// A collection of related issues and pull requests.
    pub milestone: Option<NullableMilestone>,
    pub active_lock_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    /// A GitHub user.
    pub assignee: Option<NullableSimpleUser>,
    pub assignees: Option<Vec<SimpleUser>>,
    pub requested_reviewers: Option<Vec<SimpleUser>>,
    pub requested_teams: Option<Vec<Team>>,
    pub head: PullRequestSimpleHead,
    pub base: PullRequestSimpleBase,
    pub _links: PullRequestSimpleLinks,
    /// How the author is associated with the repository.
    pub author_association: PullRequestSimpleAuthorAssociation,
    /// The status of auto merging a pull request.
    pub auto_merge: Option<AutoMerge>,
    /// Indicates whether or not the pull request is a draft.
    pub draft: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimalHeadRepo {
    pub id: i64,
    pub url: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimalHead {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub sha: String,
    pub repo: PullRequestMinimalHeadRepo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimalBaseRepo {
    pub id: i64,
    pub url: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimalBase {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub sha: String,
    pub repo: PullRequestMinimalBaseRepo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimal {
    pub id: i64,
    pub number: i64,
    pub url: String,
    pub head: PullRequestMinimalHead,
    pub base: PullRequestMinimalBase,
}

pub type PullsGetResponse = PullRequest;
