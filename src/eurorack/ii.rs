use rppal::i2c::I2c;
use std::error::Error;
use super::*;

pub fn send_i2c(module_name: Euro_modules, module_number: u8, port_number: u8, command:&str, data: Vec<i32>) /*-> Result<(), Box<dyn Error>>*/ { // return a boxed error if the i2c thing fails (bus, address or else)
    //let mut i2c = I2c::with_bus(3)?;
    //i2c.set_slave_address(address as u16)?;
    //i2c.block_write(0x5, port )?;
    println!("{:?}, {:?}, {:?}, {:?}, {:?}", module_name, module_number, port_number, command, data);
   // Ok(())
}

// TODO

/*
    Match the command
    Match the number of arguments (might need to add the number of arguments in the CONST directly)
    Get follower address
    Format the buffer ? 
    Send !
*/