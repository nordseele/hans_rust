use std::error::Error;
pub use midir::{MidiInput, MidiOutput, Ignore};
use midir::os::unix::{VirtualInput, VirtualOutput};
use crate::patch::*;
use crate::eurorack::*;

// create Virtual MIDI IO (variables stored and kept alive in main.rs)
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
    // retrieve MIDI channel
    let channel = (bytes[0] % 16) + 1;
    // route various message types 
    match bytes[0] {
        144..=159 => NoteOn{channel: channel, number: bytes[1], velocity: bytes[2]}.to_i2c(),
        128..=143 => println!("Type: note off | Channel: {} | Number: {} | Release: {}", channel, bytes[1], bytes[2]),
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

// MIDI mapping (draft)

impl NoteOn {
    fn display(self) {
        println!("{} {} {}", self.channel, self.number, self.velocity)
    }
    fn to_i2c(self) {
        match self.number {
            1..=100 => { ii::send_i2c(EuroModules::Er301, 1, self.number, Some(er301::TR_PULSE), vec![]).ok(); },
            _ => println!("no mapping"),
        }
    }
}

impl ContinuousController {
    fn display(self) {
        println!("{} {} {}", self.channel, self.number, self.value)
    }
    fn to_i2c(self) {
        let v: u16 = self.value as u16 * 128;
        match self.number {
            1..=100 => { ii::send_i2c(EuroModules::Er301, 1, self.number, Some(er301::CV), vec![v]).ok(); },
            _ => println!("no mapping"),
        }
    }
}
