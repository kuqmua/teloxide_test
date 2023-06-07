#[tokio::main]
pub async fn start_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_get(
        std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetQueryParameters {
            limit: None,
            name: None,
            color: None,
        },
    )
    .await
    {
        Ok(vec_cat) => println!("try_get_result\n{vec_cat:#?}"),
        Err(e) => {
            println!("try_get_result error\n{e}");
            println!("try_get_result error\n{e:#?}")
        }
    }
    println!("--------------------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_get_by_id(
        std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetByIdPathParameters {
            id: 7,
        },
    )
    .await
    {
        Ok(cat) => println!("try_get_by_id\n{cat:#?}"),
        Err(e) => println!("try_get_by_id error\n{e:#?}"),
    }
    println!("--------------------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_post(
        std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPost {
            name: String::from("testcatnamepost"),
            color: String::from("testcatcolorpost"),
        },
    )
    .await
    {
        Ok(_) => println!("try_post"),
        Err(e) => println!("try_post error\n{e:#?}"),
    }
    println!("--------------------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_put(
        std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat {
            id: 65,
            name: String::from("testcatnameput"),
            color: String::from("testcatcolorput"),
        },
    )
    .await
    {
        Ok(_) => println!("try_put"),
        Err(e) => println!("try_put error\n{e:#?}"),
    }
    println!("--------------------------------");
    //todo handle please use put
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_patch(
        std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch::IdName {
            id: 65,
            name: String::from("testcatnamepatched"),
        },
    )
    .await
    {
        Ok(_) => println!("try_patch"),
        Err(e) => println!("try_patch error\n{e:#?}"),
    }
    println!("--------------------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_delete(
        std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteQueryParameters {
            name: Some(String::from("testcatnamepost")),
            color: None,
        },
    )
    .await
    {
        Ok(_) => println!("try_delete"),
        Err(e) => println!("try_delete error\n{e:#?}"),
    }
    println!("--------------------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_delete_by_id(
        std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteByIdPathParameters {
            id: 45,
        },
    )
    .await
    {
        Ok(_) => println!("try_delete_by_id"),
        Err(e) => println!("try_delete_by_id error\n{e:#?}"),
    }
    //
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
