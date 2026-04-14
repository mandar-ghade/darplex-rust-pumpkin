use std::collections::{HashMap, HashSet};

use crate::groups::PermissionGroup;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref Permissions: HashMap<PermissionGroup, HashSet<Perm>> = {
        let mut m = HashMap::new();
        m.insert(
            PermissionGroup::Owner,
            HashSet::from([
                Perm::BanAdmin,
                Perm::All,
                Perm::Op,
                Perm::BanMod,
                Perm::Give,
                Perm::Tp,
                Perm::BanPlayer,
                Perm::Help,
            ]),
        );
        m.insert(
            PermissionGroup::Admin,
            HashSet::from([
                Perm::All,
                Perm::Op,
                Perm::BanMod,
                Perm::Give,
                Perm::Tp,
                Perm::BanPlayer,
                Perm::Help,
            ]),
        );
        m.insert(
            PermissionGroup::Mod,
            HashSet::from([Perm::Tp, Perm::BanPlayer, Perm::Help]),
        );
        m.insert(PermissionGroup::Player, HashSet::from([Perm::Help]));
        m
    };
}

#[derive(PartialEq, Eq, Hash)]
pub enum Perm {
    /// Owner
    BanAdmin,
    /// Admin
    All, //// Maybe get rid of this
    Op,
    BanMod,
    Give,
    // Moderator
    Tp,
    BanPlayer,
    /// Player
    Help,
}
