Follow the instruction from https://github.com/esp-rs/esp-idf-template to create similar project from template.

````
. ~/export-esp.sh
cargo build
espflash /dev/tty.usbserial-2110 target/xtensa-esp32-espidf/debug/hello-world
espmonitor /dev/tty.usbserial-2110
````