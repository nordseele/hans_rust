pub fn send_i2c() -> Result<(), Box<dyn Error>> { // return a boxed error if the i2c thing fails (bus, address or else)
    let mut i2c = I2c::with_bus(3)?;
    println!("{}", f);
    i2c.set_slave_address(0x31)?;
    i2c.block_write(0x5, &[0x1] )?;
    Ok(())
}