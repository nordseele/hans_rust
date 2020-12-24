#### Hans - OSC I2C MIDI 

Status: Work in progress - Do not use 

*******************************************
Rust version of the Node.js project. 
OSC I2C MIDI for Eurorack Orthogonal Devices ER-301 Sound Computer, TXo, etc. 

See nordseele/hans for a full description.
*******************************************

To do: 
1 - Txo implementation (commands)
2 - Complete Basic MIDI message mapping. (The OSC to i2c and MIDI to i2c system is perfectly fine but needs a generic mapping).
3 - i2c input (Full Pipgpio binding C tor Rust or via RPPAL ?)

Done:
- Midi IO
- i2c output
- OSC input