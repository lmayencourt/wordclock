#ifndef __CONFIG_SERVER_HPP
#define __CONFIG_SERVER_HPP

#include <WiFi.h>
#include <ESPAsyncWebServer.h>

#include "FlashFs.hpp"

  const char* config_ssid = "WordClock";
  // Set web server port number to 80
  AsyncWebServer config_server(80);

  // Variable to store the HTTP request
  String header;

#define CONFIG_INPUT(arg) ("input_" #arg)
#define CONFIG_FILE(arg) ("/config_" #arg ".txt")
#define CONFIG_ELEM(arg) {CONFIG_INPUT(arg), CONFIG_FILE(arg)}

typedef struct config_time {
    int hour;
    int min;
} config_time_t;

typedef struct night_mode_times {
  bool is_valide;
  config_time_t start;
  config_time_t end;
} night_mode_times_t;

struct configuration_t {
    char wifi_ssid[40];
    char wifi_password[40];
    bool summer_time;
    char city[20];
    char night_mode_start[10];
    char night_mode_end[10];
    night_mode_times_t night_mode;
};

const char* config_elements[][2] = {
    CONFIG_ELEM(wifi_ssid),
    CONFIG_ELEM(wifi_password),
    CONFIG_ELEM(summer_time),
    CONFIG_ELEM(city),
    CONFIG_ELEM(night_mode_start),
    CONFIG_ELEM(night_mode_end),
};

  // HTML web page to handle 3 input fields (inputString, inputInt, inputFloat)
  const char index_html[] PROGMEM = R"rawliteral(
<!DOCTYPE HTML>
  <html>
  <style>
    .tooltip {
    position: relative;
    display: inline-block;
    border-bottom: 1px dotted black;
    }
    .tooltip .tooltiptext {
        visibility: hidden;
        width: 120px;
        background-color: black;
        color: #fff;
        text-align: center;
        border-radius: 6px;
        padding: 5px 0;

        /* Position the tooltip */
        position: absolute;
        z-index: 1;
    }

    .tooltip:hover .tooltiptext {
        visibility: visible;
    }
  </style>
  <head>
    <title>Word-Clock</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <script>
      function submitMessage() {
        alert("Saved value to ESP SPIFFS");
        setTimeout(function(){ document.location.reload(false); }, 500);   
      }
    </script></head>
  <body>
    <h1>Word-Clock configuration</h1>
    <form action="/get" target="hidden-form">
      Wifi SSID (name): <input type="text" name="input_wifi_ssid"><br><br>
      Wifi password: <input type="text" name="input_wifi_password"><br><br>
      Summer/winter time:
      <select name="Summer time" id="summer_time">
        <option value="yes">yes</option>
        <option value="no">no</option>
      </select><br><br>
      City (for weather forcast): <input type="text" name="input_city"><br><br>
      Night-mode starts at: <input type="text" name="input_night_mode_start">
        <div class="tooltip">&#9432;
          <span class="tooltiptext">E.g. 12:00 <br>
          Must be between 12:00 and 23:59</span>
       </div><br><br>
      Night-mode ends at: <input type="text" name="input_night_mode_end">
      <div class="tooltip">&#9432;
          <span class="tooltiptext">E.g. 08:00 <br>
          Must be between 00:00 and 12:00</span>
       </div><br><br>
      <input type="submit" value="Submit" onclick="submitMessage()">
    </form><br>
    <iframe style="display:none" name="hidden-form"></iframe>
  </body>
</html>)rawliteral";

  /* Same inputs form is used for other format.
   * Value is stored in char* and must be converted back with:
   * .toInt(), .toFloat(), ...
   * int yourInputInt = Flash_fs::readFile(SPIFFS, "/inputInt.txt").toInt();
   * float yourInputFloat = Flash_fs::readFile(SPIFFS, "/inputFloat.txt").toFloat();
   */

class ConfigServer
{
private:
  static struct configuration_t config;

  static void notFound(AsyncWebServerRequest *request) {
    request->send(404, "text/plain", "Not found");
  }

  static bool convertCharToTime(char* in, int* h, int* m) {
    if (strlen(in) != 5) {
        return false;
    }

    char hour_c[3], min_c[3];
    strncpy(hour_c, in, 2);
    strncpy(min_c, &in[3], 2);
    *h = atoi(hour_c);
    *m = atoi(min_c);

    if (*h < 0 || *h > 23) {
        return false;
    }
    if (*m < 0 || *m > 59) {
        return false;
    }
    return true;
}

  static void nightModeConvert(void) {
      config.night_mode.is_valide = convertCharToTime(config.night_mode_start, &config.night_mode.start.hour, &config.night_mode.start.min);
      config.night_mode.is_valide &= convertCharToTime(config.night_mode_end, &config.night_mode.end.hour, &config.night_mode.end.min);
      if (config.night_mode.end.hour > 12) {
          config.night_mode.is_valide = false;
      }
      if (config.night_mode.start.hour < 12) {
          config.night_mode.is_valide = false;
      }
  }

public:
  static void init() {

    // Start configuration server
    // Connect to Wi-Fi network with SSID and password
    Serial.print("Setting AP (Access Point)…");
    // Remove the password parameter, if you want the AP (Access Point) to be open
    WiFi.softAP(config_ssid);

    // Send web page with input fields to client
    config_server.on("/", HTTP_GET, [](AsyncWebServerRequest *request){
      request->send_P(200, "text/html", index_html);
    });

  // Send a GET request to <ESP_IP>/get?inputString=<inputMessage>
    config_server.on("/get", HTTP_GET, [] (AsyncWebServerRequest *request) {
      String inputMessage;
      size_t config_nbr = sizeof(config_elements) / sizeof(const char*[2]);
      for (size_t i=0; i<config_nbr; i++) {
        // GET inputString value on <ESP_IP>/get?inputString=<inputMessage>
        if (request->hasParam(config_elements[i][0])) {
          inputMessage = request->getParam(config_elements[i][0])->value();
          Flash_fs::writeFile(SPIFFS, config_elements[i][1], inputMessage.c_str());
        }
        else {
          inputMessage = "No message sent";
        }
        Serial.println(inputMessage);
      }
      request->send(200, "text/text", inputMessage);
    });
    config_server.onNotFound(notFound);
    config_server.begin();
  }

  static void disconnect() {
    WiFi.softAPdisconnect();
  }

	static bool configurationIsValid() {
		if (config.wifi_ssid[0] != '\0' && config.wifi_password[0] != '\0') {
			return true;
		} else {
			return false;
		}
	}

	static struct configuration_t* readConfiguration() {
    listAllFiles();

    String tmp = Flash_fs::readFile(SPIFFS, CONFIG_FILE(wifi_ssid));
    Serial.print(">> ");
    Serial.print(CONFIG_FILE(wifi_ssid));
    tmp.toCharArray(config.wifi_ssid, sizeof(config.wifi_ssid));
    Serial.println(config.wifi_ssid);

		tmp = Flash_fs::readFile(SPIFFS, CONFIG_FILE(wifi_password));
		Serial.print(">> ");
    Serial.print(CONFIG_FILE(wifi_password));
		tmp.toCharArray(config.wifi_password, sizeof(config.wifi_password));
		Serial.println(config.wifi_password);

    tmp = Flash_fs::readFile(SPIFFS, CONFIG_FILE(summer_time));
		Serial.print(">> ");
    Serial.print(CONFIG_FILE(summer_time));
		// tmp.toCharArray(config.summer_time, sizeof(config.summer_time));
		Serial.println(config.summer_time);

    tmp = Flash_fs::readFile(SPIFFS, CONFIG_FILE(city));
		Serial.print(">> ");
    Serial.print(CONFIG_FILE(city));
		tmp.toCharArray(config.city, sizeof(config.city));
		Serial.println(config.city);

    tmp = Flash_fs::readFile(SPIFFS, CONFIG_FILE(night_mode_start));
		Serial.print(">> ");
    Serial.print(CONFIG_FILE(night_mode_start));
		tmp.toCharArray(config.night_mode_start, sizeof(config.night_mode_start));
		Serial.println(config.night_mode_start);
    tmp = Flash_fs::readFile(SPIFFS, CONFIG_FILE(night_mode_end));
		Serial.print(">> ");
    Serial.print(CONFIG_FILE(night_mode_end));
		tmp.toCharArray(config.night_mode_end, sizeof(config.night_mode_end));
		Serial.println(config.night_mode_end);

    // Populate the night mode start and end times
    nightModeConvert();

    return &config;
	}

  static void listAllFiles(){
    File root = SPIFFS.open("/");
    File file = root.openNextFile();

    Serial.println("Files on system: ");
    while(file){
        Serial.print("|- ");
        Serial.println(file.name());
        file = root.openNextFile();
    }
  }

  static void clearConfig() {
    File root = SPIFFS.open("/");
    File file = root.openNextFile();

    while(file){
        Serial.print("Clearing ");
        Serial.println(file.name());
        SPIFFS.remove(file.name());
        file = root.openNextFile();
    }
  }

};

struct configuration_t ConfigServer::config;

#endif // __CONFIG_SERVER_HPP
