#ifndef __sw_fixed_block_h__
#define __sw_fixed_block_h__

#include <iostream>

namespace skywhale {

class fixed_block {
  public:
    explicit fixed_block(char *ptr, std::size_t N) : _ptr(ptr), _size(N) {}

    fixed_block(const fixed_block &rhs) = delete;
    fixed_block &operator=(const fixed_block &rhs) = delete;
    fixed_block(fixed_block &&rhs) = delete;
    fixed_block &operator=(fixed_block &&rhs) = delete;

    char *data(void) { return _ptr; }

    std::size_t size(void) const { return _size; }

    void resize(std::size_t n, std::size_t) {
        throw std::runtime_error("fixed_block::resize() is not supported");
    }

  private:
    char *_ptr;
    std::size_t _size;
};

} // namespace skywhale

#endif