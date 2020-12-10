/*
    OSC Module => Handle, parse and route OSC messages
*/

use rosc::OscPacket;
use rosc::OscType;
use crate::eurorack::{ii, Command, EuroModules};

pub fn handle_packet(packet: OscPacket) {
    // Split the address and route the components
    match packet {
        OscPacket::Message(msg) => {
            let path: Vec< &str > = (msg.addr).trim_matches('/').split_terminator("/").collect();
            match path[0] {
                // Route to Eurorack modules
                "er301" | "Er301" | "Txo" | "txo" => route_eurorack_module(&msg, path),
                // Route to the set of custom actions defined in submodules
                "hans" | "Hans" => route_custom_action(&msg, path),
                _ => OscErrors::NoSetAction.print_error()
            }
        },
        _ => (),// we don't care for OSC bundles yet
    }    
}

fn route_eurorack_module(msg: &rosc::OscMessage, path: Vec< &str >) {
    println!("{:?}", msg);
    let mut data: Vec<u16> = Vec::new();
    let m = msg.args.to_vec();
    let _cmd_args: Option<Command> = None;
    // Push the additional arguments/values to the vector
    for i in m {
        match i {
            OscType::Int(value) => data.push(value as u16),
            _ => ()
        }
    }
    // Retrieve the module number  
    let module_number = get_module_number(&path[1]);
    // Retrieve the port number  
    let port_number = get_port_number(&path);

    // Assign an enum variant to the module name, check if the module exists
    let module_name =
        match path[0] {
            "er301" | "Er301" => Some(EuroModules::Er301),
            "txo" | "Txo" => Some(EuroModules::Txo),
            _ => None,
        };
    // Extract a command, we'll test it later in the corresponding Module
    let command = &path[2];
    // Check if we have all the data needed and pass to the II Module
    if let (Some(module_name), Some(module_number), Some(port_number)) = (module_name, module_number, port_number) {
        match ii::send_i2c(module_name, module_number, port_number, command, data) {
            Ok(_) => {},
            Err(_) => println!("Are you sure the Eurorack module is connected ?"),
        }
    } else {
        println!("There's something wrong with the way your OSC message is formated, please try again.")
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
            OscErrors::NoSetAction => println!("I cannot find a corresponding route"),
            OscErrors::CommandNotFound     => println!("Command not found"),
            OscErrors::PortNumberIncorrect => println!("The port number is incorrect"),
        }
    }
}
