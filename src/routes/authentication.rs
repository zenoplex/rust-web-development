use argon2::{self, Config};
use rand::Rng;
use warp::{Rejection, Reply};

use crate::{store::Store, types::account::Account};

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
        Ok(account) => {
            todo!("implement password verification");
            Ok(warp::reply::json(&account))
        }
        Err(e) => Err(warp::reject::custom(e))
    }
}
