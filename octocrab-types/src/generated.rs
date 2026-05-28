use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PullRequestState {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleUser {
    name: Option<String>,
    email: Option<String>,
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: Option<String>,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_: String,
    site_admin: bool,
    starred_at: String,
    user_view_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelsItem {
    id: i64,
    node_id: String,
    url: String,
    name: String,
    description: Option<String>,
    color: String,
    default: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NullableMilestoneState {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullableSimpleUser {
    name: Option<String>,
    email: Option<String>,
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: Option<String>,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_: String,
    site_admin: bool,
    starred_at: String,
    user_view_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullableMilestone {
    url: String,
    html_url: String,
    labels_url: String,
    id: i64,
    node_id: String,
    /// The number of the milestone.
    number: i64,
    /// The state of the milestone.
    state: NullableMilestoneState,
    /// The title of the milestone.
    title: String,
    description: Option<String>,
    /// A GitHub user.
    creator: Option<NullableSimpleUser>,
    open_issues: i64,
    closed_issues: i64,
    created_at: String,
    updated_at: String,
    closed_at: Option<String>,
    due_on: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TeamSimpleType {
    Enterprise,
    Organization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamSimple {
    /// Unique identifier of the team
    id: i64,
    node_id: String,
    /// URL for the team
    url: String,
    members_url: String,
    /// Name of the team
    name: String,
    /// Description of the team
    description: Option<String>,
    /// Permission that the team will have for its repositories
    permission: String,
    /// The level of privacy this team should have
    privacy: String,
    /// The notification setting the team has set
    notification_setting: String,
    html_url: String,
    repositories_url: String,
    slug: String,
    /// Distinguished Name (DN) that team maps to within LDAP environment
    ldap_dn: String,
    /// The ownership type of the team
    #[serde(rename = "type")]
    type_: TeamSimpleType,
    /// Unique identifier of the organization to which this team belongs
    organization_id: i64,
    /// Unique identifier of the enterprise to which this team belongs
    enterprise_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullableLicenseSimple {
    key: String,
    name: String,
    url: Option<String>,
    spdx_id: Option<String>,
    node_id: String,
    html_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryPermissions {
    admin: bool,
    pull: bool,
    triage: bool,
    push: bool,
    maintain: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositoryPullRequestCreationPolicy {
    All,
    CollaboratorsOnly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositorySquashMergeCommitTitle {
    PRTITLE,
    COMMITORPRTITLE,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositorySquashMergeCommitMessage {
    PRBODY,
    COMMITMESSAGES,
    BLANK,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositoryMergeCommitTitle {
    PRTITLE,
    MERGEMESSAGE,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepositoryMergeCommitMessage {
    PRBODY,
    PRTITLE,
    BLANK,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryCodeSearchIndexStatus {
    lexical_search_ok: bool,
    lexical_commit_sha: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    /// Unique identifier of the repository
    id: i64,
    node_id: String,
    /// The name of the repository.
    name: String,
    full_name: String,
    /// License Simple
    license: Option<NullableLicenseSimple>,
    forks: i64,
    permissions: RepositoryPermissions,
    /// A GitHub user.
    owner: SimpleUser,
    /// Whether the repository is private or public.
    private: bool,
    html_url: String,
    description: Option<String>,
    fork: bool,
    url: String,
    archive_url: String,
    assignees_url: String,
    blobs_url: String,
    branches_url: String,
    collaborators_url: String,
    comments_url: String,
    commits_url: String,
    compare_url: String,
    contents_url: String,
    contributors_url: String,
    deployments_url: String,
    downloads_url: String,
    events_url: String,
    forks_url: String,
    git_commits_url: String,
    git_refs_url: String,
    git_tags_url: String,
    git_url: String,
    issue_comment_url: String,
    issue_events_url: String,
    issues_url: String,
    keys_url: String,
    labels_url: String,
    languages_url: String,
    merges_url: String,
    milestones_url: String,
    notifications_url: String,
    pulls_url: String,
    releases_url: String,
    ssh_url: String,
    stargazers_url: String,
    statuses_url: String,
    subscribers_url: String,
    subscription_url: String,
    tags_url: String,
    teams_url: String,
    trees_url: String,
    clone_url: String,
    mirror_url: Option<String>,
    hooks_url: String,
    svn_url: String,
    homepage: Option<String>,
    language: Option<String>,
    forks_count: i64,
    stargazers_count: i64,
    watchers_count: i64,
    /// The size of the repository, in kilobytes. Size is calculated hourly. When a repository is initially created, the size is 0.
    size: i64,
    /// The default branch of the repository.
    default_branch: String,
    open_issues_count: i64,
    /// Whether this repository acts as a template that can be used to generate new repositories.
    is_template: bool,
    topics: Vec<String>,
    /// Whether issues are enabled.
    has_issues: bool,
    /// Whether projects are enabled.
    has_projects: bool,
    /// Whether the wiki is enabled.
    has_wiki: bool,
    has_pages: bool,
    /// Whether downloads are enabled.
    has_downloads: bool,
    /// Whether discussions are enabled.
    has_discussions: bool,
    /// Whether pull requests are enabled.
    has_pull_requests: bool,
    /// The policy controlling who can create pull requests: all or collaborators_only.
    pull_request_creation_policy: RepositoryPullRequestCreationPolicy,
    /// Whether the repository is archived.
    archived: bool,
    /// Returns whether or not this repository disabled.
    disabled: bool,
    /// The repository visibility: public, private, or internal.
    visibility: String,
    pushed_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    /// Whether to allow rebase merges for pull requests.
    allow_rebase_merge: bool,
    temp_clone_token: String,
    /// Whether to allow squash merges for pull requests.
    allow_squash_merge: bool,
    /// Whether to allow Auto-merge to be used on pull requests.
    allow_auto_merge: bool,
    /// Whether to delete head branches when pull requests are merged
    delete_branch_on_merge: bool,
    /// Whether or not a pull request head branch that is behind its base branch can always be updated even if it is not required to be up to date before merging.
    allow_update_branch: bool,
    /// Whether a squash merge commit can use the pull request title as default. **This property is closing down. Please use `squash_merge_commit_title` instead.
    use_squash_pr_title_as_default: bool,
    /// The default value for a squash merge commit title:
    /// 
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
    squash_merge_commit_title: RepositorySquashMergeCommitTitle,
    /// The default value for a squash merge commit message:
    /// 
    /// - `PR_BODY` - default to the pull request's body.
    /// - `COMMIT_MESSAGES` - default to the branch's commit messages.
    /// - `BLANK` - default to a blank commit message.
    squash_merge_commit_message: RepositorySquashMergeCommitMessage,
    /// The default value for a merge commit title.
    /// 
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name).
    merge_commit_title: RepositoryMergeCommitTitle,
    /// The default value for a merge commit message.
    /// 
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `PR_BODY` - default to the pull request's body.
    /// - `BLANK` - default to a blank commit message.
    merge_commit_message: RepositoryMergeCommitMessage,
    /// Whether to allow merge commits for pull requests.
    allow_merge_commit: bool,
    /// Whether to allow forking this repo
    allow_forking: bool,
    /// Whether to require contributors to sign off on web-based commits
    web_commit_signoff_required: bool,
    open_issues: i64,
    watchers: i64,
    master_branch: String,
    starred_at: String,
    /// Whether anonymous git access is enabled for this repository
    anonymous_access_enabled: bool,
    /// The status of the code search index for this repository
    code_search_index_status: RepositoryCodeSearchIndexStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestHead {
    label: String,
    #[serde(rename = "ref")]
    ref_: String,
    /// A repository on GitHub.
    repo: Repository,
    sha: String,
    /// A GitHub user.
    user: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestBase {
    label: String,
    #[serde(rename = "ref")]
    ref_: String,
    /// A repository on GitHub.
    repo: Repository,
    sha: String,
    /// A GitHub user.
    user: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    href: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestLinks {
    /// Hypermedia Link
    comments: Link,
    /// Hypermedia Link
    commits: Link,
    /// Hypermedia Link
    statuses: Link,
    /// Hypermedia Link
    html: Link,
    /// Hypermedia Link
    issue: Link,
    /// Hypermedia Link
    review_comments: Link,
    /// Hypermedia Link
    review_comment: Link,
    /// Hypermedia Link
    #[serde(rename = "self")]
    self_: Link,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PullRequestAuthorAssociation {
    COLLABORATOR,
    CONTRIBUTOR,
    FIRSTTIMER,
    FIRSTTIMECONTRIBUTOR,
    MANNEQUIN,
    MEMBER,
    NONE,
    OWNER,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AutoMergeMergeMethod {
    Merge,
    Squash,
    Rebase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMerge {
    /// A GitHub user.
    enabled_by: SimpleUser,
    /// The merge method to use.
    merge_method: AutoMergeMergeMethod,
    /// Title for the merge commit message.
    commit_title: String,
    /// Commit message for the merge commit.
    commit_message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    url: String,
    id: i64,
    node_id: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
    issue_url: String,
    commits_url: String,
    review_comments_url: String,
    review_comment_url: String,
    comments_url: String,
    statuses_url: String,
    /// Number uniquely identifying the pull request within its repository.
    number: i64,
    /// State of this Pull Request. Either `open` or `closed`.
    state: PullRequestState,
    locked: bool,
    /// The title of the pull request.
    title: String,
    /// A GitHub user.
    user: SimpleUser,
    body: Option<String>,
    labels: Vec<LabelsItem>,
    /// A collection of related issues and pull requests.
    milestone: Option<NullableMilestone>,
    active_lock_reason: Option<String>,
    created_at: String,
    updated_at: String,
    closed_at: Option<String>,
    merged_at: Option<String>,
    merge_commit_sha: Option<String>,
    /// A GitHub user.
    assignee: Option<NullableSimpleUser>,
    assignees: Vec<SimpleUser>,
    requested_reviewers: Vec<SimpleUser>,
    requested_teams: Vec<TeamSimple>,
    head: PullRequestHead,
    base: PullRequestBase,
    _links: PullRequestLinks,
    /// How the author is associated with the repository.
    author_association: PullRequestAuthorAssociation,
    /// The status of auto merging a pull request.
    auto_merge: Option<AutoMerge>,
    /// Indicates whether or not the pull request is a draft.
    draft: bool,
    merged: bool,
    mergeable: Option<bool>,
    rebaseable: Option<bool>,
    mergeable_state: String,
    /// A GitHub user.
    merged_by: Option<NullableSimpleUser>,
    comments: i64,
    review_comments: i64,
    /// Indicates whether maintainers can modify the pull request.
    maintainer_can_modify: bool,
    commits: i64,
    additions: i64,
    deletions: i64,
    changed_files: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamPermissions {
    pull: bool,
    triage: bool,
    push: bool,
    maintain: bool,
    admin: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TeamType {
    Enterprise,
    Organization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NullableTeamSimpleType {
    Enterprise,
    Organization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullableTeamSimple {
    /// Unique identifier of the team
    id: i64,
    node_id: String,
    /// URL for the team
    url: String,
    members_url: String,
    /// Name of the team
    name: String,
    /// Description of the team
    description: Option<String>,
    /// Permission that the team will have for its repositories
    permission: String,
    /// The level of privacy this team should have
    privacy: String,
    /// The notification setting the team has set
    notification_setting: String,
    html_url: String,
    repositories_url: String,
    slug: String,
    /// Distinguished Name (DN) that team maps to within LDAP environment
    ldap_dn: String,
    /// The ownership type of the team
    #[serde(rename = "type")]
    type_: NullableTeamSimpleType,
    /// Unique identifier of the organization to which this team belongs
    organization_id: i64,
    /// Unique identifier of the enterprise to which this team belongs
    enterprise_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    id: i64,
    node_id: String,
    name: String,
    slug: String,
    description: Option<String>,
    privacy: String,
    notification_setting: String,
    permission: String,
    permissions: TeamPermissions,
    url: String,
    html_url: String,
    members_url: String,
    repositories_url: String,
    /// The ownership type of the team
    #[serde(rename = "type")]
    type_: TeamType,
    /// Unique identifier of the organization to which this team belongs
    organization_id: i64,
    /// Unique identifier of the enterprise to which this team belongs
    enterprise_id: i64,
    /// Groups of organization members that gives permissions on specified repositories.
    parent: Option<NullableTeamSimple>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimpleHead {
    label: String,
    #[serde(rename = "ref")]
    ref_: String,
    /// A repository on GitHub.
    repo: Repository,
    sha: String,
    /// A GitHub user.
    user: Option<NullableSimpleUser>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimpleBase {
    label: String,
    #[serde(rename = "ref")]
    ref_: String,
    /// A repository on GitHub.
    repo: Repository,
    sha: String,
    /// A GitHub user.
    user: Option<NullableSimpleUser>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimpleLinks {
    /// Hypermedia Link
    comments: Link,
    /// Hypermedia Link
    commits: Link,
    /// Hypermedia Link
    statuses: Link,
    /// Hypermedia Link
    html: Link,
    /// Hypermedia Link
    issue: Link,
    /// Hypermedia Link
    review_comments: Link,
    /// Hypermedia Link
    review_comment: Link,
    /// Hypermedia Link
    #[serde(rename = "self")]
    self_: Link,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PullRequestSimpleAuthorAssociation {
    COLLABORATOR,
    CONTRIBUTOR,
    FIRSTTIMER,
    FIRSTTIMECONTRIBUTOR,
    MANNEQUIN,
    MEMBER,
    NONE,
    OWNER,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimple {
    url: String,
    id: i64,
    node_id: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
    issue_url: String,
    commits_url: String,
    review_comments_url: String,
    review_comment_url: String,
    comments_url: String,
    statuses_url: String,
    number: i64,
    state: String,
    locked: bool,
    title: String,
    /// A GitHub user.
    user: Option<NullableSimpleUser>,
    body: Option<String>,
    labels: Vec<LabelsItem>,
    /// A collection of related issues and pull requests.
    milestone: Option<NullableMilestone>,
    active_lock_reason: Option<String>,
    created_at: String,
    updated_at: String,
    closed_at: Option<String>,
    merged_at: Option<String>,
    merge_commit_sha: Option<String>,
    /// A GitHub user.
    assignee: Option<NullableSimpleUser>,
    assignees: Vec<SimpleUser>,
    requested_reviewers: Vec<SimpleUser>,
    requested_teams: Vec<Team>,
    head: PullRequestSimpleHead,
    base: PullRequestSimpleBase,
    _links: PullRequestSimpleLinks,
    /// How the author is associated with the repository.
    author_association: PullRequestSimpleAuthorAssociation,
    /// The status of auto merging a pull request.
    auto_merge: Option<AutoMerge>,
    /// Indicates whether or not the pull request is a draft.
    draft: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimalHeadRepo {
    id: i64,
    url: String,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimalHead {
    #[serde(rename = "ref")]
    ref_: String,
    sha: String,
    repo: PullRequestMinimalHeadRepo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimalBaseRepo {
    id: i64,
    url: String,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimalBase {
    #[serde(rename = "ref")]
    ref_: String,
    sha: String,
    repo: PullRequestMinimalBaseRepo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimal {
    id: i64,
    number: i64,
    url: String,
    head: PullRequestMinimalHead,
    base: PullRequestMinimalBase,
}