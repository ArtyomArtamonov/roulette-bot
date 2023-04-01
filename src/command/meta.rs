use teloxide::prelude::*;

const HELP_TEXT: &str = "Available commands:
/help - shows this message
/start - welcoming message

/roll - rolls the dice
";

const WELCOME_TEXT: &str = "Hello! Welcome to my bot!";

pub async fn help_handler(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, HELP_TEXT)
        .await
        .log_on_error()
        .await;
}

pub async fn start_handler(bot: Bot, msg: Message) {
    bot.send_message(msg.chat.id, WELCOME_TEXT)
        .await
        .log_on_error()
        .await;
}
