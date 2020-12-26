## Hans

Osc to ii, MIDI to ii for ER-301 and Txo.
See nordseele/hans for a full description.


*Feel free to suggest a full generic MIDI mapping (MIDI to ii for the Er-301) by posting an issue in this repository or DM me on Lines.*

#### Status:
- [x] MIDI to ii (ER-301 + basic set for TXo)
- [x] OSC to ii (ER-301 + basic set for TXo)
- [ ] Full generic MIDI mapping 

*******************************************

#### Midi mapping:

##### Channel 1

    Note On
    60-71: TR pulse
    72-83: TR toggle

    Note Off 
    72-83: TR toggle

    Controller 
    1-100: CV