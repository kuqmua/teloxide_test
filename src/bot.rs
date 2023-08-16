#[tokio::main]
pub async fn start_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    println!("---------------try_get_result-----------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::get::try_get(
        &std::string::String::from("http://127.0.0.1:8080"),
        //todo - builder pattern?
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetQueryParameters {
            limit: 10,
            id: Some(tufa_common::server::postgres::bigserial_ids::BigserialIds(
                vec![tufa_common::server::postgres::bigserial::Bigserial::try_from(65).unwrap()],
            )),
            name: None,
            color: None,
            select: Some(
                tufa_common::repositories_types::tufa_server::routes::api::cats::GetSelect::Id,
            ),
        },
    )
    .await
    {
        Ok(vec_cat_options) => {
            let vec_cat_options_len = vec_cat_options.len();
            println!("{vec_cat_options:#?}");
            let vec_cat_id: Vec<
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatId,
            > = vec_cat_options
                .into_iter()
                .filter_map(|value| match value.id {
                    Some(id) => Some(
                        tufa_common::repositories_types::tufa_server::routes::api::cats::CatId {
                            id,
                        },
                    ),
                    None => None,
                })
                .collect();
            let vec_cat_id_len = vec_cat_id.len();
            println!(
                "vec_cat_options_len == vec_cat_id_len {}",
                vec_cat_options_len == vec_cat_id_len
            );
        }
        Err(e) => {
            println!("{e}");
        }
    }
    // println!("---------------try_get_by_id-----------------");
    // match tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::try_get_by_id(
    //     &std::string::String::from("http://127.0.0.1:8080"),
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::GetByIdPathParameters {
    //         id: 65,
    //     },
    // )
    // .await
    // {
    //     Ok(cat) => println!("{cat:#?}"),
    //     Err(e) => {
    //         println!("{e}")
    //     }
    // }
    // println!("---------------try_post-----------------");
    // match tufa_common::repositories_types::tufa_server::routes::api::cats::post::try_post(
    //     &std::string::String::from("http://127.0.0.1:8080"),
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPost {
    //         name: String::from("testcatnamepost"),
    //         color: String::from("testcatcolorpost"),
    //     },
    // )
    // .await
    // {
    //     Ok(_) => (),
    //     Err(e) => {
    //         println!("{e:#?}")
    //     }
    // }
    // println!("--------------try_put------------------");
    // match tufa_common::repositories_types::tufa_server::routes::api::cats::put::try_put(
    //     &std::string::String::from("http://127.0.0.1:8080"),
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::Cat {
    //         id: 65,
    //         name: String::from("testcatnameput"),
    //         color: String::from("testcatcolorput"),
    //     },
    // )
    // .await
    // {
    //     Ok(_) => (),
    //     Err(e) => println!("{e}"),
    // }
    // // //todo handle please use put
    // println!("--------------try_patch------------------");
    // match tufa_common::repositories_types::tufa_server::routes::api::cats::patch::try_patch(
    //     &std::string::String::from("http://127.0.0.1:8080"),
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::CatToPatch::IdName {
    //         id: 65,
    //         name: String::from("testcatnamepatched"),
    //     },
    // )
    // .await
    // {
    //     Ok(_) => (),
    //     Err(e) => println!("{e}"),
    // }
    // println!("--------------try_delete------------------");
    // match tufa_common::repositories_types::tufa_server::routes::api::cats::delete::try_delete(
    //     &std::string::String::from("http://127.0.0.1:8080"),
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteQueryParameters {
    //         name: Some(String::from("testcatnamepost")),
    //         color: None,
    //     },
    // )
    // .await
    // {
    //     Ok(_) => (),
    //     Err(e) => println!("{e}"),
    // }
    // println!("--------------try_delete_by_id------------------");
    // match tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::try_delete_by_id(
    //     &std::string::String::from("http://127.0.0.1:8080"),
    //     tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteByIdPathParameters {
    //         id: 65,
    //     },
    // )
    // .await
    // {
    //     Ok(_) => (),
    //     Err(e) => println!("{e}"),
    // }

    // let bot = teloxide::Bot::from_env();
    // teloxide::commands_repl(bot, answer, {
    //     use teloxide::utils::command::BotCommands;
    //     Command::ty()
    // })
    // .await;
}

// #[derive(teloxide::utils::command::BotCommands, Clone)]
// #[command(
//     rename_rule = "lowercase",
//     description = "These commands are supported:"
// )]
// enum Command {
//     #[command(description = "display this text.")]
//     Help,
//     #[command(description = "handle a username.")]
//     Username(String),
//     #[command(description = "handle a username and an age.", parse_with = "split")]
//     UsernameAndAge { username: String, age: u8 },
//     #[command(description = "show bot source code info ")]
//     GitInfo,
// }

// async fn answer(
//     bot: teloxide::Bot,
//     msg: teloxide::types::Message,
//     cmd: Command,
// ) -> teloxide::requests::ResponseResult<()> {
//     log::info!("answer");
//     match cmd {
//         Command::Help => {
//             use teloxide::prelude::Requester;
//             bot.send_message(
//                 msg.chat.id,
//                 {
//                     use teloxide::utils::command::BotCommands;
//                     Command::descriptions()
//                 }
//                 .to_string(),
//             )
//             .await?
//         }
//         Command::Username(username) => {
//             use teloxide::prelude::Requester;
//             bot.send_message(msg.chat.id, format!("Your username is @{username}."))
//                 .await?
//         }
//         Command::UsernameAndAge { username, age } => {
//             use teloxide::prelude::Requester;
//             bot.send_message(
//                 msg.chat.id,
//                 format!("Your username is @{username} and age is {age}."),
//             )
//             .await?
//         }
//         Command::GitInfo => {
//             use teloxide::prelude::Requester;
//             bot.send_message(msg.chat.id, {
//                 use tufa_common::common::git::get_git_commit_link::GetGitCommitLink;
//                 crate::global_variables::compile_time::git_info::GIT_INFO.get_git_commit_link()
//             })
//             .await?
//         }
//     };

//     Ok(())
// }
