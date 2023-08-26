use argon2::{self, Config};
use paseto::v2::local_paseto;
use rand::Rng;
use warp::{Rejection, Reply};

use crate::{
    store::Store,
    types::account::{Account, AccountId},
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

const PASETO_KEY: [u8; 32] = [0u8; 32];

fn issue_token(account_id: AccountId) -> String {
    let state = serde_json::to_string(&account_id).expect("Failed to serialize");

    local_paseto(&state, None, &PASETO_KEY).expect("Failed to create token")
}
