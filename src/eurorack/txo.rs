pub use super::Command;
pub use super::Arg;
pub use super::Bufsize;

pub const CV_OFFSET: Command = Command {command_number: 32, args: &[ Arg{ name: "Port", argtype: Bufsize::U8}, Arg{ name: "Offset", argtype: Bufsize::S16V}]};
