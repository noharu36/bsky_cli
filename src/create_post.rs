use dialoguer::Input;
use reqwest::{Client, Error};
use serde::Serialize;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Serialize)]
struct Record {
    text: String,
    createdAt: String
}

#[derive(Serialize)]
struct PostData {
    repo: String,
    collection: String,
    record: Record
}

pub async fn create_post(handle: &str, access_jwt: &str) -> Result<(), Error> {
    println!("Tell me what you would like to post.");
    let content = Input::<String>::new().interact().ok().unwrap();

    let url = "https://bsky.social/xrpc/com.atproto.repo.createRecord";
    let now: DateTime<Utc> = Utc::now();
    let post_data = PostData {
        repo: handle.to_string(),
        collection: "app.bsky.feed.post".to_string(),
        record: Record {
            text: content,
            createdAt: format!("{}", now.to_rfc3339())
        }
    };

    let client = Client::new();

    let mut headers = reqwest::header::HeaderMap::new();

    let token = format!("Bearer {}", access_jwt);

    headers.insert("AUTHORIZATION", reqwest::header::HeaderValue::from_str(&token).unwrap());

    let res = client.post(url)
        .header("Content-Type", "application/json")
        .headers(headers)
        .body(serde_json::to_string(&post_data).unwrap())
        .send()
        .await?;

    let posted: Value = serde_json::from_str(&res.text().await?).ok().unwrap();
    
    println!("Success!\nurl {}", posted["uri"].as_str().unwrap().to_string());

    Ok(())
}
