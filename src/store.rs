use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};

#[derive(Clone)]
pub struct Store {
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            answers: Arc::new(RwLock::new(HashMap::new())),
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file: &str = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }
}
