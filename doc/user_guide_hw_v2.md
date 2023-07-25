# User guide wood clock
This covers the second iteration of the *WordClock*, with custom hardware and firmware v3.x.x

## Display meanings
When you power the device, it will display:
 * 1 dot: Startup.
 * 2 dots: Configuration mode.
 * 1 red cross: Error. Press "Restart" button to restart the clock.
If the startup sequence went successfully, the device will start displaying the time.

### Configuration mode
The device create a WiFi access point called "WordClock Configuration". In order to configure the clock, you must connect to it and access the page [http://192.168.71.1](http://192.168.71.1) in a browser. Enter your wifi name (SSID) and your wifi password.
If you want the clock to be off during the night, set the "Night mode" start and end times.

## Menu
To enter the menu, press the "Enter" button until the first dots is displayed, when the device is displaying the time. A single push of the "Enter" button changes the menu, a long push (< 2 secs) validate the menu selection and trigger the associated actions:
 * 1 dot: Check if a new version of the firmware is available and download it.
 * 2 dots: Erase the current configuration and switch back to configuration mode.
 * 3 dots: Go back to time display.
