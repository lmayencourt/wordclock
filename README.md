# WordClock (Rust)

## About
Display the time in swiss-german, on a custom made hardware.
The clock get the time from the internet. A network connection with WiFI is needed.

## Tips
Follow the instruction from https://github.com/esp-rs/esp-idf-template to create similar project from template.

````
. ~/export-esp.sh
cargo build
espflash /dev/tty.usbserial-2110 target/xtensa-esp32-espidf/debug/hello-world --flash-freq 80M --flash-size 4MB --flash-mode DIO --speed 921600
espmonitor /dev/tty.usbserial-2110
````

Add `--monitor` option to `espflash` to directly open monitoring after flashing.

## Setup rust analyzer for ESP32
setup the environment variables in `.cargo/config.toml` to match the `export-esp.sh` values.

## Project management
A [GitHub project](https://github.com/users/lmayencourt/projects/1/) is used for tasks management.

## License
Licensed under MIT license ([LICENSE-MIT](LICENSE.txt) or http://opensource.org/licenses/MIT)