use serde::{Deserialize, Serialize};

// use crate::app::providers::interfaces::question::PubQuestion;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubAnswer {
    pub id: i32,
    pub question_id: i32,
    pub answer: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubNewAnswer {
    pub question_id: i32,
    pub answer: String,
}
