extern "C"
{
#include "unity_fixture.h"
};

#include "NeoPixelSpy.h"

#include "LedMatrixInterface.h"
#include "LedMatrixDisplay.h"

#include "string.h"
#define NBR_OF_PIXEL 114

NeoPixelSpy neopixel;
LedMatrixDisplay the_display;

TEST_GROUP_RUNNER(Display) {
    RUN_TEST_CASE(Display, init);
    RUN_TEST_CASE(Display, TurnOn1Letter);
    RUN_TEST_CASE(Display, TurnOff1Letter);
    RUN_TEST_CASE(Display, TurnOnMultiple);
    RUN_TEST_CASE(Display, TurnOffMultiple);
    RUN_TEST_CASE(Display, TurnOnAll);
    RUN_TEST_CASE(Display, TurnOffAll);
    RUN_TEST_CASE(Display, TurnOnOutOfBound);
    RUN_TEST_CASE(Display, DisplayStartup);
    RUN_TEST_CASE(Display, DisplayError);
    RUN_TEST_CASE(Display, DisplayConfigState);
    RUN_TEST_CASE(Display, DisplayTimeNoon);
    RUN_TEST_CASE(Display, DisplayAllHours);
}

TEST_GROUP(Display);

char error_msg[30];

void checkRangeIs(uint8_t start, uint8_t nbr, uint32_t value) {
    for (int i=start; i<(start+nbr); i++) {
        sprintf(error_msg, "idx %d", i);
        TEST_ASSERT_EQUAL_HEX32_MESSAGE(value, neopixel.getPixel(i), error_msg);
    }
}

TEST_SETUP(Display) {
    neopixel.init(NBR_OF_PIXEL);
    the_display.init(&neopixel, NBR_OF_PIXEL);
    memset(error_msg, 0, sizeof(error_msg));
}

TEST_TEAR_DOWN(Display) {
    the_display.turnOffAll();
}

TEST(Display, init) {
}

TEST(Display, TurnOn1Letter) {
    the_display.turnOn(0, WHITE);
    TEST_ASSERT_EQUAL_HEX32(WHITE, neopixel.getPixel(0));
}

TEST(Display, TurnOff1Letter) {
    the_display.turnOn(0, WHITE);
    the_display.turnOff(0);
    TEST_ASSERT_EQUAL_HEX32(OFF, neopixel.getPixel(0));
}

TEST(Display, TurnOnMultiple) {
    the_display.turnOn(0, WHITE);
    the_display.turnOn(1, RED);
    the_display.turnOn(113, BLUE);
    TEST_ASSERT_EQUAL_HEX32(WHITE, neopixel.getPixel(0));
    TEST_ASSERT_EQUAL_HEX32(RED, neopixel.getPixel(1));
    TEST_ASSERT_EQUAL_HEX32(BLUE, neopixel.getPixel(113));
}

TEST(Display, TurnOffMultiple) {
    the_display.turnOn(0, WHITE);
    the_display.turnOn(50, RED);
    the_display.turnOn(113, BLUE);
    the_display.turnOff(0);
    the_display.turnOff(50);
    the_display.turnOff(113);
    TEST_ASSERT_EQUAL_HEX32(OFF, neopixel.getPixel(0));
    TEST_ASSERT_EQUAL_HEX32(OFF, neopixel.getPixel(50));
    TEST_ASSERT_EQUAL_HEX32(OFF, neopixel.getPixel(113));
}

TEST(Display, TurnOnOutOfBound) {
    the_display.turnOn(NBR_OF_PIXEL+1, WHITE);
    TEST_ASSERT_TRUE(neopixel.outOfBoundDetected());
    neopixel.clear();
    the_display.turnOn(UINT8_MAX-1, WHITE);
    TEST_ASSERT_TRUE(neopixel.outOfBoundDetected());
    neopixel.clear();
}

TEST(Display, TurnOnAll) {
    the_display.turnOnAll(WHITE);
    checkRangeIs(0,NBR_OF_PIXEL, WHITE);
}

TEST(Display, TurnOffAll) {
    the_display.turnOnAll(WHITE);
    the_display.turnOffAll();
    checkRangeIs(0,NBR_OF_PIXEL, OFF);
}

TEST(Display, DisplayStartup) {
    the_display.displayState(STATE_STARTUP);
    checkRangeIs(LEDMATRIX_STATE_DOT_IDX, 1, GREEN);
    checkRangeIs(LEDMATRIX_STATE_DOT_IDX+1, NBR_OF_PIXEL-1, OFF);
}

TEST(Display, DisplayConfigState) {
    the_display.displayState(STATE_CONFIG);
    checkRangeIs(LEDMATRIX_STATE_DOT_IDX, 2, BLUE);
    checkRangeIs(LEDMATRIX_STATE_DOT_IDX+2, NBR_OF_PIXEL-2, OFF);
}

TEST(Display, DisplayError) {
    the_display.displayState(STATE_ERROR);
    checkRangeIs(LEDMATRIX_STATE_DOT_IDX, LEDMATRIX_STATE_DOT_NBR, RED);
    checkRangeIs(LEDMATRIX_STATE_DOT_IDX+4, NBR_OF_PIXEL-4, OFF);
}

TEST(Display, DisplayTimeNoon) {
    uint8_t hour = 12;
    uint8_t minute = 0;
    const char *expected_string = "ESISCHZWOIUHR";
    the_display.displayTime(hour, minute);
    checkRangeIs(4, 3, WHITE);
    sprintf(error_msg, "expected: %s", neopixel.toString());
    TEST_ASSERT_EQUAL_CHAR_ARRAY_MESSAGE(expected_string, neopixel.toString(), sizeof(expected_string), error_msg);
}

TEST(Display, DisplayAllHours) {
    uint8_t minute = 0;
    const char expected_string[12][20] = {
        "ESISCHEISUHR",
        "ESISCHZWOIUHR",
        "ESISCHDRUUHR",
        "ESISCHVIERIUHR",
        "ESISCHFUFIUHR",
        "ESISCHSACHSIUHR",
        "ESISCHSIBNIUHR",
        "ESISCHACHTIUHR",
        "ESISCHNUNIUHR",
        "ESISCHZANIUHR",
        "ESISCHEUFIUHR",
        "ESISCHZWOUFIUHR",
    };
    for (uint8_t hour=1; hour<=12; hour++) {
        the_display.displayTime(hour, minute);
        sprintf(error_msg, "hour %d: %s", hour, neopixel.toString());
        TEST_ASSERT_EQUAL_CHAR_ARRAY_MESSAGE(expected_string[hour-1], neopixel.toString(), sizeof(expected_string[hour-1]), error_msg);
    }
}