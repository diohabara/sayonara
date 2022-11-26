use dotenv::dotenv;
use egg_mode::{
    tweet::{delete, user_timeline},
    user::{self},
};
use log::{info, warn};

use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    dotenv().ok();
    let consumer_key = env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY must be set");
    let consumer_secret =
        env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET must be set");
    let access_token = env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN must be set");
    let access_token_secret =
        env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET must be set");
    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: egg_mode::KeyPair::new(access_token, access_token_secret),
    };
    let account_name = env::var("TWITTER_ACCOUNT_NAME").expect("TWITTER_ACCOUNT_NAME must be set");
    info!("account_name: {}", account_name);
    let delete_page_size = 3200;
    let user_id = user::show(account_name, &token).await.unwrap().id;
    let timeline = user_timeline(user_id, true, true, &token).with_page_size(delete_page_size);
    let (_, feed) = timeline.start().await.unwrap();
    for tweet in &*feed {
        if chrono::Utc::now() - chrono::Duration::days(7) < tweet.created_at {
            continue;
        }
        info!("deleting...: id={} text={}", tweet.id, tweet.text);
        match delete(tweet.id, &token).await {
            Ok(_) => info!("deleted: id={} text={}", tweet.id, tweet.text),
            Err(e) => warn!(
                "failed to delete: id={} text={} error={}",
                tweet.id, tweet.text, e
            ),
        }
    }
    Ok(())
}
