#ifndef __CONTROLLER_HPP
#define __CONTROLLER_HPP
 
#include <Arduino.h>

#include <WiFi.h>
bool isConnected() {
  return WiFi.isConnected();
}

bool connectToWifi(char* ssid, char* password) {
  // Connect to Wi-Fi
  Serial.print("Connecting to WiFi: ");
  Serial.print(ssid);
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);
  int attempts = 0;
  while (WiFi.status() != WL_CONNECTED) {
	Serial.print("Connecting...");
    delay(1000);
    attempts++;
    if (attempts > 3) {
      Serial.println("Failed connecting to WiFi");
      return false;
    }
  }

  // Print ESP32 Local IP Address
  WiFi.localIP();
  Serial.println(WiFi.localIP());

  return true;
}

bool disconnectWifi() {
	WiFi.mode(WIFI_OFF);
}

// BOOT bouton as input
#define BOOT_GPIO 0
bool bootButtonPushed() {
	return !digitalRead(BOOT_GPIO);
}

///////////////////////////////////////////////////
// Display
#include "Display.hpp"

Display display;

String display_state, display_log;

void printState(String state, String log) {
	printf("%s: %s", state, log);
}

void UpdateState(String txt) {
	display_state = txt;
	printState(display_state, display_log);
}

void UpdateLog(String txt) {
	display_log = txt;
	printState(display_state, display_log);
}

///////////////////////////////////////////////////
// Flash file storage
#include "FlashFs.hpp"

///////////////////////////////////////////////////
// Network Time via NTP server
#include "NetworkTime.hpp"

///////////////////////////////////////////////////
// Game of life
// #include "Life.hpp"

///////////////////////////////////////////////////
// Webserver for configuration
#include "ConfigServer.hpp"

///////////////////////////////////////////////////
// Firmware over the air
#include "FirmwareOverTheAir.hpp"

///////////////////////////////////////////////////
// Telegram Bot
// Replace with your network credentials
//#include "telegram_bot.hpp"

enum state {
	state_startup,
	state_idle,
	state_config,
	state_clock,
	state_userMenu,
	state_error,
	// state_life,
} state;

enum menu {
	menu_clock,
	menu_fota,
	menu_cleanConfig,
} menu;

RTC_DATA_ATTR int displayed_hour;
RTC_DATA_ATTR int displayed_min;

class Controller
{
private:
	configuration_t *configuration;
public:
	void init() {
		Serial.begin(115200);

		Flash_fs::init();

		// pinMode(ledPin, OUTPUT);
		// digitalWrite(ledPin, ledState);

		// displayUpdateLog("Connected");

		// // Configure life game
		// life.cell_size = 4;
		// life.board_width = SCREEN_WIDTH/life.cell_size;
		// life.board_heigth = SCREEN_HEIGTH/life.cell_size;

		// int count=0;
		// for (int w=0; w<life.board_width; w++)
		// for (int h=0; h<life.board_heigth; h++) {
		// 	life.board[w][h] = random(0,2);
		// 	if (life.board[w][h] == 1) {
		// 	count++;
		// 	}
		// }

		// Serial.println("Life: Initial population:"+count);

		pinMode(BOOT_GPIO, INPUT);

		// Set initial state
		state = state_startup;
	}

	void stateStartup() {
		UpdateState("Startup");

		String version = Fota::currentVersion();
		Serial.println("Starup with v" + version);

		// Check the reset reason
		esp_reset_reason_t reset_reason = esp_reset_reason();
		if (reset_reason == ESP_RST_DEEPSLEEP) {
			// We wake-up from deepsleep, switch config
			// esp_sleep_wakeup_cause_t wakeup_reason;
			// wakeup_reason = esp_sleep_get_wakeup_cause();
			// switch(wakeup_reason)
			// {
			// 	case ESP_SLEEP_WAKEUP_EXT0 : Serial.println("Wakeup caused by external signal using RTC_IO"); break;
			// 	case ESP_SLEEP_WAKEUP_EXT1 : Serial.println("Wakeup caused by external signal using RTC_CNTL"); break;
			// 	case ESP_SLEEP_WAKEUP_TIMER : Serial.println("Wakeup caused by timer"); break;
			// 	case ESP_SLEEP_WAKEUP_TOUCHPAD : Serial.println("Wakeup caused by touchpad"); break;
			// 	case ESP_SLEEP_WAKEUP_ULP : Serial.println("Wakeup caused by ULP program"); break;
			// 	default : Serial.printf("Wakeup was not caused by deep sleep: %d\n",wakeup_reason); break;
			// }
			Serial.println("Wakeup from deep-sleep");
			state = state_clock;
			// return;
		} else {
			// Run init needed after power off
			display.init();
			// Check matrix display
			display.test(DISPLAY_TESTS_TIME);
			display.displayStartup();
		}

		// Check if configuration is valid
		// TODO: put config in RTC_DATA and only read if startup is not deepsleep.
		configuration = ConfigServer::readConfiguration();

		if (!ConfigServer::configurationIsValid()) {
			Serial.println("Config not valid, start config-server");
			ConfigServer::init();
			state = state_config;
		} else {
			Serial.println("Config valid, start clock");
			state = state_clock;
		}

	}

	void stateConfigure() {
		UpdateState("Config");

		display.displayConfig();
		delay(5000);

		configuration = ConfigServer::readConfiguration();
		if (ConfigServer::configurationIsValid()) {
			ConfigServer::disconnect();
			state = state_clock;
		}
	}

	void stateCLock() {
		Serial.println("state: clock");

		// Check if time is already set
		struct tm timeinfo;
		NetworkTime::initTZ();
		bool timeValid = getLocalTime(&timeinfo);

		if (!timeValid) {
			// Time is not valid yet, ask network
			Serial.println("no valid time, query from network");
			if (!isConnected()) {
				bool connected = connectToWifi(configuration->wifi_ssid, configuration->wifi_password);
				if (!connected) {
					state = state_config;
					return;
				}
			}
			Serial.println("connected");

			NetworkTime::init();

			Serial.println("disconnect wifi");
			disconnectWifi();

			getLocalTime(&timeinfo);
		}

		bool networkSynchNeeded = (timeinfo.tm_hour == 12 && timeinfo.tm_min == 0 ||
									timeinfo.tm_hour == 0 && timeinfo.tm_min == 0);
		if (networkSynchNeeded) {
			// Check FOTA and synch time
			Serial.println("Network synch time reached, connect to network");
			if (!isConnected()) {
				bool connected = connectToWifi(configuration->wifi_ssid, configuration->wifi_password);
				if (!connected) {
					state = state_config;
					return;
				}
			}
			Serial.println("connected");

			// Update network time
			NetworkTime::init();

			// Check for update
			if(Fota::newVersionAvailable()) {
				Serial.println("New version detected, start update...");
				Fota::doFota();
			} else {
				Serial.println("Current version is the latest available");
			}

			Serial.println("disconnect wifi");
			disconnectWifi();
		}

		bool in_night_mode = false;
		if (configuration->night_mode.is_valide) {
			if (timeinfo.tm_hour > configuration->night_mode.start.hour) {
				in_night_mode = true;
			} else if (timeinfo.tm_hour == configuration->night_mode.start.hour) {
				if (timeinfo.tm_min >= configuration->night_mode.start.min) {
					in_night_mode = true;
				}
			}

			if (timeinfo.tm_hour < configuration->night_mode.end.hour) {
				in_night_mode = true;
			} else if (timeinfo.tm_hour == configuration->night_mode.end.hour) {
				if (timeinfo.tm_min <= configuration->night_mode.end.min) {
					in_night_mode = true;
				}
			}
		} else {
			Serial.println("Night mode is not set");
		}

		Serial.print("Time is already  valid: ");
		Serial.print(timeinfo.tm_hour);
		Serial.println(timeinfo.tm_min);
		if (in_night_mode) {
			Serial.print("Night from ");
			Serial.print(configuration->night_mode.start.hour);
			Serial.print(configuration->night_mode.start.min);
			Serial.print(" to ");
			Serial.print(configuration->night_mode.end.hour);
			Serial.println(configuration->night_mode.end.min);
			display.clear();
			display.display();
		} else {
			// Display time
			if (displayed_min != timeinfo.tm_min) {
				displayed_min = timeinfo.tm_min;
				display.displayTime(timeinfo.tm_hour, timeinfo.tm_min);
			}
		}

		// Go to deepsleep
		Serial.println("Go to sleep now");
		esp_sleep_enable_timer_wakeup(15*1000*1000);
		Serial.flush();
		esp_deep_sleep_start();
	}

	void stateMenu() {
		Serial.println("state: menu");

		bool long_push = false;
		bool mutli_push = false;
		int pushed_duration = 0;

		enum menu menu_select = menu_clock;
		bool in_menu = true;

		// Inform that we are in Menu
		display.displayMenu(menu_select);

		// Wait that the button goes down
		while(bootButtonPushed() == true) {
			delay(300);
		}

		while (in_menu) {
			display.displayMenu(menu_select);

			if (bootButtonPushed()) {
				while(bootButtonPushed()) {
					delay(100);
					pushed_duration+=100;
					if (pushed_duration > 100 && pushed_duration < 2000) {
						display.displayProgressBar(pushed_duration/500);
					}
				}

				if (pushed_duration >= 2000) {
					// Long push: exit loop and execute menu action
					Serial.println("long push");
					in_menu = false;
				} else {
					// Short push: change menu
					switch (menu_select) {
						case menu_clock: menu_select = menu_fota; break;
						case menu_fota: menu_select = menu_cleanConfig; break;
						case menu_cleanConfig: menu_select = menu_clock; break;
					}
				}
				pushed_duration = 0;
			}

			delay(150);
		}

		Serial.println("Menu action:");
		switch (menu_select) {
			case menu_clock: state = state_clock; break;
			case menu_fota:
				if (!isConnected()) {
					bool connected = connectToWifi(configuration->wifi_ssid, configuration->wifi_password);
					if (!connected) {
						state = state_error;
						return;
					}
				}
				Serial.println("connected");
				// Check for update
				if(Fota::newVersionAvailable()) {
					Serial.println("New version detected, start update...");
					Fota::doFota();
				} else {
					Serial.println("Current version is the latest available");
				}

				Serial.println("disconnect wifi");
				disconnectWifi();
				break;
			case menu_cleanConfig:
				ConfigServer::clearConfig();
				state = state_startup;
				break;
			default: state = state_clock;
		}
	}

	void stateError() {
		display.clear();
		display.display();
		delay(500);
		display.displayError();
		delay(500);
	}

	void main() {
		Serial.println("Current state: ");
		Serial.println(state);

		// Startup must be executed before anything else
		if (state == state_startup) {
			stateStartup();
		}

		if (!digitalRead(BOOT_GPIO)) {
			Serial.println("boot pushed!");
			state = state_userMenu;
		}

		switch (state) {
		// 	case state_idle: Serial.println("state: idle"); break;
		 	case state_config: stateConfigure(); break;
		 	case state_clock: stateCLock(); break;
			case state_userMenu: stateMenu(); break;
			case state_error: stateError(); break;
		// 	case state_life:
		// 		Serial.println("state: life");
		// 		if (millis() > life.last_gen_timestamp + 250) {
		// 			life_NextGen();
		// 			life_display();
		// 		}
		// 		life.last_gen_timestamp = millis();
		// 	break;
		}
	}

};

#endif //__CONTROLLER_HPP