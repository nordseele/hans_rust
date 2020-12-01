use crate::eurorack::*;

// match ?


pub enum Midi {
    Note_on {channel: u8, number: u8, velocity: u8},
    Note_off {channel: u8, number: u8, velocity: u8},
    CC {channel: u8, number: u8},
}

impl Midi {
    fn handle (&self) {
        match self {
            Note_on => println!("Hello "),
            Note_off => println!("Hi "),
            CC => println!("Hey"),
        }
    }
}