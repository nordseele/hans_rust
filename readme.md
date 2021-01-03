## Hans

Osc to ii, MIDI to ii for ER-301 and Txo.
See nordseele/hans for a full description.


#### Status:
- [x] MIDI to ii
- [x] OSC to ii
- [ ] Full generic MIDI mapping 

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