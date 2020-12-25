#### Hans - OSC I2C MIDI 

Status: Work in progress - Do not use 

*******************************************
Rust version of the Node.js project. 
OSC I2C MIDI for Eurorack Orthogonal Devices ER-301 Sound Computer, TXo, etc. 

See nordseele/hans_legacy for a full description.
*******************************************

To do: 
- Txo implementation (commands)
- Complete Basic MIDI message mapping. [1]
- i2c input [2]

Done:
- Midi IO
- i2c output
- OSC input

[1]: The OSC to ii and MIDI to ii system is perfectly fine but needs a generic mapping.

[2]: Option A -> by extending RPPAL ? | Option B -> full Pigpio binding and use bsc2xfer.