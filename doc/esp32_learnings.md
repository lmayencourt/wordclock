# ESP32 developer learnings

## Reading the ESP32 partition table
Extract the partition table from a ESP32 device FLASH and dump the table to
stdout:
````
esptool.py read_flash 0x8000 0xc00 ptable.img
python3 crates/cross_compiled/.embuild/espressif/esp-idf/release-v4.4/components/partition_table/gen_esp32part.py ptable.img
````

Even better, using `espflash`:
````
espflash partition-table ptable.img
````

## Partition table must be flashed explicitly
When using the `espflash` command to program the device, the `--partition-table`
option should be used to update the device partition table.

````
espflash /dev/tty.usbserial-0001 crates/cross_compiled/target/xtensa-esp32-espidf/debug/cross_compiled --flash-freq 80M --flash-size 4MB --flash-mode DIO --speed 921600 --partition-table crates/cross_compiled/esp32_ota_partitions.csv
````
This is necessary to flash an image with FOTA capabilities.
Omitting to specify the partition-table in the command can allow to flash bigger images, like debug build.

## Partitions restrictions
Partitions must be 0x10000 (64kB) aligned.

## Generate an OTA image
Use the `espflash` tool to convert an `ELF` image to `bin` format:
````
espflash save-image ESP32 --flash-size 2MB crates/cross_compiled/target/xtensa-esp32-espidf/release/cross_compiled ota-test-img/ota_v0.1.0.bin
````

This also allow to see the real size of the image binary in FLASH.

## needed cargo-espflash?
````
cargo install cargo-espflash
````