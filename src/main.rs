use bsky_cli::create_session::create_session;
use bsky_cli::create_post::create_post;
use dialoguer::Select;

#[tokio::main]
async fn main() {
    println!("Hello! I am bsky_cli!");
    let Ok(login_info) = create_session().await else {
        println!("Something wrong. Please try again.");
        return;
    };

    if login_info.did == "null".to_string() {
        println!("Email address or password is incorrect. Please try again.")
    } else {
        //println!("{:?}", login_info);
        println!("Welcome {}, Authentication succeeded!", login_info.name);
        'main: loop {
            let choice = vec!["Post", "Delete", "Get Profile", "Get Follower", "Get Follows", "Exit"];

            let selection = Select::new().with_prompt("What do you want to do?")
                .items(&choice).interact().ok().unwrap();

            match selection {
                0_usize => create_post(&login_info.name, &login_info.access_jwt).await.ok().unwrap(),
                1_usize => println!("delete"),
                2_usize => println!("profile"),
                3_usize => println!("follower"),
                4_usize => println!("follows"),
                5_usize => {
                    println!("bye👋");
                    break 'main
                },
                _ => println!("undefined selection"),
            }
        }
    }


}
