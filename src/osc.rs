#![allow(dead_code)]

use rosc::OscPacket;
use rosc::OscType;
use crate::eurorack::*;

/* pub fn handle_packet(packet: OscPacket) {
    let mut args = Vec::new();
    let mut cmd_args: Option<Command> = None;
    let mut address: usize = 0x31;
    
    match packet {
        OscPacket::Message(msg) => {
            let path: Vec< &str > = (msg.addr).trim_matches('/').split_terminator("/").collect();
            // Parse the port number and move it to the list of args if the structure of the path is OK
            if path.len() == 4 {
                match path[3].parse() {
                    Ok(v) =>  args.push(v),
                    Err(_) => OscErrors::PortNumberIncorrect.print_error(),
                };
            }
            // Push the args 
            for i in msg.args {
                match i {
                    OscType::Int(value) => args.push(value),
                    _ => ()
                }
            }   
            // Get the corresponding command
            match path[0] {
                "er301" | "Er301" =>
                    { match er301::cmd_from_string(path[2]) { 
                        Some(value) => cmd_args = Some(value), 
                        None => OscErrors::UnrecognizedAddress.print_error(),
                    };
                    // Get the address of the module
                    address = er301::address(path[1].parse::<usize>().unwrap());
                    }
                    ,
                // If the module is not found...
                _ => OscErrors::DeviceNotFound.print_error(),
            }
        },
        _ => {
            ()
        },
    }
    if let Some(cmd) = &cmd_args {
        // Send the i2c message 
       // ii::send_i2c(cmd, address, args);
    }
}
*/

pub fn handle_packet(packet: OscPacket) {
    // Split the address and route components
    match packet {
        OscPacket::Message(msg) => {
            let path: Vec< &str > = (msg.addr).trim_matches('/').split_terminator("/").collect();
            match path[0] {
                "er301" | "Er301" | "Txo" | "txo" => route_eurorack_module(&msg, path),
                "hans" | "Hans" => route_custom_action(&msg, path),
                _ => OscErrors::NoSetAction.print_error()
            }
        },
        _ => (),
    }    
}

fn route_eurorack_module(msg: &rosc::OscMessage, path: Vec< &str >) {
    println!("{:?}", msg);
    let mut data = Vec::new();
    let m = msg.args.to_vec();
    let cmd_args: Option<Command> = None;

    // Push the additional arguments/values to the vector
    for i in m {
        match i {
            OscType::Int(value) => data.push(value),
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
            "er301" | "Er301" => Some(Euro_modules::Er301),
            "txo" | "Txo" => Some(Euro_modules::Txo),
            _ => None,
        };

    // Extract a command, we'll test it later in the corresponding Module
    let command = &path[2];

    // Check if we have all the data needed and pass to the II Module
    if let (Some(module_name), Some(module_number), Some(port_number)) = (module_name, module_number, port_number) {
        ii::send_i2c(module_name, module_number, port_number, command, data)
    } else {
        println!("There's something wrong with the way your OSC message is formated, please try again.")
    }

}

fn route_custom_action(msg: &rosc::OscMessage, path: Vec< &str >) {
    println!("Custom Action!");

}

fn get_module_number(module: &str) -> Option<u8> {
    match module.parse::<u8>() {
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
            _                              => (),
        }
    }
}
