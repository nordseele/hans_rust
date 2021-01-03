use rosc::OscPacket;
use rosc::OscType;
use crate::eurorack::*;

pub fn handle_packet(packet: OscPacket) {
    // split the address and route the components
    match packet {
        OscPacket::Message(msg) => {
            let path: Vec< &str > = (msg.addr).trim_matches('/').split_terminator("/").collect();
            match path[0] {
                // route to Eurorack modules
                "er301" | "Er301" | "Txo" | "txo" => route_eurorack_module(&msg, path),
                // route to the set of custom actions defined in submodules
                "hans" | "Hans" => route_custom_action(&msg, path),
                _ => OscErrors::NoSetAction.print_error()
            }
        },
        _ => (),
    }    
}

fn route_eurorack_module(msg: &rosc::OscMessage, path: Vec< &str >) {
    println!("{:?}", msg);
    let mut data: Vec<u16> = Vec::new();
    let m = msg.args.to_vec();
    let _cmd_args: Option<Command> = None;
    // push the additional arguments/values to the vector
    for i in m {
        match i {
            OscType::Int(value) => data.push(value as u16),
            _ => ()
        }
    } 
    let module_number = get_module_number(&path[1]);
    let port_number = get_port_number(&path);

    // assign an enum variant to the module name, check if the module exists
    let module_name =
        match path[0] {
            "er301" | "Er301" => Some(EuroModules::Er301),
            "txo" | "Txo"     => Some(EuroModules::Txo),
            _                 => None,
        };
    
    // check if we have all the data needed and pass to the II Module
    if let (Some(module_name), Some(module_number), Some(port_number)) = (module_name, module_number, port_number) {
        // get the command details
        let command = get_module_command(&module_name, &path[2]);
        match ii::send_i2c(module_name, module_number, port_number, command, data) {
            Ok(_) => {},
            Err(_) => println!("Unreachable module"),
        }
    } else {
        println!("Osc format error")
    }
}

fn route_custom_action(_msg: &rosc::OscMessage, _path: Vec< &str >) {
    println!("Custom Action!");

}

fn get_module_number(module: &str) -> Option<usize> {
    match module.parse::<usize>() {
        Ok(value) => Some(value),
        Err(_) => None,

    }
}

fn get_module_command(module_name: &EuroModules, command: &str) -> Option<Command>{
    let cmd;
    // route command lookup to the corresponding module
    match module_name {
        EuroModules::Er301 => cmd = er301::cmd_from_string(command),
        EuroModules::Txo => cmd = txo::cmd_from_string(command),
    }
    cmd
}

fn get_port_number(path: &Vec<&str>) -> Option<u8> {
    let port: Option<u8>;
    if path.len() == 4 {
        match path[3].parse::<u8>() {
            Ok(value) =>  port = Some(value),
            Err(_) => port = None,
        }
    } else {
        port = None
    }
    port
}

pub enum OscErrors {
    DeviceNotFound,
    UnrecognizedAddress,
    NoSetAction,
    CommandNotFound,
    PortNumberIncorrect
}
impl OscErrors {
    pub fn print_error(&self) {
        match self {
            OscErrors::DeviceNotFound      => println!("Device not found"),
            OscErrors::UnrecognizedAddress => println!("Address not found"),
            OscErrors::NoSetAction         => println!("No corresponding route"),
            OscErrors::CommandNotFound     => println!("Command not found"),
            OscErrors::PortNumberIncorrect => println!("Incorrect port number"),
        }
    }
}
