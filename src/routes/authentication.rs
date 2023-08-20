use warp::{Rejection, Reply};

use crate::{store::Store, types::account::Account};

pub async fn register(store: Store, account: Account) -> Result<impl Reply, Rejection> {
    match store.add_account(account).await {
        Ok(account) => Ok(warp::reply::json(&account)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
