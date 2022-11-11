use compile_time_git_info::CompileTimeGitInfoTufaTelegramBot;
use tufa_common::common::git::git_info::GitInformation;

#[derive(Debug, CompileTimeGitInfoTufaTelegramBot)]
pub struct GitInfoGlobalStaticConst {}
