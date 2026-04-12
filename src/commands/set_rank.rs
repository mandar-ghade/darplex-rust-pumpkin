use pumpkin_plugin_api::{
    command::{CommandError, CommandSender},
    command_wit::Arg,
    commands::CommandHandler,
    gui::TextComponent,
};
use tracing::info;

pub struct SetRankCommandExecutor;

impl CommandHandler for SetRankCommandExecutor {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        server: pumpkin_plugin_api::Server,
        args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if !sender.is_player() {
            return Ok(1);
        }
        if let Arg::Simple(target) = args.get_value("target")
            && let Arg::Simple(rank) = args.get_value("rank")
            && sender.is_player()
        {
            sender
                .as_player()
                .expect("Player expected")
                .send_system_message(
                    TextComponent::text(&format!(
                        "{}'s rank has been set to: {}!",
                        target.as_str(),
                        rank.as_str()
                    )),
                    true,
                );
        } else {
            info!("Yo this is bad")
        }
        Ok(1)
    }
}
