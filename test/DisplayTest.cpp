extern "C"
{
#include "unity_fixture.h"
};

#include "LedMatrixDisplay.h"

TEST_GROUP_RUNNER(Display) {
    RUN_TEST_CASE(Display, init);
}

TEST_GROUP(Display);

TEST_SETUP(Display) {}

TEST_TEAR_DOWN(Display) {}

TEST(Display, init) {
    LedMatrixDisplay the_display;
    the_display.init();
}
