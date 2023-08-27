use crate::profanity::check_profanity;
use crate::store;
use crate::types::account::Session;
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
    session: Session,
    store: store::Store,
    new_question: NewQuestion,
) -> Result<impl Reply, Rejection> {
    let account_id = session.account_id;
    let title = check_profanity(new_question.title);

    let content = check_profanity(new_question.content);

    let (title, content) = tokio::join!(title, content);

    let title = match title {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match content {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    match store.add_question(question, account_id).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    session: Session,
    store: store::Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    if store
        .is_question_owner(question.id.0, session.account_id)
        .await?
    {
        let title = check_profanity(question.title);

        let content = check_profanity(question.content);

        let (title, content) = tokio::join!(title, content);

        let title = match title {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };

        let content = match content {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };

        let question = Question {
            id: question.id,
            title,
            content,
            tags: question.tags,
        };

        match store.update_question(question, id).await {
            Ok(question) => Ok(warp::reply::json(&question)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(handle_error::Error::Unauthorized))
    }
}

pub async fn delete_question(id: i32, store: store::Store) -> Result<impl Reply, Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
}
