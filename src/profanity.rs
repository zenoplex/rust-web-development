use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

pub async fn check_profanity(content: String) -> Result<String, handle_error::Error> {
    let client = Client::new();
    let res = client
        .post(env::var("BAD_WORDS_API_ENDPOINT").expect("BAD_WORDS_API_ENDPOINT not set"))
        .header(
            "apiKey",
            env::var("BAD_WORDS_API_KEY").expect("BAD_WORDS_API_KEY not set"),
        )
        .body(content)
        .send()
        .await
        .map_err(handle_error::Error::ExternalAPIError)?;

    if !res.status().is_success() {
        if res.status().is_client_error() {
            let err = transform_error(res).await;
            return Err(handle_error::Error::ClientError(err).into());
        } else {
            let err = transform_error(res).await;
            return Err(handle_error::Error::ServerError(err).into());
        }
    }

    match res.json::<BadWordsResponse>().await {
        Ok(res) => Ok(res.censored_content),
        Err(e) => Err(handle_error::Error::ExternalAPIError(e)),
    }
}

async fn transform_error(res: reqwest::Response) -> handle_error::APILayerError {
    handle_error::APILayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}
