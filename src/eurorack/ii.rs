/*
    II Module => Handle, parse and route commands over i2c (Eurorack modules)
*/
use std::num::Wrapping;
use rppal::i2c::I2c;
use std::error::Error;
use crate::eurorack::*;
use super::*;

pub fn send_i2c(module_name: EuroModules, module_number: usize, port_number: u8, command:&str, data: Vec<u16>) -> Result<(), Box<dyn Error>> { // return a boxed error if the i2c thing fails (bus, address or else)
    let mut i2c = I2c::with_bus(3)?;
    //i2c.block_write(0x5, port )?;
    
    // Test the command received
    let cmd = get_module_command(&module_name, command);
    // Get the module address and set it
    let module_address: Option<usize> = get_module_address(&module_name, module_number);
    
    //i2c.set_slave_address(module_address as u16)?;
    let mut match_args = false;
    let mut args: &[Arg] = &[];
    let mut cmd_addr: u8 = 0;
    let port = offset_port_number(&module_name, port_number).unwrap();

    let buff: Vec<u8> = Vec::new();


    if let Some(cmd) = cmd {
        if data.len() == cmd.required {
            match_args = true;
            args = cmd.args;
            cmd_addr = cmd.command_number;
        }
    }

    // Carry on if we have an address and a valid command
    if let (Some(module_address), true) = (module_address, match_args) {
        let dat = &data[0].to_be_bytes();
        let p = &[port, dat[0], dat[1]];
        //let buffer = &[p, dat];
        i2c.set_slave_address(module_address as u16)?;
        i2c.block_write(cmd_addr, p)?;
        
        //println!("{:?}, {:?}, {:?}", module_address, cmd_addr, p)
    }

    Ok(())
}


fn get_module_command(module_name: &EuroModules, command: &str) -> Option<Command>{
    let mut cmd: Option<Command> = None;
    // Route command lookup to the corresponding module
        match module_name {
            EuroModules::Er301 => cmd = er301::cmd_from_string(command),
            _ => cmd = None,
        }
    cmd
}

fn get_module_address(module_name: &EuroModules, module_number: usize) -> Option<usize> {
    let mut addr: Option<usize> = None;
    // Get the address if the module number matches
        match module_name {
            EuroModules::Er301 => addr = Some(er301::ADDRESSES[((module_number + 3 - 1) % 3)]), // Modulo trick otherwise we count from 0
            _ => addr = None,
        }
        addr
}

fn offset_port_number(module_name: &EuroModules, port: u8) -> Option<u8> {
    let mut port_number: Option<u8> = None;
    match module_name {
        EuroModules::Er301 => port_number = Some((port + 101 - 1) % 101), // Modulo trick otherwise we count from 0
        _ => port_number = None,
    }
    port_number
}
