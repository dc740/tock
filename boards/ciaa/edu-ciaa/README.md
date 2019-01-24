Platform-Specific Instructions: EDU-CIAA-NXP
===================================

The EDU-CIAA-NXP board is based on the LPC 4337 chip.
It's used as a teaching platform in Argentina, and it was
developed locally. The CIAA-NXP is its bigger sibling, designed
specifially for industrial usage.

The boards are open source and can be obtained here: http://www.proyecto-ciaa.com.ar/index_en.html

There are many different versions, using different chips, depending on the
develpment needs.

## Getting Started

First, follow the [Tock Getting Started guide](../../doc/Getting_Started.md)

JTAG on the Debug USB port is the prefered connection method.
Then you can use OpenOCD to program the board and the apps.

Requirements:
1. ftdi_lpc4337.cfg must be present in the openocd board folder 

(OpenOCD boards folders are located in /usr/share/openocd/scripts/board,
or /usr/local/share/openocd/scripts/board)

### Programming the kernel

Once you have all software installed, you should be able to simply run
`make flash` in this directory to install a fresh kernel.

### Programming user-level applications

You can program an application via OpenOCD:
 1. via `tockloader`:

    ```bash
    $ cd libtock-c/examples/<app>
    $ make
    $ tockloader  install blink --openocd --arch cortex-m4 --board edu-ciaa --openocd-board ftdi_lpc4337.cfg
    ```


If you run this in the application folder, `tockloader` will automatically
find the tab to flash, otherwise you need to specify the path.

### Debugging the kernel

To debug 

1. Configure Eclipse IDE with the GNU MCU extensions (Help -> Eclipse Marketplace -> search for "GNU  MCU Eclipse"), and run the "OpenOCD GDB Debugging" target.
2. Set arm-none-eabi-gdb is available and selected as debugger
3. Add this line to the Debugger config options `-f /usr/local/share/openocd/scripts/board/ftdi_lpc4337.cfg`
4. Select the edu-ciaa kernel elf so the debugger can follow you through the Rust code in the IDE. `boards/ciaa/edu-ciaa/target/thumbv7em-none-eabi/release/edu-ciaa.elf`
5. Make sure your Debug configuration has the SVD file in the 'SVD Path' tab so you can see the registers
6. Set the C++ settings so the build directory is the edu-ciaa board directory, so the incremental build doesn't throw an error preventing you from going into debug mode.

For extra instructions check:
http://www.proyecto-ciaa.com.ar/devwiki/doku.php?id=repo%3Aconfiguracion%3Adebug

Alternatively you can do
`/usr/bin/openocd -c gdb_port 3333 -c telnet_port 4444 -c tcl_port 6666 -f /home/dc740/CIAA/repos/tock/boards/ciaa/edu-ciaa/ftdi_lpc4337.cfg`

`arm-none-eabi-gdb --tui --nx target/thumbv7em-none-eabi/release/edu-ciaa`


Then run these commands in gdb to debug without Eclipse:

```
target remote localhost:3333
set mem inaccessible-by-default off
monitor reset halt
layout split
```

To go through the assembly use `si` and `ni` to step through the machine instructions instead of the rust code.


#### Debugging Tricks

Since Rust heavily optimized and inlines code, it can be difficult to
understand, from the assembly, exactly where you are in source code. Two
tricks can help in this regard: the ``inline`` and ``no_mangle`` attributes. If you label a function

```rust
#[inline(never)]
```

then Rust will not inline it so you can see calls into it and break on
entry. However, since Rust often emits complex symbol names, you also
might want to use

```rust
#[no_mangle]
```

which will keep the function's symbol identical to the function name.
For example, if you do this:

```rust
#[no_mangle]
#[inline(never)]

fn important_func(&self) -> u32 {
   ...
}
```

then `important_func` will not be inlined and you can break on
`important_func` in gdb. The code itself will still be assembly, but
you can usually piece together what's happening by keeping the source
code alongside. Note that it also helps a lot to use the above
attributes on functions that your function calls -- otherwise figuring
out if the instructions are the function or its callees can be
difficult.
