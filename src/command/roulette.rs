use teloxide::prelude::*;

pub async fn roll_handler(bot: Bot, msg: Message) {
    bot.send_dice(msg.chat.id).await.log_on_error().await;
}
