#ifndef __skw_buffer_h__
#define __skw_buffer_h__

#include <memory>

namespace skywhale {

template <typename T> class buffer {
  public:
    explicit buffer(std::size_t N) : _data(std::make_shared<T[]>(N)) {}

    buffer(const buffer &rhs) : _data(rhs._data) {}

    buffer &operator=(const buffer &rhs) {
        _data = rhs._data;
        return *this;
    }

    std::shared_ptr<T[]> data(void) { return _data; }

  private:
    std::shared_ptr<T[]> _data;
};
} // namespace skywhale

#endif