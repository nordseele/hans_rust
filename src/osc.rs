#![allow(dead_code)]

use rosc::OscPacket;
use rosc::OscType;
use crate::eurorack::*;

pub fn handle_packet(packet: OscPacket) {
    let mut args = Vec::new();
    let mut cmd_args: Option<Command> = None;
    
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

            for i in msg.args {
                match i {
                    OscType::Int(value) => args.push(value),
                    _ => ()
                }
            }   

            match path[0] {
                "er301" | "Er301" =>
                    match er301::get_cmd_from_string(path[2]) { 
                        Some(value) => cmd_args = Some(value), 
                        None => OscErrors::UnrecognizedAddress.print_error(),
                    },
                "txo" | "Txo" =>
                    match txo::get_cmd_from_string(path[2]) {
                        Some(value) => cmd_args = Some(value),
                        None => OscErrors::UnrecognizedAddress.print_error(),
                    }, 
                _ => OscErrors::DeviceNotFound.print_error(),
            }
        },
        _ => {
            ()
        },
    } // endof match packet

    match &cmd_args {
        //Some(value) => if args.len() == value.args.len() {println!("{:?} with value {:?}", value, args)} else { println!("incorrect amount of arguments")},
        Some(value) =>  println!("{:?} with value {:?} received {} args and needs {}", value, args, args.len(), value.args.len()),
        None => println!("Cannot parse OSC message"),
    }
}

pub enum OscErrors {
    DeviceNotFound,
    UnrecognizedAddress,
    CommandNotFound,
    PortNumberIncorrect
}
impl OscErrors {
    pub fn print_error(&self) {
        match self {
            OscErrors::DeviceNotFound      => println!("Device not found"),
            OscErrors::UnrecognizedAddress => println!("Address not found"),
            OscErrors::CommandNotFound     => println!("Command not found"),
            OscErrors::PortNumberIncorrect => println!("The port number is incorrect"),
            _                              => (),
        }
    }
}