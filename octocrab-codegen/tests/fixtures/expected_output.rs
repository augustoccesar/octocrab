use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestHead {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub sha: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleUser {
    pub login: String,
    pub id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: i64,
    pub head: PullRequestHead,
    pub user: SimpleUser,
    pub merged_by: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PullRequestSimpleState {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "CLOSED")]
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimpleLabelsItem {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimple {
    pub id: i64,
    pub name: String,
    pub score: f64,
    pub draft: bool,
    pub description: Option<String>,
    /// The state of the pull request.
    pub state: PullRequestSimpleState,
    pub tags: Vec<String>,
    pub labels: Vec<PullRequestSimpleLabelsItem>,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimal {
    /// The unique identifier.
    pub id: i64,
    pub url: String,
}