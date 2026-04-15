use pumpkin_plugin_api::{
    command::CommandError, command_wit::Arg, commands::CommandHandler, text::TextComponent,
};

pub struct TpDirect();

impl CommandHandler for TpDirect {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        server: pumpkin_plugin_api::Server,
        args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, pumpkin_plugin_api::command::CommandError> {
        if sender.is_console() {
            return Err(CommandError::CommandFailed(TextComponent::text(
                "Must be a player.",
            )));
        }
        let Arg::Players(targets) = args.get_value("target") else {
            return Err(CommandError::CommandFailed(TextComponent::text(
                "Target was not found.",
            )));
        };
        let target = targets.first().expect("Target expected");
        let sender = sender.as_player().expect("Player expected");
        sender.teleport(
            target.get_position(),
            Some(target.get_yaw()),
            Some(target.get_pitch()),
            target.get_world(),
        );
        sender.send_system_message(TextComponent::text("Teleport successful!"), true);
        Ok(1)
    }
}
