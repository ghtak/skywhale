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

struct Link {
    Link(void) : read_buffer(2048) {}
    skywhale::shared_buffer read_buffer;
};

TEST(BasicBufferTest, shared_buffer_1) {
    Link link;

    const char *data = "packet data";
    std::size_t op = 0x01;
    std::size_t size = strlen(data);

    link.read_buffer.reserve(sizeof(size) + size);

    if (true) {
        link.read_buffer.write(op);
        link.read_buffer.write(size);
        link.read_buffer.write(data, size);
    } else {
        memcpy(link.read_buffer.pptr(), &op, sizeof(op));
        link.read_buffer.commit(sizeof(op));
        memcpy(link.read_buffer.pptr(), &size, sizeof(size));
        link.read_buffer.commit(sizeof(size));
        memcpy(link.read_buffer.pptr(), data, size);
        link.read_buffer.commit(size);
    }

    ASSERT_EQ(link.read_buffer.gsize(), sizeof(op) + sizeof(size) + size);

    std::size_t rop = 0;
    std::size_t rsize = 0;
    char rdata[32];
    if (true) {
        rop = link.read_buffer.read<std::size_t>();
        rsize = link.read_buffer.read<std::size_t>();
        link.read_buffer.read(rdata, rsize);
    } else {
        memcpy(&rop, link.read_buffer.gptr(), sizeof(rop));
        link.read_buffer.consume(sizeof(rop));
        memcpy(&rsize, link.read_buffer.gptr(), sizeof(rsize));
        link.read_buffer.consume(sizeof(rsize));
        memcpy(rdata, link.read_buffer.gptr(), rsize);
        link.read_buffer.consume(rsize);
    }
    rdata[rsize] = 0;

    ASSERT_EQ(op, rop);
    ASSERT_EQ(size, rsize);
    ASSERT_STREQ(data, rdata);

    EXPECT_THROW(
        {
            try {
                rop = link.read_buffer.read<std::size_t>();
            } catch (const std::range_error &e) {
                EXPECT_STREQ("underflow data", e.what());
                throw;
            }
        },
        std::range_error);
}