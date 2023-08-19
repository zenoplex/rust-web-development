use crate::profanity::check_profanity;
use crate::store;
use crate::types::pagination::extract_pagination;
use crate::types::pagination::Pagination;
use crate::types::question::NewQuestion;
use crate::types::question::Question;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{event, instrument, Level};
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: store::Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "rust_web_development", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err(warp::reject::custom(e));
        }
    };

    Ok(warp::reply::json(&res))
}

// pub async fn get_question(id: i32, store: store::Store) -> Result<impl Reply, Rejection> {
//     match store.questions.read().await.get(&QuestionId(id)) {
//         Some(q) => Ok(warp::reply::json(&q)),
//         None => Err(warp::reject::custom(Error::QuestionNotFound)),
//     }
// }

pub async fn add_question(
    store: store::Store,
    new_question: NewQuestion,
) -> Result<impl Reply, Rejection> {
    let title = match check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    match store.add_question(question).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: store::Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    if let Err(e) = store.update_question(question, id).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(id: i32, store: store::Store) -> Result<impl Reply, Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
}
