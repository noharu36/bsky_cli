use dialoguer::Input;
use reqwest::{Client, Error};
use serde::Serialize;

#[derive(Serialize)]
struct PostData {
    repo: String,
    collection: String,
    rkey: String
}


pub async fn delete_post(handle: &str, access_jwt: &str) -> Result<(), Error> {
    println!("Tell me the post url.");
    let record_key = Input::<String>::new().interact().ok().unwrap();

    let url = "https://bsky.social/xrpc/com.atproto.repo.deleteRecord";

    let post_data = PostData {
        repo: handle.to_string(),
        collection: "app.bsky.feed.post".to_string(),
        rkey: record_key,
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

    if res.status().is_success() {
        println!("Delete success!")
    } else {
        println!("Oh, delete failed. Please try again.")
    }

    Ok(())
}
