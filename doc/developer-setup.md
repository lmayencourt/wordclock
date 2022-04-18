- Arduino 1.8.16

Board is ESP32 Dev Module

Include the libraries provided as `.zip` in `firmware/libraries/`:
- Sketch > Include Library > Add .zip Library.


# Release procedure
- Export the binary with `Croquis > Exporter les binaires compiler`.
- Copy the `firmware/firmware.ino.esp32.bin` to `firmware.bin`.
- Bump the version number in `version.txt`.
- Bump the version number in `FirmwareOverTheAir.hpp`.
- Merge into the `release` branch to make the firmware available to all clocks.


# Library needed
- ESP32 arduino: https://github.com/espressif/arduino-esp32.git v1.0.6
- Adafruit Neopixel: https://github.com/adafruit/Adafruit_NeoPixel.git v1.10.4
- Asynchonus web-server: https://github.com/me-no-dev/ESPAsyncWebServer.git
- AsyncTCP : https://github.com/me-no-dev/AsyncTCP.git