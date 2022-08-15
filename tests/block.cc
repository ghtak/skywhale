#include <gtest/gtest.h>
#include <skywhale/shared_block.h>

// Demonstrate some basic assertions.
TEST(BlockTest, shared_block) {
    auto A = skywhale::shared_block(32);

    ASSERT_EQ(A.ptr().use_count(), 2);
    ASSERT_TRUE(A.data() != nullptr);
    ASSERT_EQ(A.size(), 32);

    auto B(A);

    ASSERT_EQ(A.ptr().use_count(), 3);
    
    auto C(std::move(B));

    ASSERT_EQ(A.ptr().use_count(), 3);
    ASSERT_TRUE(B.data() == nullptr);
    ASSERT_TRUE(C.data() != nullptr);

    B = std::move(C);

    ASSERT_EQ(A.ptr().use_count(), 3);
    ASSERT_TRUE(B.data() != nullptr);
    ASSERT_TRUE(C.data() == nullptr);
}