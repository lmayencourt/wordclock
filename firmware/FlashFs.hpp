#ifndef __FLASH_FS_HPP
#define __FLASH_FS_HPP

#include <SPIFFS.h>

class Flash_fs {
public:

  static String readFile(fs::FS &fs, const char * path){
    Serial.printf("Reading file: %s\r\n", path);
    File file = fs.open(path, "r");
    if(!file || file.isDirectory()){
      Serial.println("- empty file or failed to open file");
      return String();
    }
    Serial.println("- read from file:");
    String fileContent;
    while(file.available()){
      fileContent+=String((char)file.read());
    }
    Serial.println(fileContent);
    return fileContent;
  }

  static void writeFile(fs::FS &fs, const char * path, const char * message){
    Serial.printf("Writing file: %s\r\n", path);
    File file = fs.open(path, "w");
    if(!file){
      Serial.println("- failed to open file for writing");
      return;
    }
    if(file.print(message)){
      Serial.println("- file written");
    } else {
      Serial.println("- write failed");
    }
  }

// Replaces placeholder with stored values
  String processor(const String& var){
    //Serial.println(var);
    if(var == "inputString"){
      return readFile(SPIFFS, "/inputString.txt");
    }
    else if(var == "inputInt"){
      return readFile(SPIFFS, "/inputInt.txt");
    }
    else if(var == "inputFloat"){
      return readFile(SPIFFS, "/inputFloat.txt");
    }
    return String();
  }

  static void init() {
    // Initialize SPIFFS
    if(!SPIFFS.begin(true)){
      Serial.println("An Error has occurred while mounting SPIFFS");
      return;
    }
  }

};

#endif // __FLASH_FS_HPP

// void loop() {
//   // To access your stored values on inputString, inputInt, inputFloat
//   String yourInputString = readFile(SPIFFS, "/inputString.txt");
//   Serial.print("*** Your inputString: ");
//   Serial.println(yourInputString);
  
//   int yourInputInt = readFile(SPIFFS, "/inputInt.txt").toInt();
//   Serial.print("*** Your inputInt: ");
//   Serial.println(yourInputInt);
  
//   float yourInputFloat = readFile(SPIFFS, "/inputFloat.txt").toFloat();
//   Serial.print("*** Your inputFloat: ");
//   Serial.println(yourInputFloat);
//   delay(5000);
// }