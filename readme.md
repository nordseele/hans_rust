## Hans

Osc to i2c, MIDI to i2c for Orthogonal Devices Eurorack module ER-301 and BPCMusic Txo.

See nordseele/hans for a full description.


#### Status:
- [x] Serial MIDI to ii
- [x] Bluetooth MIDI to ii
- [x] OSC to ii

*******************************************

#### Midi mapping:

##### Channel 1 - [ Mono mode ER-301 ]

    Note On
    0 - 120: TR 1 1 | PITCH CV 1 | VELOCITY CV 2

    Note Off 
    0 - 120: TR 1 0 if Note count = 0 

    Controller 
    1: CV SLEW 1
    3 - 100: CV N