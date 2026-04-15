use std::collections::{HashMap, HashSet};

use crate::groups::PermissionGroup;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref UniquePermissionsToGroup: HashMap<PermissionGroup, HashSet<Perm>> = {
        // Basically just each permission unique to each group (higher level permission level inhertits)
        let mut m = HashMap::new();
        m.insert(PermissionGroup::Owner, HashSet::from(
            [Perm::BanAdmin]));
        m.insert(PermissionGroup::Admin, HashSet::from(
            [Perm::Op, Perm::BanMod, Perm::Give, Perm::SetRank, Perm::TpDirect] // TpDirect is test
        ));
        m.insert(PermissionGroup::Mod, HashSet::from(
            [Perm::BanPlayer, Perm::Tp]
        ));
        m.insert(PermissionGroup::Player, HashSet::from(
            [Perm::Help]
        ));
        m
    };
}

#[derive(PartialEq, Eq, Hash)]
/// Not to be confused with Pumpkin's `Permission`
pub enum Perm {
    /// Owner
    BanAdmin,
    /// Admin
    Op,
    BanMod,
    Give,
    SetRank,
    TpDirect, // Test
    // Moderator
    Tp,
    BanPlayer,
    /// Player
    Help,
}

impl Perm {
    pub fn as_short_str(&self) -> &'static str {
        match self {
            Self::BanAdmin => "BanAdmin",
            Self::Op => "Op",
            Self::BanMod => "BanMod",
            Self::Give => "Give",
            Self::SetRank => "SetRank",
            Self::TpDirect => "TpDirect",
            Self::Tp => "Tp",
            Self::BanPlayer => "BanPlayer",
            Self::Help => "Help",
        }
    }

    pub fn as_full_str(&self) -> &'static str {
        match self {
            Self::BanAdmin => "darplex-network:ban-admin",
            Self::Op => "darplex-network:op",
            Self::BanMod => "darplex-network:ban-mod",
            Self::Give => "darplex-network:give",
            Self::SetRank => "darplex-network:set-rank",
            Self::TpDirect => "darplex-network:tp-direct",
            Self::Tp => "darplex-network:tp",
            Self::BanPlayer => "darplex-network:ban-player",
            Self::Help => "darplex-network:help",
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            Self::BanAdmin => "Permission to ban admins",
            Self::Op => "Permission to OP players",
            Self::BanMod => "Permission to ban moderators",
            Self::Give => "Permission to give items",
            Self::SetRank => "Permission to use setrank command",
            Self::TpDirect => "Permission to teleport directly to other players",
            Self::Tp => "Permission to teleport players",
            Self::BanPlayer => "Permission to ban players",
            Self::Help => "Permission to use help command",
        }
    }
}
