extern "C"
{
#include "unity_fixture.h"
};

#include "NeoPixelSpy.h"

#include "LedMatrixInterface.h"
#include "LedMatrixDisplay.h"

TEST_GROUP_RUNNER(Display) {
    RUN_TEST_CASE(Display, init);
    RUN_TEST_CASE(Display, TurnOn1Letter);
}

TEST_GROUP(Display);

TEST_SETUP(Display) {}

TEST_TEAR_DOWN(Display) {}

TEST(Display, init) {
    NoePixelSpy neopixel;
    LedMatrixDisplay the_display;

    the_display.init(&neopixel);
}

TEST(Display, TurnOn1Letter) {
//     uint8_t fakeLedMatrix[114];
//     FakeNeoPixel neopixel;
//     LedMatrixDisplay the_display;
//     the_display.init();
//     the_display.turnOn(0, WHITE);
//     TEST_ASSERT_EQUAL_HEX8(WHITE, fakeLedMatrix[0]);
}