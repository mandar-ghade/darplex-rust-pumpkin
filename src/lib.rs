use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata,
    permission::{Permission, PermissionDefault, PermissionLevel},
};
use tracing::info;

use crate::{
    commands::get_set_rank_cmd,
    events::chat_event::MessageHandler,
    groups::{perms::Perm, register_perms},
};

mod cmd_wrap;
mod commands;
mod events;
mod groups;
mod text_wrap;

struct DarplexPlugin;

impl Plugin for DarplexPlugin {
    fn new() -> Self {
        DarplexPlugin
    }

    fn metadata(&self) -> pumpkin_plugin_api::PluginMetadata {
        PluginMetadata {
            name: "darplex-network".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Puckdar".into()],
            description: "Darplex network plugin".into(),
        }
    }

    fn on_load(&mut self, mut context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Darplex plugin has loaded!");

        register_perms(&mut context)?;
        context.register_command(get_set_rank_cmd(), Perm::SetRank.as_full_str());
        info!("Darplex EventHandler is registering!");
        context.register_event_handler(
            MessageHandler,
            pumpkin_plugin_api::events::EventPriority::Highest,
            true,
        )?;
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Darplex plugin has unloaded!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(DarplexPlugin);
