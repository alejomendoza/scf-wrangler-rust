use std::env;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Error, Client};
use serde_json::Value;


// const PROJECT_ID: &str = "145hugiz";
// const DATASET: &str = "public";


fn build_client() -> Result<Client, Error> {
    let mut headers = HeaderMap::new();
    let token = env::var("SANITY_API_KEY").expect("$SANITY_API_KEY is not set");
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .build()?;


    Ok(client)
}

pub(crate) async fn fetch_users() -> Result<Value, Error> {

    let url = format!("https://jsonplaceholder.typicode.com/posts/1");
    // let url = format!("https://\'{}\'.api.sanity.io/v2021-06-07/query/\'{}\'?query=*[_type == \'user\']",PROJECT_ID, DATASET);
    
    let client = build_client().unwrap();

    let response = client.get(&url).send().await?
        .json::<Value>()
        .await?;
    

    Ok(response)
}