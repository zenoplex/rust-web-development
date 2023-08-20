use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccountId(i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Account {
    id: Option<AccountId>,
    email: String,
    password: String,
}
