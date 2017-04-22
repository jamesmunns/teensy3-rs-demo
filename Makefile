build:
	xargo build --release --target thumbv7em-none-eabi

flash: build
	arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/release/teensy3-rs-demo target/hex
	teensy-loader-cli -w -s --mcu=mk64fx512 target/hex

monitor:
	screen /dev/ttyACM0 115200

clean:
	xargo clean
	git clean -Xf
