use pumpkin_plugin_api::{
    events::{EventData, EventHandler, PlayerChatEvent},
    text::{NamedColor, TextComponent},
};
use tracing::info;

use crate::groups::PermissionGroup;

pub struct MessageHandler;

fn get_text(rank: &PermissionGroup, name_str: &str, message: &str) -> TextComponent {
    // "RANK "
    let mut rank_str = String::from(rank.as_str()) + " ";

    // Players got no tag
    if *rank == PermissionGroup::Player {
        rank_str = String::new();
    }

    let rank_text = TextComponent::text(rank_str.as_str());
    rank_text.color_named(rank.get_color());
    rank_text.bold(true);
    let name = TextComponent::text(name_str);
    name.color_named(NamedColor::Yellow);
    name.bold(false);

    let body = TextComponent::text(message);
    body.color_named(NamedColor::Gray);
    name.bold(false);

    rank_text.add_child({
        name.add_child(body);
        name
    });

    rank_text
}

impl EventHandler<PlayerChatEvent> for MessageHandler {
    fn handle(
        &self,
        _server: pumpkin_plugin_api::Server,
        mut event: EventData<PlayerChatEvent>,
    ) -> EventData<PlayerChatEvent> {
        event.cancelled = true;

        let rank = PermissionGroup::from_permission_lvl(&event.player.get_permission_level());
        let name_str = format!("{} ", event.player.get_name());

        dbg!(event.recipients.len());
        for player in event.recipients.iter() {
            player.send_system_message(get_text(&rank, &name_str, &event.message), false);
        }
        event
            .player
            .send_system_message(get_text(&rank, &name_str, &event.message), false);

        event
    }
}
