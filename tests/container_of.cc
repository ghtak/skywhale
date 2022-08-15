#include <cstddef>
#include <gtest/gtest.h>
#include <iostream>

template <class Parent, class Member>
std::ptrdiff_t offset_of(const Member Parent::*ptr_to_member) {
    const Parent *const parent = nullptr;
    const char *const member = static_cast<const char *>(
        static_cast<const void *>(&(parent->*ptr_to_member)));
    std::ptrdiff_t val(
        member - static_cast<const char *>(static_cast<const void *>(parent)));
    return val;
}

template <class Parent, class Member>
inline Parent *container_of(Member *member,
                            const Member Parent::*ptr_to_member) {
    return static_cast<Parent *>(
        static_cast<void *>(static_cast<char *>(static_cast<void *>(member)) -
                            offset_of(ptr_to_member)));
}

template <class Parent, class Member>
inline const Parent *container_of(const Member *member,
                                  const Member Parent::*ptr_to_member) {
    return static_cast<const Parent *>(static_cast<const void *>(
        static_cast<const char *>(static_cast<const void *>(member)) -
        offset_of(ptr_to_member)));
}
// Demonstrate some basic assertions.
TEST(ContainerOf, Basic) {
    struct foo {
        int i;
        void *ctx;
    };

    foo bar;
    void **ctx = &bar.ctx;
    foo *pbar = container_of(ctx, &foo::ctx);
    ASSERT_EQ(&bar, pbar);
}