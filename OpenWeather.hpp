#ifndef __OPEN_WEATHER_HPP
#define __OPEN_WEATHER_HPP

/*
  Rui Santos
  Complete project details at Complete project details at https://RandomNerdTutorials.com/esp32-http-get-open-weather-map-thingspeak-arduino/

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files.

  The above copyright notice and this permission notice shall be included in all
  copies or substantial portions of the Software.
*/

#include <HTTPClient.h>
#include <Arduino_JSON.h>

// Your Domain name with URL path or IP address with path
String openWeatherMapApiKey = "408a8f5915c6328266508b72aa7c3c56";

String city = "Bern";
// String countryCode = "PT";

#endif // __OPEN_WEATHER_HPP