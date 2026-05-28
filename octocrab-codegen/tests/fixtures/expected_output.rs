use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestHead {
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub sha: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleUser {
    pub login: Option<String>,
    pub id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: Option<i64>,
    pub head: Option<PullRequestHead>,
    pub user: Option<SimpleUser>,
    pub merged_by: Option<SimpleUser>,
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
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimple {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub score: Option<f64>,
    pub draft: Option<bool>,
    pub description: Option<String>,
    /// The state of the pull request.
    pub state: Option<PullRequestSimpleState>,
    pub tags: Option<Vec<String>>,
    pub labels: Option<Vec<PullRequestSimpleLabelsItem>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimal {
    /// The unique identifier.
    pub id: Option<i64>,
    pub url: Option<String>,
}