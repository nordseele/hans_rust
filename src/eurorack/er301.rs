pub use super::Command;
pub use super::Arg;
pub use super::Bufsize;
pub use super::er301;
/*
pub const ER301: Module = Module {
    tr       : Command {number_of_args:1, command_number: 0x0,  args: [ Arg { argtype: Bufsize::U8 }; 1]},
    tr_tog   : Command {number_of_args:1, command_number: 0x01, args: [ Arg { argtype: Bufsize::U8 }; 1]},
    tr_pulse : Command {number_of_args:1, command_number: 0x5,  args: [ Arg { argtype: Bufsize::U8 }; 1]},
    tr_time  : Command {number_of_args:2, command_number: 0x32, args: [ Arg { argtype: Bufsize::U8 }, Arg { argtype: Bufsize::S16}; 2] },
    tr_pol   : Command {number_of_args:2, command_number: 0x6,  args: [ Arg { argtype: Bufsize::U8 }, Arg { argtype: Bufsize::S16}; 2] },
    cv       : Command {number_of_args:2, command_number: 0x10, args: [ Arg { argtype: Bufsize::U8 }, Arg { argtype: Bufsize::S16V};2] },
    cv_slew  : Command {number_of_args:2, command_number: 0x12, args: [ Arg { argtype: Bufsize::U8 }, Arg { argtype: Bufsize::S16V};2] },
    cv_set   : Command {number_of_args:2, command_number: 0x11, args: [ Arg { argtype: Bufsize::U8 }, Arg { argtype: Bufsize::S16V};2] },
    cv_offset: Command {number_of_args:2, command_number: 0x15, args: [ Arg { argtype: Bufsize::U8 }, Arg { argtype: Bufsize::S16V};2] },
};
*/

pub const CV_OFFSET: Command = Command {command_number: 0x15, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "offset", argtype: Bufsize::S16V}]};

// Use static slice for array


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
/*
pub fn cmd_from_string(cmd_name: &str) -> Option<Command>{
    match cmd_name {
        "tr" | "trigger"            => Some(ER301.tr),
        "tr_tog"                    => Some(ER301.tr),
        "tr_pulse" | "trp" | "tr_p" => Some(ER301.tr),
        "tr_time"                   => Some(ER301.tr),
        "tr_pol"                    => Some(ER301.tr),
        "cv"                        => Some(ER301.tr),
        "cv_slew"                   => Some(ER301.tr),
        "cv_set"                    => Some(ER301.tr),
        "cv_off"                    => Some(ER301.tr),
        _                           => None,
    }
}
*/
pub enum Cmd {
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

pub struct Module {
    pub tr       : Command,
    pub tr_tog   : Command,
    pub tr_pulse : Command,
    pub tr_time  : Command,
    pub tr_pol   : Command,
    pub cv       : Command,
    pub cv_slew  : Command,
    pub cv_set   : Command,
    pub cv_offset: Command,
}

