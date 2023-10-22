#[tokio::main]
pub async fn start_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let id  = 14;
    let api_location = std::string::String::from("http://127.0.0.1:8080/api");
    println!("--------------try_read_many_with_body-----------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_read_many_with_body(
        &api_location,
        //todo - builder pattern?
        tufa_common::repositories_types::tufa_server::routes::api::cats::ReadManyWithBodyParameters{ 
            payload: tufa_common::repositories_types::tufa_server::routes::api::cats::ReadManyWithBodyPayload { 
                select: tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumnSelect::Id,
                id: Some(vec![tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap()]),
                name: Some(vec![tufa_common::server::postgres::regex_filter::RegexFilter {
                    regex: std::string::String::from("test"),
                    conjuctive_operator: tufa_common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                }]),//or and support
                color: Some(vec![tufa_common::server::postgres::regex_filter::RegexFilter {
                    regex: std::string::String::from("test"),
                    conjuctive_operator: tufa_common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                }]),
                order_by: tufa_common::server::postgres::order_by::OrderBy {
                    column: tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumn::Name,
                    order: Some(tufa_common::server::postgres::order::Order::Desc),
                },
                limit: tufa_common::server::postgres::postgres_bigint::PostgresBigint(10),
                offset: tufa_common::server::postgres::postgres_bigint::PostgresBigint(1),
            } 
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
    println!("--------------try_read_many-----------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_read_many(
        &api_location,
        //todo - builder pattern?
        tufa_common::repositories_types::tufa_server::routes::api::cats::ReadManyParameters { 
            query: tufa_common::repositories_types::tufa_server::routes::api::cats::ReadManyQuery {
                select: Some(
                    tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumnSelect::Id,
                ),
                id: 
                Some(tufa_common::server::postgres::bigserial_ids::BigserialIds(
                    vec![tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap()],
                )),
                name: Some(tufa_common::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma(vec![std::string::String::from("onename"), std::string::String::from("twoname")])),
                color: None,
                order_by: Some(tufa_common::repositories_types::tufa_server::routes::api::cats::CatOrderByWrapper(
                    tufa_common::server::postgres::order_by::OrderBy {
                        column: tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumn::Id,
                        order: Some(tufa_common::server::postgres::order::Order::Asc)
                    }
                )),
                limit: tufa_common::server::postgres::postgres_bigint::PostgresBigint(10),
                offset: Some(tufa_common::server::postgres::postgres_bigint::PostgresBigint(1)),
            }
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
    println!("--------------try_read_one-----------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_read_one(
        &api_location,
        tufa_common::repositories_types::tufa_server::routes::api::cats::ReadOneParameters { 
            path: tufa_common::repositories_types::tufa_server::routes::api::cats::ReadOnePath {
                id: tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap(),
            }, 
            query: tufa_common::repositories_types::tufa_server::routes::api::cats::ReadOneQuery {
                select: Some(tufa_common::repositories_types::tufa_server::routes::api::cats::CatColumnSelect::IdColor)    
            }
        },
    )
    .await
    {
        Ok(cat) => println!("{cat:#?}"),
        Err(e) => {
            println!("{e}")
        }
    }
    println!("--------------try_create_one-----------------");//todo add try_create_many
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_create_one(
        &api_location,
        tufa_common::repositories_types::tufa_server::routes::api::cats::CreateOneParameters { 
            payload: tufa_common::repositories_types::tufa_server::routes::api::cats::CreateOnePayload {
                name: String::from("testcatnamepost"),
                color: String::from("testcatcolorpost"),
            }
        },
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("{e}")
        }
    }
    // //todo handle please use put
    println!("--------------try_update_one------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_update_one(
        &api_location,
        tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateOneParameters { 
            path: tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateOnePath {
                id: tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap(),
            }, 
            payload: tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateOnePayload { 
                name: Some(std::string::String::from("name")), 
                color: Some(std::string::String::from("color")), 
            }
        }
    )
    .await
    {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
    println!("--------------try_delete_many------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_delete_many(
        &api_location,
        tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteManyParameters { 
            query: tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteManyQuery {
                id: Some(tufa_common::server::postgres::bigserial_ids::BigserialIds(
                    vec![tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap()],
                )),
                name: Some(tufa_common::server::routes::helpers::strings_deserialized_from_string_splitted_by_comma::StringsDeserializedFromStringSplittedByComma(vec![std::string::String::from("onename"), std::string::String::from("twoname")])),
                color: None,
            } 
        },
    )
    .await
    {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
    println!("--------------try_delete_one------------------");
    match tufa_common::repositories_types::tufa_server::routes::api::cats::try_delete_one(
        &api_location,
        tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteOneParameters { 
            path: tufa_common::repositories_types::tufa_server::routes::api::cats::DeleteOnePath {
                id: tufa_common::server::postgres::bigserial::Bigserial::try_from(id).unwrap(),
            }
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
