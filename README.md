# WordClock (Rust)

## About
Display the time in swiss-german, on a custom made hardware.
The clock get the time from the internet. A network connection with WiFI is needed.

## Tips
Follow the instruction from https://github.com/esp-rs/esp-idf-template to create similar project from template.

````
. ~/export-esp.sh
cargo build
espflash /dev/tty.usbserial-2110 target/xtensa-esp32-espidf/debug/hello-world
espmonitor /dev/tty.usbserial-2110
````

## Setup rust analyzer for ESP32
setup the environment variables in `.cargo/config.toml` to match the `export-esp.sh` values.

## TODO list
- [ ] FOTA.
- [ ] Persistent storage for configuration.
- [ ] NeoPixel LEDs driver.
- [ ] Main state-machine.
- [ ] Configuration server.
  - [ ] HTML over WiFi.
  - [ ] BLE.

## License
Licensed under MIT license ([LICENSE-MIT](LICENSE.txt) or http://opensource.org/licenses/MIT)