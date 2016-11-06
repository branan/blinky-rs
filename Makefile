PROJECT=blinky
TARGET=thumbv7em-none-eabi
MODE?=debug
OUTDIR=target/$(TARGET)/$(MODE)
HEX=$(OUTDIR)/$(PROJECT).hex
ELF=$(OUTDIR)/$(PROJECT)

all:: $(HEX)

$(HEX): $(ELF)
	arm-none-eabi-objcopy -R .stack -O ihex $(ELF) $(HEX)

.PHONY: $(ELF)
$(ELF):
ifeq ($(MODE),release)
	~/.cargo/bin/xargo build --target $(TARGET) --release
else
	~/.cargo/bin/xargo build --target $(TARGET)
endif

flash: $(HEX)
	@echo "Reset your Teensy now!"
	teensy-loader-cli -w -mmcu=mk20dx128 $(HEX) -v
