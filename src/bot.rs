#[tokio::main]
pub async fn start_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let id  = 9;
    println!("--------------try_read_post-----------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::read_post::try_read_post(
        &std::string::String::from("http://127.0.0.1:8080"),
        //todo - builder pattern?
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToReadPost { 
            select: tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumnSelectJson::Id,
            ids: Some(vec![tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap()]),
            name_regex: Some(vec![tufa_common::server::postgres::regex_filter::RegexFilter {
                regex: std::string::String::from("test"),
                conjuctive_operator: tufa_common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
            }]),//or and support
            color_regex: Some(vec![tufa_common::server::postgres::regex_filter::RegexFilter {
                regex: std::string::String::from("test"),
                conjuctive_operator: tufa_common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
            }]),
            order_by: tufa_common::repositories_types::tufa_server::routes::api::cats::CatOrderBy {
                column: tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumn::Name,
                order: Some(tufa_common::server::postgres::order::Order::Desc),
            },
            limit: tufa_common::server::postgres::postgres_number::PostgresNumber(10),
            offset: tufa_common::server::postgres::postgres_number::PostgresNumber(1),
        },
    )
    .await
    {
        Ok(vec_cat_options) => {
            // let vec_cat_options_len = vec_cat_options.len();
            // println!("{vec_cat_options:#?}");
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
            // let vec_cat_id_len = vec_cat_id.len();
            println!("{vec_cat_id:#?}");
            // println!(
            //     "vec_cat_options_len == vec_cat_id_len {}",
            //     vec_cat_options_len == vec_cat_id_len
            // );
        }
        Err(e) => {
            println!("{e}");
        }
    }
    println!("--------------try_read-----------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::read::try_read(
        &std::string::String::from("http://127.0.0.1:8080"),
        //todo - builder pattern?
        tufa_common::repositories_types::tufa_server::routes::api::cats::ReadQueryParameters {
            select: Some(
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumnSelectUrl::Id,
            ),
            id: None,
            // Some(tufa_common::server::postgres::bigserial_ids::BigserialIds(
            //     vec![tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap()],
            // ))
            name: None,
            color: None,
            order_by: Some(tufa_common::repositories_types::tufa_server::routes::api::cats::CatOrderByWrapper(
                tufa_common::repositories_types::tufa_server::routes::api::cats::CatOrderBy {
                    column: tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumn::Id,
                    order: Some(tufa_common::server::postgres::order::Order::Asc)
                }
            )),
            limit: tufa_common::server::postgres::postgres_number::PostgresNumber(10),
            offset: Some(tufa_common::server::postgres::postgres_number::PostgresNumber(1)),
        },
    )
    .await
    {
        Ok(vec_cat_options) => {
            // let vec_cat_options_len = vec_cat_options.len();
            // println!("{vec_cat_options:#?}");
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
            // let vec_cat_id_len = vec_cat_id.len();
            println!("{vec_cat_id:#?}");
            // println!(
            //     "vec_cat_options_len == vec_cat_id_len {}",
            //     vec_cat_options_len == vec_cat_id_len
            // );
        }
        Err(e) => {
            println!("{e}");
        }
    }
    println!("--------------try_read_by_id-----------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::read_by_id::try_read_by_id(
        &std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::ReadByIdPathParameters {
            id: tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap(),
        },
        tufa_common::repositories_types::tufa_server::routes::api::cats::ReadByIdQueryParameters {
            select: Some(tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumnSelectUrl::IdColor)    
        }
    )
    .await
    {
        Ok(cat) => println!("{cat:#?}"),
        Err(e) => {
            println!("{e}")
        }
    }
    println!("--------------try_create-----------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::create::try_create(
        &std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToCreate {
            name: String::from("testcatnamepost"),
            color: String::from("testcatcolorpost"),
        },
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("{e:#?}")
        }
    }
    println!("--------------try_create_or_update_by_id------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::create_or_update_by_id::try_create_or_update_by_id(
        &std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::CreateOrUpdateByIdPathParameters { 
            id: tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap(),
        },
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToCreateOrUpdateById { 
            name: std::string::String::from("testcatnameput"), 
            color: std::string::String::from("testcatcolorput") 
        },
    )
    .await
    {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
    // //todo handle please use put
    println!("--------------try_update_by_id------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::update_by_id::try_update_by_id(
        &std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateByIdPathParameters {
            id: tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap(),
        },
        tufa_common::repositories_types::tufa_server::routes::api::cats::CatToUpdateById::NameColor { 
            name: std::string::String::from("name"), 
            color: std::string::String::from("color"), 
        }
    )
    .await
    {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
    println!("--------------try_delete------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::delete::try_delete(
        &std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteQueryParameters {
            name: Some(String::from("testcatnamepost")),
            color: None,
        },
    )
    .await
    {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
    println!("--------------try_delete_by_id------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::delete_by_id::try_delete_by_id(
        &std::string::String::from("http://127.0.0.1:8080"),
        tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteByIdPathParameters {
            id: tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap(),
        },
    )
    .await
    {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }

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
