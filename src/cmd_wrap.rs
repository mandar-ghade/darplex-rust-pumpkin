use pumpkin_plugin_api::{
    command::{Command, CommandNode},
    commands::CommandHandler,
};

pub enum CmdWrap {
    Command(Command),
    CommandNode(CommandNode),
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
            (_, _) => panic!("Conditions not fullfilled!"),
        }
        self
    }

    /// Execute just functions like how it did before, but with wrapping
    pub fn execute<H: CommandHandler + Send + Sync + 'static>(self, handler: H) -> Self {
        match self {
            CmdWrap::Command(lhs) => CmdWrap::Command(lhs.execute(handler)),
            CmdWrap::CommandNode(lhs) => CmdWrap::CommandNode(lhs.execute(handler)),
        }
    }

    pub fn build(self) -> Command {
        match self {
            CmdWrap::Command(cmd) => cmd,
            CmdWrap::CommandNode(_) => panic!("CommandNode not expected"),
        }
    }
}
