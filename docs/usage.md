# Usage


### OSC
The OSC messages received by Hans must respect the following format and the number of arguments must match the number expected by each command. 

 `/module/number/command/arg1 arg2 ` 

Where ***module*** stands for the module name (ex: er301), ***number*** is the unit number in the case you own several modules of the same type (For an ER-301, you need to activate the Teletype control in the admin menu and set the address of the module | for a Txo you need to adjust the position of the jumper), *command* is the command name (ex: cv_set) and ***arg*** is the list of arguments separated by a space and usually starting with the port number.

<u>Examples:</u>

✅ `/txo/3/tr_pulse/1` will trigger a pulse on the first gate output of your third TXo

or 

✅ `/txo/2/cv/2 3400` will set the second CV output of your second TXo to 3400



[Read the full list of commands for the TXo.](https://github.com/bpcmusic/telex/blob/master/commands.md) Bear in mind that the . must be replaced by an underscore _ . 

If the number of arguments sent does not match what the "command" expects, nothing will be sent to the "follower" module. 

<u>Example:</u>

❌ `/txo/1/cv_set/1` is missing the "value" argument, and the message will not be sent. 


### MIDI

##### MIDI Thru

By default each message received is forwarded to the MIDI output.

##### Pairing a Bluetooth MIDI device with the Raspberry Pi

    sudo bluetoothctl
        scan on 
        devices (copy device address)

        connect 48:B6:20:03:18:5B (a Roli block)

    aconnect -i (check if the Roli Block is listed here, amidiauto should connect it to Hans automatically)

##### Editing the basic MIDI mapping 
You can edit the MIDI to ii mapping in midi.rs (from line 82) It is recommended to know the Rust syntax, be careful not to break anything. You'll have to recompile the binary. You can do that by running `cargo build --release` from the hans folder when you're done editing the code.

Use the following macros in the different pattern matching blocks. ie:

    ii!(Er301, 1, 1, CV, pitch);
    ii!(Er301, 1, 2, CV, velocity);  
    ii!(Er301, 1, number, TR_PULSE);
    ii!(Er301, 1, 1, TR, 1);
    ii!(Txo, 1, 2, CV, velocity);  