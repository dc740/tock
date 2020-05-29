#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT
# we need to be in the root folder or it won't find the sources.
cd ../../../
### OpenOCD
openocd -c "gdb_port 3333" -c "telnet_port 4444" -c "tcl_port 6666" -f boards/ciaa/edu-ciaa/ftdi_lpc4337.cfg >> /dev/null 2>&1 &
#arm-none-eabi-gdb --tui target/thumbv7em-none-eabi/debug/edu-ciaa -x boards/ciaa/edu-ciaa/gdb_from_commandline_startup_commands
# other debugger options:
#arm-none-eabi-gdb --tui --nx target/thumbv7em-none-eabi/release/edu-ciaa -x boards/ciaa/edu-ciaa/gdb_from_commandline_startup_commands
ddd --debugger "arm-none-eabi-gdb" --command boards/ciaa/edu-ciaa/gdb_from_commandline_startup_commands_ddd target/thumbv7em-none-eabi/debug/edu-ciaa

## JLink allows us to properly debug line by line!!!!!!!!!!
### Start server from /opt/SEGGER/JLink/JLinkGDBServerExe
# execdbg help us unwind the stack trace when used with a custom hardware fault handler
# specially for this purpose.
# make sure these two are in the asm of the handler. in this order. 10 can be any number. 
# bkpt 10
# bx lr
#/opt/SEGGER/JLink/JLinkGDBServerCLExe -device Cortex-M4 -endian little -if SWD -speed auto -LocalhostOnly 1 -execdbg >> /dev/null 2>&1 &
#arm-none-eabi-gdb --tui target/thumbv7em-none-eabi/release/edu-ciaa -x gdb_from_commandline_startup_commands_jlink

kill $(jobs -p)
wait
cd -
