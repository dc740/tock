#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

openocd -c "gdb_port 3333" -c "telnet_port 4444" -c "tcl_port 6666" -f ftdi_lpc4337.cfg >> /dev/null 2>&1 &
arm-none-eabi-gdb --tui --nx target/thumbv7em-none-eabi/release/edu-ciaa -x gdb_from_commandline_startup_commands
kill $(jobs -p)
wait
