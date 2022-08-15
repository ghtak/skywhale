#include <gtest/gtest.h>
#include <iostream>
#include <memory>
#include <skywhale/basic_buffer.h>

#define GTEST_COUT std::cerr << "[          ] [ USER ] "

// Demonstrate some basic assertions.
TEST(BasicBufferTest, shared_block) {
    const char* helloworld = "helloworld";
    std::size_t dsize = strlen(helloworld) + 1;
    skywhale::shared_buffer buf(32);
    ASSERT_TRUE(buf.pptr() != nullptr);
    ASSERT_EQ(buf.psize(), 32);
    
    buf.reserve(64);
    std::memcpy(buf.pptr(), helloworld, dsize);
    buf.commit(dsize);
    
    ASSERT_TRUE(buf.pptr() != nullptr);
    ASSERT_EQ(buf.psize(), 64 - dsize);
    
    GTEST_COUT << buf.gptr() << std::endl;
    
    char helloworldb[32];
    std::memcpy(helloworldb, buf.gptr(), dsize);
    buf.consume(dsize);
    buf.shift_data();
    ASSERT_EQ(buf.gsize(), 0);
    ASSERT_EQ(buf.psize(), 64);
    GTEST_COUT << helloworldb << std::endl;    
}

