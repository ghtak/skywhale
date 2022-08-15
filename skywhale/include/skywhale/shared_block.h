#ifndef __sw_shared_block_h__
#define __sw_shared_block_h__

#include <memory>

namespace skywhale {

class shared_block {
  public:
    explicit shared_block(std::size_t N)
        : _ptr(std::make_shared<char[]>(N)), _size(N) {}

    shared_block(const shared_block &rhs) = default;
    shared_block &operator=(const shared_block &rhs) = default;
    shared_block(shared_block &&rhs) = default;
    shared_block &operator=(shared_block &&rhs) = default;

    char *data(void) const { return _ptr.get(); }

    std::size_t size(void) const { return _size; }

    std::shared_ptr<char[]> ptr(void) const { return _ptr; }

    void resize(std::size_t n) {
        auto nptr = std::make_shared<char[]>(n);
        std::memcpy(nptr.get(), _ptr.get(), _size);
        _ptr = nptr;
        _size = n;
    }

  private:
    std::shared_ptr<char[]> _ptr;
    std::size_t _size;
};

} // namespace skywhale

#endif