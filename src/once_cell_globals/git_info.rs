use crate::constants::PROJECT_NAME;
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
