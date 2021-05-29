#ifndef __FOTA_HPP
#define __FOTA_HPP

/** From example:
   httpUpdateSecure.ino
    Created on: 16.10.2018 as an adaptation of the ESP8266 version of httpUpdate.ino
*/

#include <WiFi.h>
#include <WiFiMulti.h>

#include <HTTPClient.h>
#include <HTTPUpdate.h>
// for version
#include <WiFiClientSecure.h>

#include <time.h>

#define FIRMWARE_URL "https://raw.githubusercontent.com/lmayencourt/wordclock/released/firmware.bin"
#define FIRMWARE_VERSION_URL "https://raw.githubusercontent.com/lmayencourt/wordclock/released/version.txt"

class Fota {
private:

    static String firmwareVersion;

    //static WiFiMulti WiFiMulti;
    /**
     * This is lets-encrypt-x3-cross-signed.pem
     */
    static const char * rootCACertificate;

public:
    // Set time via NTP, as required for x.509 validation
    static void setClock() {
        configTime(0, 0, "pool.ntp.org", "time.nist.gov");  // UTC

        Serial.print(F("Waiting for NTP time sync: "));
        time_t now = time(nullptr);
        while (now < 8 * 3600 * 2) {
            yield();
            delay(500);
            Serial.print(F("."));
            now = time(nullptr);
        }

        Serial.println(F(""));
        struct tm timeinfo;
        gmtime_r(&now, &timeinfo);
        Serial.print(F("Current time: "));
        Serial.print(asctime(&timeinfo));
    }

    static String currentVersion() {
        return firmwareVersion;
    }

/* From github:
 *  https://github.com/programmer131/ESP8266_ESP32_SelfUpdate/blob/master/esp32_ota/esp32_ota.ino
 */
    static bool newVersionAvailable(void) {
        String payload;
        int httpCode;
        String fwurl = "";
        fwurl += FIRMWARE_VERSION_URL;
        fwurl += "?";
        fwurl += String(rand());
        Serial.println(fwurl);
        WiFiClientSecure * client = new WiFiClientSecure;

        if (client) 
        {
            client -> setCACert(rootCACertificate);

            // Add a scoping block for HTTPClient https to make sure it is destroyed before WiFiClientSecure *client is 
            HTTPClient https;

            if (https.begin( * client, fwurl)) 
            { // HTTPS      
            Serial.print("[HTTPS] GET...\n");
            // start connection and send HTTP header
            delay(100);
            httpCode = https.GET();
            delay(100);
            if (httpCode == HTTP_CODE_OK) // if version received
            {
                payload = https.getString(); // save received version
            } else {
                Serial.print("error in downloading version file:");
                Serial.println(httpCode);
            }
            https.end();
            }
            delete client;
        }
            
        if (httpCode == HTTP_CODE_OK) // if version received
        {
            payload.trim();
            if (payload.equals(firmwareVersion)) {
            Serial.printf("\nDevice already on latest firmware version:%s\n", firmwareVersion);
            return false;
            } 
            else 
            {
            Serial.println(payload);
            Serial.println("New firmware detected");
            return true;
            }
        } 
        return false;
    }

    static void doFota() {
        // wait for WiFi connection
        // if ((WiFiMulti.run() == WL_CONNECTED)) {

            // setClock();

            // WiFi.mode(WIFI_STA);
            // WiFiMulti.addAP("SolNet-1061741", "03539219049142703605");

            WiFiClientSecure client;
            client.setCACert(rootCACertificate);

            // Reading data over SSL may be slow, use an adequate timeout
            client.setTimeout(12000 / 1000); // timeout argument is defined in seconds for setTimeout

            // The line below is optional. It can be used to blink the LED on the board during flashing
            // The LED will be on during download of one buffer of data from the network. The LED will
            // be off during writing that buffer to flash
            // On a good connection the LED should flash regularly. On a bad connection the LED will be
            // on much longer than it will be off. Other pins than LED_BUILTIN may be used. The second
            // value is used to put the LED on. If the LED is on with HIGH, that value should be passed
            // httpUpdate.setLedPin(LED_BUILTIN, HIGH);

            t_httpUpdate_return ret = httpUpdate.update(client, FIRMWARE_URL);
            // Or:
            //t_httpUpdate_return ret = httpUpdate.update(client, "server", 443, "file.bin");

            switch (ret) {
                case HTTP_UPDATE_FAILED:
                    Serial.printf("HTTP_UPDATE_FAILED Error (%d): %s\n", httpUpdate.getLastError(), httpUpdate.getLastErrorString().c_str());
                    break;

                case HTTP_UPDATE_NO_UPDATES:
                    Serial.println("HTTP_UPDATE_NO_UPDATES");
                    break;

                case HTTP_UPDATE_OK:
                    Serial.println("HTTP_UPDATE_OK");
                    break;
            }
        }
    // }
};

//WiFiMulti Fota::WiFiMulti;
String Fota::firmwareVersion = {"1.1"};
const char * Fota::rootCACertificate = \
    "-----BEGIN CERTIFICATE-----\n"
    "MIIDxTCCAq2gAwIBAgIQAqxcJmoLQJuPC3nyrkYldzANBgkqhkiG9w0BAQUFADBs\n"
    "MQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\n"
    "d3cuZGlnaWNlcnQuY29tMSswKQYDVQQDEyJEaWdpQ2VydCBIaWdoIEFzc3VyYW5j\n"
    "ZSBFViBSb290IENBMB4XDTA2MTExMDAwMDAwMFoXDTMxMTExMDAwMDAwMFowbDEL\n"
    "MAkGA1UEBhMCVVMxFTATBgNVBAoTDERpZ2lDZXJ0IEluYzEZMBcGA1UECxMQd3d3\n"
    "LmRpZ2ljZXJ0LmNvbTErMCkGA1UEAxMiRGlnaUNlcnQgSGlnaCBBc3N1cmFuY2Ug\n"
    "RVYgUm9vdCBDQTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAMbM5XPm\n"
    "+9S75S0tMqbf5YE/yc0lSbZxKsPVlDRnogocsF9ppkCxxLeyj9CYpKlBWTrT3JTW\n"
    "PNt0OKRKzE0lgvdKpVMSOO7zSW1xkX5jtqumX8OkhPhPYlG++MXs2ziS4wblCJEM\n"
    "xChBVfvLWokVfnHoNb9Ncgk9vjo4UFt3MRuNs8ckRZqnrG0AFFoEt7oT61EKmEFB\n"
    "Ik5lYYeBQVCmeVyJ3hlKV9Uu5l0cUyx+mM0aBhakaHPQNAQTXKFx01p8VdteZOE3\n"
    "hzBWBOURtCmAEvF5OYiiAhF8J2a3iLd48soKqDirCmTCv2ZdlYTBoSUeh10aUAsg\n"
    "EsxBu24LUTi4S8sCAwEAAaNjMGEwDgYDVR0PAQH/BAQDAgGGMA8GA1UdEwEB/wQF\n"
    "MAMBAf8wHQYDVR0OBBYEFLE+w2kD+L9HAdSYJhoIAu9jZCvDMB8GA1UdIwQYMBaA\n"
    "FLE+w2kD+L9HAdSYJhoIAu9jZCvDMA0GCSqGSIb3DQEBBQUAA4IBAQAcGgaX3Nec\n"
    "nzyIZgYIVyHbIUf4KmeqvxgydkAQV8GK83rZEWWONfqe/EW1ntlMMUu4kehDLI6z\n"
    "eM7b41N5cdblIZQB2lWHmiRk9opmzN6cN82oNLFpmyPInngiK3BD41VHMWEZ71jF\n"
    "hS9OMPagMRYjyOfiZRYzy78aG6A9+MpeizGLYAiJLQwGXFK3xPkKmNEVX58Svnw2\n"
    "Yzi9RKR/5CYrCsSXaQ3pjOLAEFe4yHYSkVXySGnYvCoCWw9E1CAx2/S6cCZdkGCe\n"
    "vEsXCS+0yx5DaMkHJ8HSXPfqIbloEpw8nL+e/IBcm2PN7EeqJSdnoDfzAIJ9VNep\n"
    "+OkuE6N36B9K\n"
    "-----END CERTIFICATE-----\n";

#endif // __FOTA_HPP