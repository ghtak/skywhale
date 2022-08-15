#include <gtest/gtest.h>
#include <iostream>
#include <memory>
#include <skywhale/basic_buffer.h>

#define GTEST_COUT std::cerr << "[          ] [ USER ] "

// Demonstrate some basic assertions.
TEST(BasicBufferTest, shared_buffer) {
    const char *helloworld = "helloworld";
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


TEST(BasicBufferTest, shared_buffer_1) {
    skywhale::shared_buffer read_buffer(2048);

    const char *data = "packet data";
    std::size_t op = 0x01;
    std::size_t size = strlen(data);

    read_buffer.reserve(sizeof(size) + size);

    if (true) {
        read_buffer.write(op);
        read_buffer.write(size);
        read_buffer.write(data, size);
    } else {
        memcpy(read_buffer.pptr(), &op, sizeof(op));
        read_buffer.commit(sizeof(op));
        memcpy(read_buffer.pptr(), &size, sizeof(size));
        read_buffer.commit(sizeof(size));
        memcpy(read_buffer.pptr(), data, size);
        read_buffer.commit(size);
    }

    ASSERT_EQ(read_buffer.gsize(), sizeof(op) + sizeof(size) + size);

    std::size_t rop = 0;
    std::size_t rsize = 0;
    char rdata[32];
    if (true) {
        rop = read_buffer.read<std::size_t>();
        rsize = read_buffer.read<std::size_t>();
        read_buffer.read(rdata, rsize);
    } else {
        memcpy(&rop, read_buffer.gptr(), sizeof(rop));
        read_buffer.consume(sizeof(rop));
        memcpy(&rsize, read_buffer.gptr(), sizeof(rsize));
        read_buffer.consume(sizeof(rsize));
        memcpy(rdata, read_buffer.gptr(), rsize);
        read_buffer.consume(rsize);
    }
    rdata[rsize] = 0;

    ASSERT_EQ(op, rop);
    ASSERT_EQ(size, rsize);
    ASSERT_STREQ(data, rdata);

    EXPECT_THROW(
        {
            try {
                rop = read_buffer.read<std::size_t>();
            } catch (const std::range_error &e) {
                EXPECT_STREQ("underflow data", e.what());
                throw;
            }
        },
        std::range_error);
}


// Demonstrate some basic assertions.
TEST(BasicBufferTest, simple_buffer) {
    const char *helloworld = "helloworld";
    std::size_t dsize = strlen(helloworld) + 1;
    skywhale::simple_buffer buf(32);
    ASSERT_TRUE(buf.pptr() != nullptr);
    ASSERT_EQ(buf.psize(), 32);

    buf.reserve(64);
    std::memcpy(buf.pptr(), helloworld, dsize);
    buf.commit(dsize);

    skywhale::simple_buffer buf2(buf);

    ASSERT_TRUE(buf2.pptr() != nullptr);
    ASSERT_EQ(buf2.psize(), 64 - dsize);

    GTEST_COUT << buf2.gptr() << std::endl;

    char helloworldb[32];
    std::memcpy(helloworldb, buf2.gptr(), dsize);
    buf2.consume(dsize);
    buf2.shift_data();
    ASSERT_EQ(buf2.gsize(), 0);
    ASSERT_EQ(buf2.psize(), 64);
    GTEST_COUT << helloworldb << std::endl;
}


TEST(BasicBufferTest, simple_buffer_1) {
    skywhale::simple_buffer read_buffer(2048);

    const char *data = "packet data";
    std::size_t op = 0x01;
    std::size_t size = strlen(data);

    read_buffer.reserve(sizeof(size) + size);

    if (true) {
        read_buffer.write(op);
        read_buffer.write(size);
        read_buffer.write(data, size);
    } else {
        memcpy(read_buffer.pptr(), &op, sizeof(op));
        read_buffer.commit(sizeof(op));
        memcpy(read_buffer.pptr(), &size, sizeof(size));
        read_buffer.commit(sizeof(size));
        memcpy(read_buffer.pptr(), data, size);
        read_buffer.commit(size);
    }

    ASSERT_EQ(read_buffer.gsize(), sizeof(op) + sizeof(size) + size);

    std::size_t rop = 0;
    std::size_t rsize = 0;
    char rdata[32];
    if (true) {
        rop = read_buffer.read<std::size_t>();
        rsize = read_buffer.read<std::size_t>();
        read_buffer.read(rdata, rsize);
    } else {
        memcpy(&rop, read_buffer.gptr(), sizeof(rop));
        read_buffer.consume(sizeof(rop));
        memcpy(&rsize, read_buffer.gptr(), sizeof(rsize));
        read_buffer.consume(sizeof(rsize));
        memcpy(rdata, read_buffer.gptr(), rsize);
        read_buffer.consume(rsize);
    }
    rdata[rsize] = 0;

    ASSERT_EQ(op, rop);
    ASSERT_EQ(size, rsize);
    ASSERT_STREQ(data, rdata);

    EXPECT_THROW(
        {
            try {
                rop = read_buffer.read<std::size_t>();
            } catch (const std::range_error &e) {
                EXPECT_STREQ("underflow data", e.what());
                throw;
            }
        },
        std::range_error);
}