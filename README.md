# Wordclock

Display the time in swiss-german, on a custom made hardware.
The clock get the time from the internet. A network connection with WiFI is needed.

<img src="./doc/images/leds_matrix_assembled.png" width="300"/> <img src="./doc/images/white.png" width="300"/>

## Configuration

Connect to "WordClock" wifi and go to [http://192.168.4.1](http://192.168.4.1) in a browser. Enter your wifi name (SSID) and your wifi password.
If you want the clock to be off during the night, set the "Night mode" start and end times.

> **Note:** You may need to restart the clock by pressing the "EN" button.

## Menu

To enter the menu, press the "BOOT" button until the "uhr" is displayed (it may take up to 15 secs to reach the menu). A single push of the "BOOT" button changes the menu, a long push (< 2 secs) validate the menu selection and trigger the associated actions:
 * uhr: Go back to time display.
 * eis: Check if a new version of the firmware is available and download it.
 * zwöi: Erase the current configuration and switch back to configuration mode.

## Display meanings

 * 1 green dot: Booting.
 * 2 blue dots: Configuration mode.
 * 1 blinking red dot: Error. Press "EN" bouton to restart the clock.

## Project management
A [GitHub project](https://github.com/users/lmayencourt/projects/1/) is used for tasks management.

## Deploy the firmware to an ESP32 board
````
. ~/export-esp.sh
cargo build
espflash /dev/tty.usbserial-2110 target/xtensa-esp32-espidf/debug/hello-world --flash-freq 80M --flash-size 4MB --flash-mode DIO --speed 921600
espmonitor /dev/tty.usbserial-2110
````

Add `--monitor` option to `espflash` to directly open monitoring after flashing.

Follow the instruction from https://github.com/esp-rs/esp-idf-template to create similar project from template.

## Setup rust analyzer for ESP32
setup the environment variables in `.cargo/config.toml` to match the `export-esp.sh` values.

## Documentation
### Architecture documentation
`mdbook` is used for rendering the architecture documentation in `doc/architecture/`.

It can be generated with:
````
mdbook build
````

To build and open locally the documentation, use:
````
mdbook serve --open
````

## License
Licensed under MIT license ([LICENSE-MIT](LICENSE.txt) or http://opensource.org/licenses/MIT)
