use std::collections::HashSet;

use crate::groups::perms::{Perm, UniquePermissionsToGroup};
use pumpkin_plugin_api::{
    Context,
    permission::{Permission, PermissionDefault, PermissionLevel},
    text::NamedColor,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod perms;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum PermissionGroup {
    Owner,
    Admin,
    Mod,
    Player,
}

impl PermissionGroup {
    /// Permissions handling is done within `Pumpkin`
    ///
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
            PermissionLevel::Two => todo!(), // TODO: Fix
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

    pub fn get_permission_default(&self) -> PermissionDefault {
        match self {
            Self::Owner => PermissionDefault::Op(PermissionLevel::Four),
            Self::Admin => PermissionDefault::Op(PermissionLevel::Three),
            Self::Mod => PermissionDefault::Op(PermissionLevel::One),
            Self::Player => PermissionDefault::Allow, // TODO: Figure out whether lvl zero is
                                                      // better?
        }
    }
}

/// Internally, permission inheritence should work.
/// TODO: Run thorough (in-game) tests to check that permission inheritance works..
pub fn register_perm(
    ctx: &mut Context,
    perm: &Perm,
    group: &PermissionGroup,
) -> Result<(), String> {
    ctx.register_permission(&Permission {
        node: perm.as_full_str().into(),
        description: perm.get_description().into(),
        default: group.get_permission_default(),
        children: Vec::new(),
    })
}

/// Time complexity: O(p) where p = every stored permission (where each is unique)
///
/// (`UniquePermissionsToGroup` contains every permission)
pub fn register_perms(ctx: &mut Context) -> Result<(), String> {
    let mut perm_set: HashSet<&Perm> = HashSet::new();
    // Just to make sure that all permissions only initialize once
    for group in PermissionGroup::iter() {
        let Some(permission_set) = UniquePermissionsToGroup.get(&group) else {
            continue;
        };
        for perm in permission_set.iter() {
            if !perm_set.contains(perm) {
                register_perm(ctx, perm, &group)?;
            }
            perm_set.insert(perm); // Adds to set
        }
    }
    Ok(())
}
