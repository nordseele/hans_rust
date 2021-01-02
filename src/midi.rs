use std::error::Error;
pub use midir::{MidiInput, MidiOutput, Ignore};
use midir::os::unix::{VirtualInput, VirtualOutput};
use crate::patch::*;
use crate::eurorack::*;

#[macro_export]

macro_rules! ii {
    ($module:ident, $unit:expr, $port:expr, $cmd:ident, $arg:ident) => {
        {ii::send_i2c(EuroModules::$module, $unit, $port, Some(er301::$cmd), vec![$arg as u16]).ok();}
    };
}

pub fn create_midi_in() -> Result<midir::MidiInputConnection<()>, Box<dyn Error>>  {
    let mut midi_in = MidiInput::new("Hans Input")?;
    midi_in.ignore(Ignore::SysexAndTime);
    println!("Opening virtual Midi in");
    Ok(
        midi_in.create_virtual("Hans midi input", move |_stamp, message, _| {
            handle_midi_message(message);  
        }, ())?
    )
}

pub fn create_midi_out() -> Result<midir::MidiOutputConnection, Box<dyn Error>>  {
    let midi_out = MidiOutput::new("Hans Output")?;
    println!("Opening virtual Midi out");
    Ok(
        midi_out.create_virtual("Hans midi output")?
    )
}

fn handle_midi_message(bytes: &[u8]) {
    let channel = (bytes[0] % 16) + 1;
    match bytes[0] {
        144..=159 => NoteOn{channel: channel, number: bytes[1], velocity: bytes[2]}.to_i2c(),
        128..=143 => NoteOff{channel: channel, number: bytes[1], velocity: bytes[2]}.to_i2c(),
        160..=175 => println!("Polyphonic aftertouch"),
        176..=191 => ContinuousController{channel: channel, number: bytes[1], value: bytes[2]}.to_i2c(),
        192..=207 => println!("Program Change"),
        250 => println!("Start"),
        251 => println!("Continue"),
        252 => println!("Stop"),
        _ => println!("?")
    }
}

struct NoteOn {
    channel: u8, 
    number: u8,
    velocity: u8,
}

struct NoteOff {
    channel: u8, 
    number: u8,
    velocity: u8,
}

struct ContinuousController {
    channel: u8, 
    number: u8,
    value: u8,
}

pub struct NoteCount {
    pub count: u8,
}
    
static mut NOTE_COUNT: u8 = 0;

// MIDI mapping WIP

impl NoteOn {
    fn display(self) {
        println!("{} {} {}", self.channel, self.number, self.velocity)
    }
    fn to_i2c(self) {

        let velocity: usize = (self.velocity as usize ) * 16384 / 127;
        let pitch: usize = (self.number as usize ) * 16384 / 120;

        match self.channel {
            1 => match self.number {
                0..=120 => {
                    unsafe {NOTE_COUNT += 1; println!("{}", NOTE_COUNT)};
                    //ii::send_i2c(EuroModules::Er301, 1, 1, Some(er301::TR), vec![1]).ok();
                    //ii::send_i2c(EuroModules::Er301, 1, 1, Some(er301::CV), vec![pitch as u16]).ok();

                    ii!(Er301, 1, 1, TR, 1);
                    ii!(Er301, 1, 1, CV, pitch);
                    ii!(Er301, 1, 2, CV, velocity);
                    //ii::send_i2c(EuroModules::Er301, 1, 2, Some(er301::CV), vec![velocity as u16]).ok();
                },
                _ => (),
            },
            2 => match self.number {
                0..=120 => { 
                    ii::send_i2c(EuroModules::Er301, 1, 1, Some(er301::TR_PULSE), vec![]).ok();
                },
                _ => (),
            },
            _ => (),
        }
    }
}

impl NoteOff {
    fn display(self) {
        println!("{} {} {}", self.channel, self.number, self.velocity)
    }
    fn to_i2c(self) {

        let velocity: usize = (self.velocity as usize ) * 16384 / 127;
        let pitch: usize = (self.number as usize ) * 16384 / 120;

        match self.channel {
            1 => match self.number {
                0..=120 => {
                    unsafe {
                    let count = NOTE_COUNT;
                    if NOTE_COUNT != 0 {NOTE_COUNT -= 1; println!("{}", NOTE_COUNT)} else {()};
                    if count == 1 {
                        ii::send_i2c(EuroModules::Er301, 1, 2, Some(er301::CV), vec![0]).ok();
                        ii::send_i2c(EuroModules::Er301, 1, 1, Some(er301::TR), vec![0]).ok(); () } else {()};
                    };
                },
                _ => (),
            },
            _ => (),
        }
    }
}

impl ContinuousController {
    fn display(self) {
        println!("{} {} {}", self.channel, self.number, self.value)
    }
    fn to_i2c(self) {

        let value: usize = (self.value as usize ) * 16384 / 127;

        match self.channel {
            1 => match self.number {
                1 => { ii::send_i2c(EuroModules::Er301, 1, self.number, Some(er301::CV_SLEW), vec![value as u16]).ok();},
                3..=100 => { ii::send_i2c(EuroModules::Er301, 1, self.number, Some(er301::CV), vec![value as u16]).ok();},
                _ => (),
            },
            _ => (),
        } 
    }
}
