use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct QuestionId(pub i32);

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
