use crate::{cmd_wrap::Wrappable, groups::PermissionGroup};
use pumpkin_plugin_api::{
    events::{EventData, EventHandler, PlayerChatEvent},
    text::{NamedColor, TextComponent},
};

pub struct MessageHandler;

fn get_text(rank: &PermissionGroup, name_str: &str, message: &str) -> TextComponent {
    // Unranked players got no tag
    let mut rank_str = String::new();

    if *rank != PermissionGroup::Player {
        rank_str = String::from(rank.as_str()) + " ";
    }

    TextComponent::text(&rank_str)
        .wrap()
        .color_named(rank.get_color())
        .bold(true)
        .add_child(
            TextComponent::text(name_str)
                .wrap()
                .color_named(NamedColor::Yellow)
                .bold(false)
                .add_child(
                    TextComponent::text(message)
                        .wrap()
                        .color_named(NamedColor::Gray)
                        .bold(false),
                ),
        )
        .build()
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
