use reqwest::{Client, Error};
use serde_json::Value;

#[derive(Debug)]
pub struct Profile {
    pub did: String,
    pub handle: String,
    pub display_name: String,
    pub description: String,
    pub indexed: String,
    pub follower: u64,
    pub follows: u64,
    pub posts_count: u64,
}


pub async fn get_profile(handle: &str, access_jwt: &str) -> Result<(), Error> {
    let url = format!(
        "https://bsky.social/xrpc/app.bsky.actor.getProfile?actor={}",
        handle);

    let client = Client::new();

    let mut headers = reqwest::header::HeaderMap::new();

    let token = format!("Bearer {}", access_jwt);

    headers.insert("AUTHORIZATION", reqwest::header::HeaderValue::from_str(&token).unwrap());

    let res = client.get(url)
        .header("Content-Type", "application/json")
        .headers(headers)
        .send()
        .await?;

    let response_body: Value = serde_json::from_str(&res.text().await?).ok().unwrap();

    let profile = Profile {
        did: response_body["did"].as_str().unwrap().to_string(),
        handle: response_body["handle"].as_str().unwrap().to_string(),
        display_name: response_body["displayName"].as_str().unwrap().to_string(),
        description: response_body["description"].as_str().unwrap().to_string(),
        indexed: response_body["indexedAt"].as_str().unwrap().to_string(),
        follower: response_body["followersCount"].as_u64().unwrap(),
        follows: response_body["followsCount"].as_u64().unwrap(),
        posts_count: response_body["postsCount"].as_u64().unwrap(),
    };

    println!(
        "Your profile
            did: {}
            handle: {}
            display name: {}
            follower: {}  follows: {}  {}posts
            description: {}
        ", profile.did, profile.handle, profile.display_name, profile.follower, profile.follows, profile.posts_count, profile.description
        );

    Ok(())

}
