use crate::groups::perms::{Perm, Permissions};
use pumpkin_plugin_api::{permission::PermissionLevel, text::NamedColor};

pub mod perms;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PermissionGroup {
    Owner,
    Admin,
    Mod,
    Player,
}

impl PermissionGroup {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Owner => "Owner",
            Self::Admin => "Admin",
            Self::Mod => "Mod",
            Self::Player => "Player",
        }
    }

    pub fn from_permission_lvl(lvl: &PermissionLevel) -> Self {
        match lvl {
            PermissionLevel::Four => Self::Owner,
            PermissionLevel::Three => Self::Admin,
            PermissionLevel::Two => Self::Player, // TODO: Fix
            PermissionLevel::One => Self::Mod,
            PermissionLevel::Zero => Self::Player,
        }
    }

    pub fn get_color(&self) -> NamedColor {
        match self {
            Self::Owner => NamedColor::DarkRed,
            Self::Admin => NamedColor::Red,
            Self::Mod => NamedColor::Blue,
            Self::Player => NamedColor::Gray,
        }
    }

    pub fn get_permission_lvl(&self) -> PermissionLevel {
        match self {
            Self::Owner => PermissionLevel::Four,
            Self::Admin => PermissionLevel::Three,
            Self::Mod => PermissionLevel::One,
            Self::Player => PermissionLevel::Zero,
        }
    }

    pub fn has_perm(&self, p: Perm) -> bool {
        Permissions
            .get(self)
            .expect(&format!(
                "PermissionGroup {:?} not found in `Permissions` table!",
                self
            ))
            .contains(&p)
    }
}
