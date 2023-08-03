use crate::store;
use crate::types::answer::{Answer, AnswerId};
use crate::types::question::QuestionId;
use std::collections::HashMap;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn add_answer(
    store: store::Store,
    params: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        // TODO: Stop using unwrap
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("question_id").unwrap().to_string()),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
