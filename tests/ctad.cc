#include <gtest/gtest.h>

template <typename T> struct sum {
    sum(const T &a, const T &b) : value(a + b) {}
    T value;
};

// user defined class template argument dedection
template <typename T, typename U> sum(T, U) -> sum<std::common_type_t<T, U>>;

TEST(CTAD, Basic) { sum s(1, 1.1); }