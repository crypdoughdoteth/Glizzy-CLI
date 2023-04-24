use anyhow::Result;
use dotenv::dotenv;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use slack_morphism::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

async fn get_bal() -> Result<u64> {
    let api_key = std::env::var("API_KEY")?;

    let provider = Provider::<Http>::try_from(api_key)?;
    let addy = "0x3a7c5f2c6C3F38A632007FE4f7e6b8676Fdc1F89".parse::<Address>()?;

    Ok(provider.get_balance(addy, None).await?.as_u64())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let threshhold: u64 = 1;
    loop {
        match get_bal().await {
            Ok(x) => {
                if x <= threshhold {
                    let client = SlackClient::new(SlackClientHyperConnector::new());
                    let token_value: SlackApiTokenValue = slack_morphism::SlackApiTokenValue(
                        std::env::var("SLACK_BOT_TOKEN").unwrap(),
                    );
                    let token: SlackApiToken = SlackApiToken::new(token_value);
                    let session = client.open_session(&token);
                    let post_chat_req = SlackApiChatPostMessageRequest::new(
                        "#general".into(),
                        SlackMessageContent::new().with_text("Balance is running low".into()),
                    );
                    let post_chat_resp = session.chat_post_message(&post_chat_req).await?;
                    println!("{:?}", post_chat_resp);
                } else {
                    println!("Balance is Sufficient");
                }
                sleep(Duration::from_secs(60)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
