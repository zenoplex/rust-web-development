use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;
use warp::{
    filters::cors::CorsForbidden, http::Method, http::StatusCode, reject::Reject, Filter,
    Rejection, Reply,
};

struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }

    fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }

    fn init(self) -> Self {
      let question = Question::new(
        QuestionId::from_str("1").expect("Id not set"),
        "How?".to_string(),
        "Please help!".to_string(),
        Some(vec!["general".to_string()]),
      );
      self.add_question(question)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
struct QuestionId(String);

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(QuestionId(s.to_string()))
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions() -> Result<impl Reply, Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provider"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    match question.id.0.parse::<u32>() {
        Err(_) => Err(warp::reject::custom(InvalidId)),
        Ok(_) => Ok(warp::reply::json(&question)),
    }
}

async fn return_error(rejection: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", rejection);

    if let Some(error) = rejection.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(InvalidId) = rejection.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);

    let routes = get_items.with(cors).recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
