use pumpkin_plugin_api::{
    command::{CommandError, CommandSender},
    command_wit::Arg,
    commands::CommandHandler,
    text::{NamedColor, TextComponent},
};
use tracing::info;

use crate::groups::PermissionGroup;

pub struct SetRankCommandExecutor(pub PermissionGroup);

impl CommandHandler for SetRankCommandExecutor {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        server: pumpkin_plugin_api::Server,
        args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let rank = &self.0;
        // if !sender.is_player() {
        //     return Err(CommandError::CommandFailed(TextComponent::text(
        //         "Sender must be player.",
        //     )));
        // }
        let Arg::Simple(target) = args.get_value("target") else {
            return Err(CommandError::CommandFailed(TextComponent::text(
                "Target not specified.",
            )));
        };
        let Some(target) = server.get_player_by_name(target.as_str()) else {
            // TODO: Offline handling
            return Err(CommandError::CommandFailed(TextComponent::text(
                "Target was not online.",
            )));
        };
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
