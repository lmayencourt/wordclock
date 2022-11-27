#include <Arduino.h>

///////////////////////////////////////////////////
// Controller
#include "controller.hpp"
Controller controller;

void setup() {
  controller.init();
}

void loop() {
  controller.main();
}
