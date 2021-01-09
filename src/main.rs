// instructions
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// modules
use std::error::Error;
use rosc::OscPacket;
use rosc::OscType;
use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};
use std::fmt;
use std::{thread, time};
mod midi;
mod osc; 
mod eurorack;
mod settings;
mod patch;
use crate::eurorack::*;
use crate::midi::*;

/*                 
  Hans - OSC and MIDI to i2C ///////
  Ble MIDI to i2C ///////
  I2C to MIDI - Nordseele ® 2020 ///
  github.com/nordseele
*/

fn main()  {

    greetings();

    // udp
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), settings::UDP_PORT);
    let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
    println!("Listening on port {} / full addr: {}", addr.port(), addr);

    // midi : midi.rs 
    let midi_in = midi::create_midi_in().unwrap();
    let mut midi_out = midi::create_midi_out().unwrap();
    //let connect_out = midi::connect_midi_out(midi_out);


    // osc : osc.rs 
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
        
    loop {
        let delay = time::Duration::from_millis(175);
        midi_out.send(&[144, 60, 1]).unwrap();
        println!("midi");
        thread::sleep(delay);
    }
}


fn greetings() {
    println!("Hans | Nordseele 2020");
}