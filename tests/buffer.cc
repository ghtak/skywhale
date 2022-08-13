#include <gtest/gtest.h>
#include <iostream>
#include <memory>
#include <skywhale/buffer.h>

#define GTEST_COUT std::cerr << "[          ] [ USER ] "

auto alloc_byte_ptr(int size) { return std::make_shared<char[]>(size); }

// Demonstrate some basic assertions.
TEST(BufferTest, alloc_ptr) {
    auto buf = alloc_byte_ptr(32);
    auto buf2 = alloc_byte_ptr(64);
    GTEST_COUT << typeid(buf).name() << std::endl;
}

TEST(BufferTest, base) { 
    auto buf = skywhale::buffer<char>(32); 
    {
        skywhale::buffer<char> buf2(buf);
        ASSERT_EQ(buf.data().use_count(), 3);
    }
    ASSERT_EQ(buf.data().use_count(), 2);
    auto buf2 = skywhale::buffer<char>(0); 
    buf2 = buf;
    ASSERT_EQ(buf.data().use_count(), 3);
}