pub use super::Command;
pub use super::Arg;
pub use super::Bufsize;
pub use super::Cmd;

pub fn address(unit: usize) -> u8 {
    let max_devices = 3;
    let addr = [0x31, 0x32, 0x33];
    if unit < max_devices {
        let unit_addr = addr[unit];
        unit_addr
    } else {
        let unit_addr = addr[0];
        unit_addr
    }
}

pub fn get_cmd_from_string(cmd_name: &str) -> Option<Command>{
    match cmd_name {
        "tr" | "trigger"            => get_cmd(Args::Tr),
        "tr_tog"                    => get_cmd(Args::TrTog),
        "tr_pulse" | "trp" | "tr_p" => get_cmd(Args::TrPulse),
        "tr_time"                   => get_cmd(Args::TrTime),
        "tr_pol"                    => get_cmd(Args::TrPol),
        "cv"                        => get_cmd(Args::Cv),
        "cv_slew"                   => get_cmd(Args::CvSlew),
        "cv_set"                    => get_cmd(Args::CvSet),
        "cv_off"                    => get_cmd(Args::CvOff),
        _                           => None,
    }
}

pub fn get_cmd(cmd: Args) -> Option<Command>{
    match cmd {
        Args::Tr      => Some(Command{command_number: 0x0,  args: vec![Arg{name: String::from("port"), argtype: Bufsize::U8}]}),
        Args::TrTog   => Some(Command{command_number: 0x01, args: vec![Arg{name: String::from("port"), argtype: Bufsize::U8}]}),
        Args::TrPulse => Some(Command{command_number: 0x5,  args: vec![Arg{name: String::from("port"), argtype: Bufsize::U8}]}),
        Args::TrTime  => Some(Command{command_number: 0x32, args: vec![Arg{name: String::from("port"), argtype: Bufsize::U8}, Arg{name: String::from("ms"), argtype: Bufsize::S16}]}),
        Args::TrPol   => Some(Command{command_number: 0x6,  args: vec![Arg{name: String::from("port"), argtype: Bufsize::U8}, Arg{name: String::from("rising"), argtype: Bufsize::S16}]}),
        Args::Cv      => Some(Command{command_number: 0x10, args: vec![Arg{name: String::from("port"), argtype: Bufsize::U8}, Arg{name: String::from("volts"), argtype: Bufsize::S16V}]}),
        Args::CvSlew  => Some(Command{command_number: 0x12, args: vec![Arg{name: String::from("port"), argtype: Bufsize::U8}, Arg{name: String::from("ms"), argtype: Bufsize::S16V}]}),
        Args::CvSet   => Some(Command{command_number: 0x11, args: vec![Arg{name: String::from("port"), argtype: Bufsize::U8}, Arg{name: String::from("volts"), argtype: Bufsize::S16V}]}),
        Args::CvOff   => Some(Command{command_number: 0x15, args: vec![Arg{name: String::from("port"), argtype: Bufsize::U8}, Arg{name: String::from("volts"), argtype: Bufsize::S16V}]}),
        _             => None, 
    }
}

pub enum Args {
     Tr, 
     TrTog,
     TrPulse,
     TrTime,
     TrPol,
     Cv,
     CvSlew,
     CvSet,
     CvOff,
}
