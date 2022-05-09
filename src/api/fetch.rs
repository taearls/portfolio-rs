// use log::error;
// use serde::de::DeserializeOwned;
use wasm_bindgen_futures::spawn_local;
// use yew::UseStateHandle;
use reqwest::Client;
use weblog::{console_error, console_log};

use lazy_static::lazy_static;

lazy_static! {
    static ref REQWEST_CLIENT: Client = reqwest::Client::new();
}

pub fn post(url: String, body: String) {
    spawn_local(async move {
        match post_inner(&url, body).await {
            Ok(value) => console_log!("value is: ", value),
            Err(e) => {
                console_error!("failed to fetch: ", e.to_string());
            }
        }
    });
}

async fn post_inner(url: &str, body: String) -> Result<String, std::io::Error> {
    let response = REQWEST_CLIENT
        .post(url)
        .body(body)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let body = response.text().await.unwrap();
    let list = serde_json::from_str(&body)?;

    Ok(list)
}
