use pumpkin_plugin_api::text::NamedColor;

enum PermissionGroup {
    ADMIN(NamedColor),
    MOD(NamedColor),
    TRAINEE(NamedColor),
    ALL(NamedColor),
}

const OWNER: PermissionGroup = PermissionGroup::ADMIN(NamedColor::DarkRed);
const ADMIN: PermissionGroup = PermissionGroup::ADMIN(NamedColor::Red);
const MODERATOR: PermissionGroup = PermissionGroup::MOD(NamedColor::DarkBlue);
const EVERYONE: PermissionGroup = PermissionGroup::ALL(NamedColor::Gray);
