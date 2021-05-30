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

// const char* PARAM_WIFI_SSID_STRING = "input_wifi_ssid"; //wifi id
// const char* PARAM_WIFI_PASSWORD_STRING = "input_wifi_password"; //wifi password
// const char* PARAM_WEATHER_CITY_STRING = "input_city"; //Bern
// const char* PARAM_SUMMER_TIME = "input_summer_time"; // yes

#define CONFIG_INPUT(arg) ("input_" #arg)
#define CONFIG_FILE(arg) ("/config_" #arg ".txt")
#define CONFIG_ELEM(arg) {CONFIG_INPUT(arg), CONFIG_FILE(arg)}

struct configuration_t {
      char wifi_ssid[40];
      char wifi_password[40];
      bool summer_time;
      char city[20];
    };

const char* config_elements[][2] = {
    CONFIG_ELEM(wifi_ssid),
    CONFIG_ELEM(wifi_password),
    CONFIG_ELEM(summer_time),
    CONFIG_ELEM(city),
};

  // const char* PARAM_STRING = "inputString";
  // const char* PARAM_INT = "inputInt";
  // const char* PARAM_FLOAT = "inputFloat";

  // HTML web page to handle 3 input fields (inputString, inputInt, inputFloat)
  const char index_html[] PROGMEM = R"rawliteral(
  <!DOCTYPE HTML><html><head>
    <title>Word-Clock</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <script>
      function submitMessage() {
        alert("Saved value to ESP SPIFFS");
        setTimeout(function(){ document.location.reload(false); }, 500);   
      }
    </script></head><body>
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
      <input type="submit" value="Submit" onclick="submitMessage()">
    </form><br>
    <iframe style="display:none" name="hidden-form"></iframe>
  </body></html>)rawliteral";

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
      // // GET inputString value on <ESP_IP>/get?inputString=<inputMessage>
      // if (request->hasParam(PARAM_WIFI_SSID_STRING)) {
      //   inputMessage = request->getParam(PARAM_WIFI_SSID_STRING)->value();
      //   Flash_fs::writeFile(SPIFFS, "/config_wifi_ssid.txt", inputMessage.c_str());
      // }
      // // GET inputString value on <ESP_IP>/get?inputString=<inputMessage>
      // if (request->hasParam(PARAM_WIFI_PASSWORD_STRING)) {
      //   inputMessage = request->getParam(PARAM_WIFI_PASSWORD_STRING)->value();
      //   Flash_fs::writeFile(SPIFFS, "/config_wifi_password.txt", inputMessage.c_str());
      // }
      // // GET inputString value on <ESP_IP>/get?inputString=<inputMessage>
      // if (request->hasParam(PARAM_WEATHER_CITY_STRING)) {
      //   inputMessage = request->getParam(PARAM_WEATHER_CITY_STRING)->value();
      //   Flash_fs::writeFile(SPIFFS, "/config_wheater_city.txt", inputMessage.c_str());
      // }
      // else {
      //   inputMessage = "No message sent";
      // }
      // Serial.println(inputMessage);
      // request->send(200, "text/text", inputMessage);
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


  /*
    //Serial.println("state: config");
    WiFiClient client = config_server.available();   // Listen for incoming clients

    if (client) {                             // If a new client connects,
      Serial.println("New Client.");          // print a message out in the serial port
      String currentLine = "";                // make a String to hold incoming data from the client
      while (client.connected()) {            // loop while the client's connected
        if (client.available()) {             // if there's bytes to read from the client,
          char c = client.read();             // read a byte, then
          Serial.write(c);                    // print it out the serial monitor
          header += c;
          if (c == '\n') {                    // if the byte is a newline character
            // if the current line is blank, you got two newline characters in a row.
            // that's the end of the client HTTP request, so send a response:
            if (currentLine.length() == 0) {
              // HTTP headers always start with a response code (e.g. HTTP/1.1 200 OK)
              // and a content-type so the client knows what's coming, then a blank line:
              client.println("HTTP/1.1 200 OK");
              client.println("Content-type:text/html");
              client.println("Connection: close");
              client.println();
              
              // turns the GPIOs on and off
              if (header.indexOf("GET /26/on") >= 0) {
                Serial.println("GPIO 26 on");
                output26State = "on";
                digitalWrite(output26, HIGH);
              } else if (header.indexOf("GET /26/off") >= 0) {
                Serial.println("GPIO 26 off");
                output26State = "off";
                digitalWrite(output26, LOW);
              } else if (header.indexOf("GET /27/on") >= 0) {
                Serial.println("GPIO 27 on");
                output27State = "on";
                digitalWrite(output27, HIGH);
              } else if (header.indexOf("GET /27/off") >= 0) {
                Serial.println("GPIO 27 off");
                output27State = "off";
                digitalWrite(output27, LOW);
              }
              
              // Display the HTML web page
              client.println("<!DOCTYPE html><html>");
              client.println("<head><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
              client.println("<link rel=\"icon\" href=\"data:,\">");
              // CSS to style the on/off buttons 
              // Feel free to change the background-color and font-size attributes to fit your preferences
              client.println("<style>html { font-family: Helvetica; display: inline-block; margin: 0px auto; text-align: center;}");
              client.println(".button { background-color: #4CAF50; border: none; color: white; padding: 16px 40px;");
              client.println("text-decoration: none; font-size: 30px; margin: 2px; cursor: pointer;}");
              client.println(".button2 {background-color: #555555;}</style></head>");
              
              // Web Page Heading
              client.println("<body><h1>ESP32 Web Server</h1>");
              
              // Display current state, and ON/OFF buttons for GPIO 26  
              client.println("<p>GPIO 26 - State " + output26State + "</p>");
              // If the output26State is off, it displays the ON button       
              if (output26State=="off") {
                client.println("<p><a href=\"/26/on\"><button class=\"button\">ON</button></a></p>");
              } else {
                client.println("<p><a href=\"/26/off\"><button class=\"button button2\">OFF</button></a></p>");
              } 
                 
              // Display current state, and ON/OFF buttons for GPIO 27  
              client.println("<p>GPIO 27 - State " + output27State + "</p>");
              // If the output27State is off, it displays the ON button       
              if (output27State=="off") {
                client.println("<p><a href=\"/27/on\"><button class=\"button\">ON</button></a></p>");
              } else {
                client.println("<p><a href=\"/27/off\"><button class=\"button button2\">OFF</button></a></p>");
              }
              client.println("</body></html>");
              
              // The HTTP response ends with another blank line
              client.println();
              // Break out of the while loop
              break;
            } else { // if you got a newline, then clear currentLine
              currentLine = "";
            }
          } else if (c != '\r') {  // if you got anything else but a carriage return character,
            currentLine += c;      // add it to the end of the currentLine
          }
        }
      }
      // Clear the header variable
      header = "";
      // Close the connection
      client.stop();
      Serial.println("Client disconnected.");
      Serial.println("");
    }
    */

#endif // __CONFIG_SERVER_HPP
