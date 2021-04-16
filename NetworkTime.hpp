#ifndef __NETWORK_TIME_HPP
#define __NETWORK_TIME_HPP

#include "time.h"

#define NTP_SERVER "ntp.metas.ch"

class NetworkTime
{
private:
	static const long  gmtOffset_sec = 3600;
	static int daylightOffset_sec;
	static bool timeValid;

public:

	static void init() {
		// Init and get the time
		daylightOffset_sec = 0;
		configTime(gmtOffset_sec, daylightOffset_sec, NTP_SERVER);
		printLocalTime();
		timeValid = true;
	}

	static void setSummerTime(bool summer_offset) {
		if (summer_offset) {
			daylightOffset_sec = 3600;
		} else {
			daylightOffset_sec = 0;
		}

		configTime(gmtOffset_sec, daylightOffset_sec, NTP_SERVER);
	}

	static bool timeIsValid() {
		return timeValid;
	}

	static void printLocalTime(){
		struct tm timeinfo;
		if(!getLocalTime(&timeinfo)){
			Serial.println("Failed to obtain time");
			return;
		}

		Serial.println(&timeinfo, "%A, %B %d %Y %H:%M:%S");
		Serial.print("Day of week: ");
		Serial.println(&timeinfo, "%A");
		Serial.print("Month: ");
		Serial.println(&timeinfo, "%B");
		Serial.print("Day of Month: ");
		Serial.println(&timeinfo, "%d");
		Serial.print("Year: ");
		Serial.println(&timeinfo, "%Y");
		Serial.print("Hour: ");
		Serial.println(&timeinfo, "%H");
		Serial.print("Hour (12 hour format): ");
		Serial.println(&timeinfo, "%I");
		Serial.print("Minute: ");
		Serial.println(&timeinfo, "%M");
		Serial.print("Second: ");
		Serial.println(&timeinfo, "%S");

		Serial.println("Time variables");
		char timeHour[3];
		strftime(timeHour,3, "%H", &timeinfo);
		Serial.println(timeHour);
		char timeWeekDay[10];
		strftime(timeWeekDay,10, "%A", &timeinfo);
		Serial.println(timeWeekDay);
		Serial.println();
	}
	
};

int NetworkTime::daylightOffset_sec = 0;
bool NetworkTime::timeValid = false;

#endif // __NETWORK_TIME_HPP