use pumpkin_plugin_api::{
    command::{Command, CommandNode},
    command_wit::ArgumentType,
};

use crate::commands::set_rank::SetRankCommandExecutor;

pub mod set_rank;

pub fn get_set_rank_cmd() -> Command {
    let set_rank_cmd_names = ["setrank".to_string()];
    let description = "Set rank cmd!";
    let cmd = Command::new(&set_rank_cmd_names, description);
    let target = CommandNode::argument(
        "target",
        &ArgumentType::String(pumpkin_plugin_api::command_wit::StringType::SingleWord),
    );
    let rank = CommandNode::argument(
        "rank",
        &ArgumentType::String(pumpkin_plugin_api::command_wit::StringType::SingleWord),
    );
    // TODO: Make note of `execute` being a builder-like return function. Execute runs on very last
    // variable specified
    target.then(rank.execute(SetRankCommandExecutor));
    cmd.then(target);
    cmd
}
