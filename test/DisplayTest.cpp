#include "unity_fixture.h"

TEST_GROUP_RUNNER(Display) {
    RUN_TEST_CASE(Display, init);
}

TEST_GROUP(Display);

TEST_SETUP(Display) {}

TEST_TEAR_DOWN(Display) {}

TEST(Display, init) {
    TEST_ASSERT_TRUE(1);
}
