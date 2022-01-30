#include "unity_fixture.h"

TEST_GROUP_RUNNER(GetStarted) {
    RUN_TEST_CASE(GetStarted, PassingTest);
}

TEST_GROUP(GetStarted);

TEST_SETUP(GetStarted) {}

TEST_TEAR_DOWN(GetStarted) {}

TEST(GetStarted, PassingTest) {
    TEST_ASSERT_TRUE(1);
}
