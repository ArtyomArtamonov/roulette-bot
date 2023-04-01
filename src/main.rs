use command::meta::{help_handler, start_handler};
use command::roulette::roll_handler;
use command::wrap_logging;
use dotenv;
use teloxide::prelude::*;

mod command;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("failed loading .env file");
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, repl_func).await;
}

async fn repl_func(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg
        .text()
        .unwrap_or("/help ")
        .split(" ")
        .next()
        .unwrap_or("/help")
        .to_lowercase()
        .as_str()
    {
        "/roll" => roll_handler(bot, msg).await,
        "/start" => start_handler(bot, msg).await,
        "/help" | _ => help_handler(bot, msg).await,
    };

    Ok(())
}
