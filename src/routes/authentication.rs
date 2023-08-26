use std::future;

use argon2::{self, Config};
use chrono::{Duration, Utc};
use rand::Rng;
use warp::{Filter, Rejection, Reply};

use crate::{
    store::Store,
    types::account::{Account, AccountId, Session},
};

pub async fn register(store: Store, account: Account) -> Result<impl Reply, Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());

    let account = Account {
        password: hashed_password,
        ..account
    };

    match store.add_account(account).await {
        Ok(account) => Ok(warp::reply::json(&account)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn login(store: Store, login: Account) -> Result<impl Reply, Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("Account id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(
                        handle_error::Error::WrongPasswordError,
                    ))
                }
            }
            Err(e) => Err(warp::reject::custom(
                handle_error::Error::ArgonLibraryError(e),
            )),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

// TODO: replace with env var
const PASETO_KEY: [u8; 32] = [0u8; 32];

fn issue_token(account_id: AccountId) -> String {
    let current_data_time = Utc::now();
    let dt = current_data_time + Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&PASETO_KEY)
        .set_expiration(&dt)
        .set_not_before(&current_data_time)
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token.")
}

fn verify_token(token: String) -> Result<Session, handle_error::Error> {
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        &PASETO_KEY,
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| handle_error::Error::CannotDecryptToken)?;

    serde_json::from_value::<Session>(token).map_err(|_| handle_error::Error::CannotDecryptToken)
}

pub fn auth() -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(_) => return future::ready(Err(warp::reject::reject())),
        };

        future::ready(Ok(token))
    })
}
