build:
	@xargo build --release

flash: build
	arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/release/teensy3-rs-demo target/hex
	teensy-loader-cli -w -s --mcu=mk20dx256 target/hex

clean:
	xargo clean
	git clean -Xf
