use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata,
    command::Command,
    events::{EventHandler, PlayerCommandSendEvent},
    permission::{Permission, PermissionDefault, PermissionLevel},
};
use tracing::info;

use crate::commands::get_set_rank_cmd;

mod commands;
mod groups;

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

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Darplex plugin has loaded!");
        context.register_permission(&Permission {
            node: "darplex-network::set-rank".into(),
            description: "Set rank".into(),
            default: PermissionDefault::Op(PermissionLevel::Three),
            children: Vec::new(),
        })?;

        context.register_command(get_set_rank_cmd(), "darplex-network::set-rank");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Darplex plugin has unloaded!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(DarplexPlugin);
