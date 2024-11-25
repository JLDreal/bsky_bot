use bsky_sdk::BskyAgent;
use bsky_sdk::api;
use api::types::string::Datetime;
use std::io;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    print!("\x1B[2J\x1B[1;1H");
    let pw = env::var("BSKY_PW").unwrap();            
    let mail = env::var("BSKY_MAIL").unwrap();
    let agent = BskyAgent::builder().build().await?;
    let _session = agent.login(mail, pw).await?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    agent.create_record(api::app::bsky::feed::post::RecordData {
                            created_at: Datetime::now(),
                            embed: None,
                            entities: None,
                            facets: None,
                            labels: None,
                            langs: None,
                            reply: None,
                            tags: None,
                            text: input,
                        })
                        .await?;
    Ok(())
}
