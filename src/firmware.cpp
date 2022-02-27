#include <Arduino.h>

///////////////////////////////////////////////////
// Controller
// #include "controller.hpp"
// Controller controller;

void setup() {
  // controller.init();
  pinMode(2, OUTPUT);
}

void loop() {
  // controller.main();
  digitalWrite(2, HIGH);
  delay(1000);
  digitalWrite(2, LOW);
  delay(1000);
}
