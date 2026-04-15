use pumpkin_plugin_api::{
    command::{Command, CommandNode},
    command_wit::ArgumentType,
};

use crate::{
    cmd_wrap::Wrappable, commands::set_rank::SetRankCommandExecutor, groups::PermissionGroup,
};

pub mod set_rank;

pub fn get_set_rank_cmd() -> Command {
    let set_rank_cmd_names = ["setrank".to_string(), "giverank".to_string()];
    let description = "Set rank cmd!";
    let cmd = Command::new(&set_rank_cmd_names, description).wrap();
    let target = CommandNode::argument("target", &ArgumentType::Players).wrap();
    let owner = CommandNode::literal("Owner").wrap();
    let admin = CommandNode::literal("Admin").wrap();
    let moderator = CommandNode::literal("Mod").wrap();
    let player = CommandNode::literal("Player").wrap();
    cmd.then(
        target
            .then(owner.execute(SetRankCommandExecutor(PermissionGroup::Owner)))
            .then(admin.execute(SetRankCommandExecutor(PermissionGroup::Admin)))
            .then(moderator.execute(SetRankCommandExecutor(PermissionGroup::Mod)))
            .then(player.execute(SetRankCommandExecutor(PermissionGroup::Player))),
    )
    .build()
}
