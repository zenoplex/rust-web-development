use crate::store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId};
use handle_error::Error;
use std::collections::HashMap;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn get_questions(
    params: HashMap<String, String>,
    store: store::Store,
    uuid: String,
) -> Result<impl Reply, Rejection> {
    log::info!("{} Start querying questions", uuid);
    let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        // TODO: check if pagination range is valid
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        Ok(warp::reply::json(&res))
    }
}

pub async fn get_question(id: String, store: store::Store) -> Result<impl Reply, Rejection> {
    match store.questions.read().await.get(&QuestionId(id)) {
        Some(q) => Ok(warp::reply::json(&q)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

pub async fn add_question(
    store: store::Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn update_question(
    id: String,
    store: store::Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(id: String, store: store::Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
