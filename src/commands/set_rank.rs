use pumpkin_plugin_api::{
    command::{CommandError, CommandSender},
    command_wit::Arg,
    commands::CommandHandler,
    gui::Gui,
    text::{NamedColor, TextComponent},
};
use tracing::info;

use crate::groups::PermissionGroup;

pub struct SetRankCommandExecutor(pub PermissionGroup);

/// Sets a `target`'s in-game rank.
///
/// `target`: online user
///
/// Restriction: the command sender must have a higher rank
/// than the one its trying to assign.
impl CommandHandler for SetRankCommandExecutor {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        server: pumpkin_plugin_api::Server,
        args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let rank = &self.0;
        let Arg::Players(targets) = args.get_value("target") else {
            return Err(CommandError::CommandFailed(TextComponent::text(
                "Target was not found.",
            )));
        };
        if targets.len() == 0 {
            return Err(CommandError::CommandFailed(TextComponent::text(
                "No target found!",
            )));
        } else if targets.len() > 1 {
            return Err(CommandError::CommandFailed(TextComponent::text(
                "Too many targets!",
            )));
        }

        let target = targets.first().expect("Target expected!");

        // Cannot assign rank higher than sender's own (Unless it is console)
        if let Some(player_sender) = sender.as_player()
            && player_sender.get_permission_level() < rank.get_permission_lvl()
        {
            return Err(CommandError::CommandFailed(TextComponent::text(
                "Cannot assign higher rank than YOUR current rank",
            )));
        }
        target.set_permission_level(rank.get_permission_lvl());
        sender.send_message({
            let name = TextComponent::text(target.get_name().as_str());
            let rank_text = TextComponent::text(rank.as_str());
            name.color_named(NamedColor::Gray);
            rank_text.color_named(rank.get_color());
            let body = TextComponent::text("'s rank has been set to: ");
            name.add_child({
                body.add_child(rank_text);
                body
            });
            name
        });
        target.send_system_message(
            {
                let text = TextComponent::text("Your rank has been set to: ");
                let rank_text = TextComponent::text(rank.as_str());
                rank_text.color_named(rank.get_color());
                text.add_child({
                    rank_text.add_child(TextComponent::text("!"));
                    rank_text
                });
                text
            },
            false,
        );
        Ok(1)
    }
}
