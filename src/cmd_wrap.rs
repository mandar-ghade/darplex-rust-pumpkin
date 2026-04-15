use std::collections::HashSet;

use pumpkin_plugin_api::{
    command::{Command, CommandNode},
    commands::CommandHandler,
};

use crate::{commands::cmd_check::CmdCheck, groups::perms::Perm};

pub enum CmdWrap {
    Command(Command),
    CommandNode(CommandNode),
    // Bottom two are intermediates types (that convert to Command and CommandNode)
    CommandWithPerms(Command, HashSet<Perm>),
    CommandNodeWithPerms(CommandNode, HashSet<Perm>),
}

impl From<Command> for CmdWrap {
    fn from(value: Command) -> Self {
        CmdWrap::Command(value)
    }
}

impl From<CommandNode> for CmdWrap {
    fn from(value: CommandNode) -> Self {
        CmdWrap::CommandNode(value)
    }
}

pub trait Wrappable<T> {
    fn wrap(self) -> T;
}

impl Wrappable<CmdWrap> for Command {
    fn wrap(self) -> CmdWrap {
        CmdWrap::Command(self)
    }
}

impl Wrappable<CmdWrap> for CommandNode {
    fn wrap(self) -> CmdWrap {
        CmdWrap::CommandNode(self)
    }
}

impl CmdWrap {
    /// The main reason why I made a CmdWrap
    pub fn then(mut self, other: Self) -> Self {
        match (&mut self, other) {
            (CmdWrap::Command(lhs), CmdWrap::CommandNode(rhs)) => {
                lhs.then(rhs);
            }
            (CmdWrap::CommandNode(lhs), CmdWrap::CommandNode(rhs)) => {
                lhs.then(rhs);
            }
            (CmdWrap::CommandWithPerms(_, _), CmdWrap::CommandNodeWithPerms(_, _)) => {
                panic!("CommandWithPerms chain -> CommandNodeWithPerms not implemented!")
            }
            (CmdWrap::CommandNodeWithPerms(_, _), CmdWrap::CommandNodeWithPerms(_, _)) => {
                panic!("CommandNodeWithPerms chain -> CommandNodeWithPerms not implemented!");
            }
            (_, _) => panic!("Conditions not fullfilled!"),
        }
        self
    }

    /// If you require a permission before executing (allows for subcommand permssions)
    /// MUST have an executor if require is specified.
    pub fn require(self, perm: Perm) -> Self {
        match self {
            CmdWrap::Command(cmd) => CmdWrap::CommandWithPerms(cmd, HashSet::from([perm])),
            CmdWrap::CommandNode(cmd) => CmdWrap::CommandNodeWithPerms(cmd, HashSet::from([perm])),
            CmdWrap::CommandWithPerms(cmd, mut perms) => CmdWrap::CommandWithPerms(cmd, {
                perms.insert(perm);
                perms
            }),
            CmdWrap::CommandNodeWithPerms(cmd, mut perms) => CmdWrap::CommandNodeWithPerms(cmd, {
                perms.insert(perm);
                perms
            }),
        }
    }

    /// Execute just functions like how it did before, but with wrapping
    pub fn execute<H: CommandHandler + Send + Sync + 'static>(self, handler: H) -> Self {
        match self {
            CmdWrap::Command(lhs) => CmdWrap::Command(lhs.execute(handler)),
            CmdWrap::CommandNode(lhs) => CmdWrap::CommandNode(lhs.execute(handler)),
            CmdWrap::CommandWithPerms(lhs, perms) => {
                CmdWrap::Command(lhs.execute(CmdCheck(perms, handler)))
            }
            CmdWrap::CommandNodeWithPerms(lhs, perms) => {
                CmdWrap::CommandNode(lhs.execute(CmdCheck(perms, handler)))
            }
        }
    }

    pub fn build(self) -> Command {
        match self {
            CmdWrap::Command(cmd) => cmd,
            CmdWrap::CommandNode(_) => panic!("CommandNode not expected"),
            CmdWrap::CommandWithPerms(_, _) => {
                panic!("Cannot build a command that has perms with no executor. MUST FIX")
            }
            CmdWrap::CommandNodeWithPerms(_, _) => {
                panic!("Command build a CommandNode with permissions but no executor")
            }
        }
    }
}
