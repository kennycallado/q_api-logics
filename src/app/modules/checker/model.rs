use serde::{Deserialize, Serialize};

use crate::app::providers::services::answer::PubNewAnswer;
use crate::app::providers::services::paper::PubPaperPush;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaperPushWithAction {
    pub id: i32,
    pub user_id: i32,
    pub user_record: rocket::serde::json::Value,
    pub project_id: i32,
    pub resource_id: i32,
    pub completed: bool,
    pub answers: Option<Vec<PubNewAnswer>>,
    pub actions: Option<Vec<Action>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Action {
    pub action: String,
    pub params: Vec<i32>
}

impl From<PaperPushWithAction> for PubPaperPush {
    fn from(paper_push: PaperPushWithAction) -> Self {
        PubPaperPush {
            id: paper_push.id,
            user_id: paper_push.user_id,
            user_record: paper_push.user_record,
            project_id: paper_push.project_id,
            resource_id: paper_push.resource_id,
            completed: paper_push.completed,
            answers: paper_push.answers,
        }
    }
}
