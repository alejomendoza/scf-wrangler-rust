use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, HeaderValue};
use reqwest::{Error};
use serde_json::Value;

const PROJECT_ID: &str = "145hugiz";
const DATASET: &str = "public";

fn construct_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let token = format!("Bearer {}", token);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

pub(crate) async fn fetch_users(token: &str) -> Result<Value, Error> {

    let url = format!("https://\'{}\'.api.sanity.io/v2021-10-21/data/query/\'{}\'?query=*[_type == \'user\']",PROJECT_ID, DATASET);

    
    let response = reqwest::Client::new()
        .get(url)
        .headers(construct_headers(token)).send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}