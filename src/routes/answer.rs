use crate::store;
use crate::types::answer::NewAnswer;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn add_answer(
    store: store::Store,
    new_answer: NewAnswer,
) -> Result<impl Reply, Rejection> {
    if let Err(e) = store.add_answer(new_answer).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
