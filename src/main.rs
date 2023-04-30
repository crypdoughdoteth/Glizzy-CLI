use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{H160, U256},
    utils::parse_ether,
};
use slack_morphism::{
    prelude::{SlackApiChatPostMessageRequest, SlackClientHyperConnector},
    SlackApiToken, SlackApiTokenValue, SlackClient, SlackMessageContent,
};
use std::time::Duration;
use tokio::time::sleep;

async fn get_bal(addy: &Vec<String>) -> Result<Vec<U256>> {
    let mut res: Vec<U256> = Vec::new();
    let api_key = std::env::var("API_KEY")?;
    let provider = Provider::<Http>::try_from(api_key)?;
    for i in addy.into_iter() {
        res.push(provider.get_balance(i.parse::<H160>()?, None).await?)
    }
    Ok(res)
}

async fn monitor(channel: String, threshhold: U256, address: Vec<String>) -> Result<()> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let token_value: SlackApiTokenValue =
        slack_morphism::SlackApiTokenValue(std::env::var("SLACK_BOT_TOKEN")?);
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let session = client.open_session(&token);
    let mut j: usize = 0; 
    loop {
        match get_bal(&address).await {
            Ok(balances) => {
                for i in balances.iter() {
                    if *i <= threshhold {
                        let post_chat_req = SlackApiChatPostMessageRequest::new(
                            channel.to_owned().into(),
                            SlackMessageContent::new().with_text(format!(
                                "Your balance at {:?} is running low: {}!\n",
                                &address[j], i
                            )),
                        );
                        let post_chat_resp = session.chat_post_message(&post_chat_req).await?;
                        j += 1;
                        println!("{:?}\n", post_chat_resp);
                    } else {
                        println!("Balance is Sufficient at {:?}: {}\n", &address[j], i);
                    }
                }
                sleep(Duration::from_secs(900)).await;
            }
            Err(e) => return Err(e),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Address of the account you want to monitor
    #[arg(short, long)]
    address: Vec<String>,
    /// Chat room for slack bot to post in
    #[arg(short, long)]
    chat: Option<String>,
    /// Threshhold (in Ether) for when bot will notify chat
    #[arg(short, long)]
    threshhold: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let cli = Cli::parse();
    monitor(
        format!("#{}", cli.chat.unwrap_or("general".to_owned())),
        parse_ether(cli.threshhold.unwrap_or(parse_ether("300")?.to_string()))?,
        cli.address,
    )
    .await?;
    Ok(())
}
