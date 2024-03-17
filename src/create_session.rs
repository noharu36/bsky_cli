use dialoguer::{Input, Password};
use reqwest::{Client, Error};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct JsonData {
    identifier: String,
    password: String,
}

#[derive(Debug)]
pub struct ResData {
    pub did: String,
    pub name: String,
    pub access_jwt: String,
    pub refresh_jwt: String
}

pub async fn create_session() -> Result<ResData, Error> {
    println!("You need login. Prease enter email and password.");
    let email = Input::<String>::new()
        .with_prompt("email").interact().ok().unwrap();
    let pass = Password::new()
        .with_prompt("password").with_confirmation("Once more", "Uncorrect")
        .interact().ok().unwrap();

    //println!("email: {}, pass: {}", email, pass);

    let url = "https://bsky.social/xrpc/com.atproto.server.createSession";
    let json_data = JsonData {identifier: email, password: pass};

    let client = Client::new();

    let res = client.post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json_data).unwrap())
        .send()
        .await?;

    let response_body: Value = serde_json::from_str(&res.text().await?).ok().unwrap();

    //println!("{:?}", response_body);

    let res_data = ResData {
        did: response_body["did"].as_str().unwrap().to_string(),
        name: response_body["handle"].as_str().unwrap().to_string(),
        access_jwt: response_body["accessJwt"].as_str().unwrap().to_string(),
        refresh_jwt: response_body["refreshJwt"].as_str().unwrap().to_string()
    };

    Ok(res_data)
}
