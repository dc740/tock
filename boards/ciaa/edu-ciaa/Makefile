# Makefile for building the tock kernel for the edu-ciaa

TARGET=thumbv7em-none-eabi
PLATFORM=edu-ciaa

include ../../Makefile.common

TOCKLOADER=tockloader
OOCD=openocd
OOCD_SCRIPT=./ftdi_lpc4337.cfg

# Where in the lpc43xx flash to load the kernel with `tockloader`

# Upload programs over uart with tockloader
ifdef PORT
  TOCKLOADER_GENERAL_FLAGS += --port $(PORT)
endif

# OpenOCD vars
BOARD=edu-ciaa
TOCK_ARCH=cortex-m4
TOCKLOADER_OPENOCD_FLAGS = --openocd --arch $(TOCK_ARCH) --board $(BOARD) --openocd-board $(OOCD_SCRIPT) -c "init; halt 0; flash write_image erase unlock $< 0x1A000000 bin; reset run; shutdown;"

# Upload the kernel over JTAG
.PHONY: flash
flash: target/$(TARGET)/release/$(PLATFORM).bin
	$(Q)$(OOCD) -f $(OOCD_SCRIPT) \
		-c "init" \
		-c "halt 0" \
		-c "flash write_image erase unlock $< 0x1A000000 bin" \
		-c "reset run" \
		-c "shutdown" 2>&1
	
#$(TOCKLOADER) $(TOCKLOADER_GENERAL_FLAGS) flash $(TOCKLOADER_OPENOCD_FLAGS) $<

# Upload the kernel over serial/bootloader
.PHONY: program
program: target/$(TARGET)/release/$(PLATFORM).hex
	$(error Cannot program over USB)
