pub use super::Command;
pub use super::Cmd;

pub fn address(unit: usize) -> u8 {
    let addr = [0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67];
    let unit_addr = addr[unit];
    unit_addr
}

pub fn get_cmd_from_string(cmd_name: &str) -> Option<Command>{
    match cmd_name {
        "tr" | "trigger" => get_cmd(Args::Tr),
        "ts" => get_cmd(Args::Ts),
        _ => None,
    }
}

pub fn get_cmd(cmd: Args) -> Option<Command>{
    match cmd {
        Args::Tr => Some(Command{command_number: 127, args: vec![]}), 
        Args::Ts => Some(Command{command_number: 0x06, args: vec![]}),
        _ => None,
    }
}

pub enum Args {
     Tr, 
     Ts,
}

/*
impl Args {
    pub fn get(&self) -> Option<Command>{
        match self {
            Tr => Some(Command{command_number: 127, args: vec![]}), 
            Ts => Some(Command{command_number: 0x06, args: vec![]}),
        }
    }
}
*/