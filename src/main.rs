use serde::Serialize;
use std::str::FromStr;
use warp::Filter;

#[derive(Debug, Serialize)]
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

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provider"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    Ok(warp::reply::json(&question))
}

#[tokio::main]
async fn main() {
    let get_items = warp::get().and(
        warp::path("questions")
            .and(warp::path::end())
            .map(|| "Hello, world!"), // .and_then(get_questions())
    );

    let routes = get_items;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
