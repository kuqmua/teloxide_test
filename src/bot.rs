use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::helpers::git_info::GIT_INFO;

#[tokio::main]
pub async fn start_bot() {
    pretty_env_logger::init();
    log::info!("Starting simple_commands_bot...");
    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: String },
    #[command(description = "show bot source code info ")]
    GitInfo,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(message.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                message.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::GitInfo => {
            bot.send_message(message.chat.id, GIT_INFO.get_commit_link())
                .await?
        }
    };

    Ok(())
}
