#[tokio::main]
pub async fn start_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    match reqwest::get("http://127.0.0.1:8080/api/cats/").await {
        Ok(r) => {
            println!("{r:#?}");
            match r
                .json::<Vec<tufa_common::repositories_types::tufa_server::routes::cats::Cat>>()
                .await
            {
                Ok(vec_cats) => println!("ok2 {vec_cats:#?}"),
                Err(ee) => println!("err2{ee:#?}"),
            }
            println!("ok");
        }
        Err(e) => println!("err{e:#?}"),
    }
    let bot = teloxide::Bot::from_env();
    teloxide::commands_repl(bot, answer, {
        use teloxide::utils::command::BotCommands;
        Command::ty()
    })
    .await;
}

#[derive(teloxide::utils::command::BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "show bot source code info ")]
    GitInfo,
}

async fn answer(
    bot: teloxide::Bot,
    msg: teloxide::types::Message,
    cmd: Command,
) -> teloxide::requests::ResponseResult<()> {
    log::info!("answer");
    match cmd {
        Command::Help => {
            use teloxide::prelude::Requester;
            bot.send_message(
                msg.chat.id,
                {
                    use teloxide::utils::command::BotCommands;
                    Command::descriptions()
                }
                .to_string(),
            )
            .await?
        }
        Command::Username(username) => {
            use teloxide::prelude::Requester;
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            use teloxide::prelude::Requester;
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::GitInfo => {
            use teloxide::prelude::Requester;
            bot.send_message(msg.chat.id, {
                use tufa_common::common::git::get_git_commit_link::GetGitCommitLink;
                crate::global_variables::compile_time::git_info::GIT_INFO.get_git_commit_link()
            })
            .await?
        }
    };

    Ok(())
}
