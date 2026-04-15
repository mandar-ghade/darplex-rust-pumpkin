use pumpkin_plugin_api::{
    command::CommandError,
    command_wit::Arg,
    commands::CommandHandler,
    text::{NamedColor, TextComponent},
};

use crate::{cmd_wrap::Wrappable, groups::PermissionGroup};

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
        _server: pumpkin_plugin_api::Server,
        args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let rank = &self.0;
        let Arg::Players(targets) = args.get_value("target") else {
            return Err(CommandError::CommandFailed(TextComponent::text(
                "Target was not found.",
            )));
        };
        if targets.is_empty() {
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

        sender.send_message(
            TextComponent::text(target.get_name().as_str())
                .wrap()
                .color_named(NamedColor::Gray)
                .add_child(
                    TextComponent::text("'s rank has been set to: ")
                        .wrap()
                        .add_child(
                            TextComponent::text(rank.as_str())
                                .wrap()
                                .color_named(rank.get_color()),
                        ),
                )
                .build(),
        );
        target.send_system_message(
            TextComponent::text("Your rank has been set to: ")
                .wrap()
                .add_child(
                    TextComponent::text(rank.as_str())
                        .wrap()
                        .color_named(rank.get_color())
                        .add_child(TextComponent::text("!").into()),
                )
                .build(),
            false,
        );
        Ok(1)
    }
}
