use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccountId(i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Account {
    id: AccountId,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NewAccount {
    email: String,
    password: String,
}
