// Instructions
#![allow(dead_code)]
#![allow(unused_imports)]

// Modules
use std::error::Error;
use rosc::OscPacket;
use rosc::OscType;
use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};
use std::fmt;
use rppal::i2c::I2c;
mod midi;
mod osc; 
mod eurorack;
mod settings;
mod patch;
use crate::eurorack::*;
use crate::midi::*;

/*                 
//    Hans - OSC and MIDI to i2C ///////
//    Ble MIDI to i2C ///////
//    I2C to MIDI - Nordseele ® 2020 ///
//    github.com/nordseele
*/

fn main()  {
    greetings();

    // UDP
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), settings::UDP_PORT);
    let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
    println!("Listening on port {} / full addr: {}", addr.port(), addr);

    // MIDI
    let midi_in = midi::create_midi_in().unwrap();
    let midi_out = midi::create_midi_out();
    
    // OSC
    let mut buf = [0u8; rosc::decoder::MTU];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("Received packet with size {} from: {}", size, addr);
                let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                osc::handle_packet(packet);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}


/*
fn get_command(module: &str, command: &str) -> Option <Command> { // we hardcode commands that way until we find another solution
    match module {
        "Er301" | "er301" => 
            match command {
                "tr" | "TR" | "trigger" => Some(Command{command_name: String::from("tr"), command_number: 0x0}),
                "tr_tog" => Some(Command{command_name: String::from("tr_toggle"), command_number: 0x0}),
                //
                _ => None,
            },
    _ => None,
    }
}
*/
/*
#[allow(dead_code)]

*/


/*let cmd = get_command(path[0], path[2]);
    match cmd {
        Some(command) => println!("{:?}", command),
        None => println!("Not found"),
    }*/
        //println!("{}", dev);

        //let f = Follower {
        //    device: device,
        //    unit: path[1].to_string(),
        //    command: path[2].to_string(),
        //    args: args
        // };

          //  match send_i2c(f) {
          //      Ok(()) => (), 
          //      Err(_) => println!("I2c configuration error"),
          //  }

    
fn greetings() {
    println!("Hans | Nordseele 2020");
}