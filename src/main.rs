use anyhow::{bail, Result};
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

async fn monitor(channel: String, mut threshhold: Vec<U256>, address: Vec<String>) -> Result<()> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let token_value: SlackApiTokenValue =
        slack_morphism::SlackApiTokenValue(std::env::var("SLACK_BOT_TOKEN")?);
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let session = client.open_session(&token);
    let mut j: usize = 0;

    if threshhold.len() == 1 {
        threshhold.resize(address.len(), threshhold[0]);
    } else if threshhold.len() != address.len() {
        bail!("Input length mismatch");
    }

    loop {
        match get_bal(&address).await {
            Ok(balances) => {
                for i in balances.iter() {
                    if *i <= threshhold[j] {
                        let post_chat_req = SlackApiChatPostMessageRequest::new(
                            channel.to_owned().into(),
                            SlackMessageContent::new().with_text(format!(
                                "Your balance at {:?} is running low: {}!\n",
                                &address[j], i
                            )),
                        );
                        let post_chat_resp = session.chat_post_message(&post_chat_req).await?;
                        println!("{:?}\n", post_chat_resp);
                    } else {
                        println!("Balance is Sufficient at {:?}: {}\n", &address[j], i);
                    }
                    j += 1;
                }
                sleep(Duration::from_secs(900)).await;
            }
            Err(e) => return Err(e),
        }
    }
}

const DEFAULT_THRESHHOLD: &str = "300";

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
    threshhold: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let cli = Cli::parse();

    let val: Vec<U256> = if let Some(x) = cli.threshhold {
        x.iter()
            .map(move |x: &String| parse_ether(x).unwrap())
            .collect()
    } else {
        vec![parse_ether(DEFAULT_THRESHHOLD.to_string())?]
    };
    monitor(
        format!("#{}", cli.chat.unwrap_or("general".to_owned())),
        val,
        cli.address,
    )
    .await?;
    Ok(())
}
