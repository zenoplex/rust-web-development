use argon2::{self, Config};
use rand::Rng;
use warp::{Rejection, Reply};

use crate::{store::Store, types::account::Account};

pub async fn register(store: Store, account: Account) -> Result<impl Reply, Rejection> {
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
