use reqwest::Error as ReqwestError;
use std::fmt;
use warp::{
    body::BodyDeserializeError, filters::cors::CorsForbidden, http::StatusCode, reject::Reject,
    Rejection, Reply,
};

use tracing::{event, instrument, Level};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    DatabaseQueryError,
    ExternalAPIError(ReqwestError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(ref err) => write!(f, "Parse error: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::DatabaseQueryError => write!(f, "Query could not be executed"),
            Error::ExternalAPIError(err) => write!(f, "Cannot execute {}", err),
        }
    }
}

impl Reject for Error {}

#[instrument]
pub async fn return_error(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError) = rejection.find() {
        event!(Level::ERROR, "Database query error");
        Ok(warp::reply::with_status(
            Error::DatabaseQueryError.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::ExternalAPIError(e)) = rejection.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(error) = rejection.find::<CorsForbidden>() {
        event!(Level::ERROR, "CORS forbidden error {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = rejection.find::<BodyDeserializeError>() {
        event!(Level::ERROR, "Cannot deserialize request body {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = rejection.find::<Error>() {
        event!(Level::ERROR, "{}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else {
        event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
