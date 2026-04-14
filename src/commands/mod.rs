use pumpkin_plugin_api::{
    command::{Command, CommandNode},
    command_wit::{ArgumentType, StringType},
};

use crate::{commands::set_rank::SetRankCommandExecutor, groups::PermissionGroup};

pub mod set_rank;

pub fn get_set_rank_cmd() -> Command {
    let set_rank_cmd_names = ["setrank".to_string()];
    let description = "Set rank cmd!";
    let cmd = Command::new(&set_rank_cmd_names, description);
    let target = CommandNode::argument("target", &ArgumentType::String(StringType::SingleWord));
    // let rank = CommandNode::argument("rank", &ArgumentType::String(StringType::SingleWord));
    let owner = CommandNode::literal("Owner");
    let admin = CommandNode::literal("Admin");
    let moderator = CommandNode::literal("Mod");
    let player = CommandNode::literal("Player");
    // TODO: Make note of `execute` being a builder-like return function. Execute runs on very last
    // variable specified

    cmd.then({
        target.then(owner.execute(SetRankCommandExecutor(PermissionGroup::Owner)));
        target.then(admin.execute(SetRankCommandExecutor(PermissionGroup::Admin)));
        target.then(moderator.execute(SetRankCommandExecutor(PermissionGroup::Mod)));
        target.then(player.execute(SetRankCommandExecutor(PermissionGroup::Player)));
        target
    });
    cmd
}
