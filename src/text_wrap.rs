use pumpkin_plugin_api::text::{NamedColor, TextComponent};

use crate::cmd_wrap::Wrappable;

pub struct TextWrap(TextComponent);

impl From<TextComponent> for TextWrap {
    fn from(value: TextComponent) -> Self {
        Self(value)
    }
}

impl Wrappable<TextWrap> for TextComponent {
    fn wrap(self) -> TextWrap {
        self.into()
    }
}

impl TextWrap {
    pub fn color_named(self, named_color: NamedColor) -> Self {
        self.0.color_named(named_color);
        self
    }

    pub fn add_child(self, other: Self) -> Self {
        self.0.add_child(other.0);
        self
    }

    pub fn bold(self, value: bool) -> Self {
        self.0.bold(value);
        self
    }

    pub fn build(self) -> TextComponent {
        self.0
    }
}
