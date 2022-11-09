use crate::constants::PROJECT_NAME;
use compile_time_git_info::CompileTimeGitInfo;
use once_cell::sync::Lazy;
use tufa_common::common::git::git_info::GitInformation;

pub static GIT_INFO: Lazy<GitInformation> =
    Lazy::new(|| GitInformation::get_git_commit_info("../", PROJECT_NAME));

// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

#[derive(Debug, CompileTimeGitInfo)]
pub struct Kekw<'a> {
    pub commit_id: &'a str,
}

// impl Kekw<'_> {
//     pub const fn get_git_commit_info<'a>(repo_git_path: &'a str, repo_name: &'a str) -> Kekw<'a> {
//         // use crate::constants::GIT_PATH_FROM_SUBMODULE;
//         use std::fs::File;
//         use std::io::prelude::*;
//         use std::io::BufReader;
//         use std::path::Path;
//         let git_path_from_submodule = "../";
//         // println!("helolo from kekwe");
//         let path: String;
//         let git_folder_name = ".git";
//         let first_guess = format!("../{}{}/", repo_git_path, git_folder_name); //maybe here some error?
//         if Path::new(&first_guess).is_dir() {
//             path = first_guess; //for docker image or run not as tufa_project repo, as git clone tufa_server
//         } else if Path::new(&format!(
//             "../{}{}/",
//             git_path_from_submodule, git_folder_name
//         ))
//         .is_dir()
//         {
//             path = format!(
//                 "../{}{}/modules/src/{}/",
//                 git_path_from_submodule, git_folder_name, repo_name
//             );
//         } else {
//             panic!("error: no .git folder inside current and parent dir(this message should be displayed only on compile time)")
//         }
//         let full_path = &format!("{}{}", path, "logs/HEAD");
//         let file = File::open(Path::new(full_path))
//             .unwrap_or_else(|e| panic!("cannot open HEAD file, error: \"{}\"", e));
//         let mut buf_reader = BufReader::new(file);
//         let mut git_logs_head_content = String::new();
//         buf_reader
//             .read_to_string(&mut git_logs_head_content)
//             .unwrap_or_else(|e| panic!("cannot read to string from HEAD file, error: \"{}\"", e));
//         Kekw { commit_id: "" }
//     }
// }

// pub const SOMETHING: Kekw = Kekw::get_git_commit_info("../", PROJECT_NAME);
