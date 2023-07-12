# Release process

## Release tests
1. Run the unit-test: `cargo test`
2. Run the manual-test:
   1. Build the binary in release mode: `cargo xbuild --release`
   2. Flash the binary on the board: `espflash /dev/tty.usbserial-0001 crates/cross_compiled/target/xtensa-esp32-espidf/release/cross_compiled --flash-freq 80M --flash-size 4MB --fla
sh-mode DIO --speed 921600 --monitor --partition-table crates/cross_compiled/esp32_ota_partitions.csv`

### Manual tests
1. Invalid configuration -> start of configuration server `WordClock Configuration`.
2. HTML page is displayed properly.
3. Entering a valid configuration is parsed, and stored in persistent memory.
4. Valid configuration -> the system switch to display time state.
5. The time displayed is correct.
6. Manuel reset of the device with 'Reset' button loads configuration from persistent memory and start displaying time properly.
7. Pressing (long or short) the 'Enter' button switch the system to the menu.
8. Long press on FOTA menu triggers FOTA.
9. FOTA download succeed with log: `I (115661) application: Update ready, restart device`.
10. System restart in new version.

## Generate release binary
1. Build firmware in release mode: `cargo xbuild --release` 
2. Build the tagged OTA image: `cargo generate_ota release`
3. Commit and push changes to `main` branch.
4. Update the `released` branch to latest `master`.