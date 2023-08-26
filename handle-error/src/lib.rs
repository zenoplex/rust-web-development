use argon2::Error as ArgonError;
use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as ReqwestMiddlewareError;
use sqlx::Error as SqlxError;
use std::fmt;
use std::fmt::Display;
use warp::{
    body::BodyDeserializeError, filters::cors::CorsForbidden, http::StatusCode, reject::Reject,
    Rejection, Reply,
};

use tracing::{event, instrument, Level};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    DatabaseQueryError(SqlxError),
    ReqwestAPIError(ReqwestError),
    ReqwestMiddlewareAPIError(ReqwestMiddlewareError),
    ClientError(APILayerError),
    ServerError(APILayerError),
    ArgonLibraryError(ArgonError),
    WrongPasswordError,
    CannotDecryptToken,
}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl Display for APILayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API error {}: {}", self.status, self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(ref err) => write!(f, "Parse error: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::DatabaseQueryError(err) => write!(f, "Query could not be executed {}", err),
            Error::ReqwestAPIError(err) => write!(f, "Reqwest error: {}", err),
            Error::ReqwestMiddlewareAPIError(err) => write!(f, "Reqwest middleware error: {}", err),
            Error::ClientError(err) => write!(f, "External Client error {}", err),
            Error::ServerError(err) => write!(f, "External Server error {}", err),
            Error::ArgonLibraryError(err) => write!(f, "Cannot verify password {}", err),
            Error::WrongPasswordError => write!(f, "WrongPassword"),
            Error::CannotDecryptToken => write!(f, "Cannot decrypt token"),
        }
    }
}

impl Reject for Error {}
impl Reject for APILayerError {}

const DUPLICATE_KEY: u32 = 23505;

#[instrument]
pub async fn return_error(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError(e)) = rejection.find() {
        event!(Level::ERROR, "Database query error");

        match e {
            SqlxError::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        "Account already exists".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Cannot update data".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                }
            }
            _ => Ok(warp::reply::with_status(
                "Cannot update data".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            )),
        }
    } else if let Some(Error::ReqwestAPIError(e)) = rejection.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ReqwestMiddlewareAPIError(e)) = rejection.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ClientError(e)) = rejection.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ServerError(e)) = rejection.find() {
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
