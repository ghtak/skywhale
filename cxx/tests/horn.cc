#include <gtest/gtest.h>
#include <skywhale/skywhale.h>

// Demonstrate some basic assertions.
TEST(HelloTest, Horn) {
  EXPECT_EQ(skywhale::horn(), 81);
}