use serde::{Deserialize, Serialize};

use crate::models::pulls::Thread;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(deprecated)]
pub struct PullRequestReviewThreadWebhookEventPayload {
    pub action: PullRequestReviewThreadWebhookEventAction,
    pub pull_request: crate::models::pulls::PullRequest,
    pub thread: Thread,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PullRequestReviewThreadWebhookEventAction {
    Resolved,
    Unresolved,
}
