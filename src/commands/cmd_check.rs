use std::collections::HashSet;

use pumpkin_plugin_api::{
    commands::CommandHandler,
    text::{NamedColor, TextComponent},
};

use crate::{cmd_wrap::Wrappable, groups::perms::Perm};

// Perm is the required permission, H is the fallback
pub struct CmdCheck<H: CommandHandler + Send + Sync + 'static>(pub HashSet<Perm>, pub H);

fn concatonate_perms(perms: Vec<&Perm>) -> String {
    let perms_vec: Vec<String> = perms.iter().map(|p| p.as_short_str().to_string()).collect();
    perms_vec.join(", ")
}

impl<H> CommandHandler for CmdCheck<H>
where
    H: CommandHandler + Send + Sync + 'static,
{
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        server: pumpkin_plugin_api::Server,
        args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, pumpkin_plugin_api::command::CommandError> {
        let missing_permissions: Vec<&Perm> = self
            .0
            .iter()
            .filter(|perm| !sender.has_permission(&server, perm.as_full_str()))
            .collect();

        if missing_permissions.is_empty() {
            return self.1.handle(sender, server, args);
        }
        sender.send_message(
            TextComponent::text("Darplex")
                .wrap()
                .color_named(NamedColor::LightPurple)
                .bold(false)
                .add_child(
                    TextComponent::text(" >> Insufficient permissions. Missing permissions: ")
                        .wrap()
                        .color_named(NamedColor::Gray)
                        .bold(false)
                        .add_child(
                            TextComponent::text(concatonate_perms(missing_permissions).as_str())
                                .wrap()
                                .color_named(NamedColor::Yellow)
                                .bold(true),
                        ),
                )
                .build(),
        );
        Ok(1)
    }
}
