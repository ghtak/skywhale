#ifndef __sw_simple_block_h__
#define __sw_simple_block_h__

#include <memory>

namespace skywhale {

class simple_block {
  public:
    explicit simple_block(std::size_t N) { _data.resize(N, 0); }

    simple_block(const simple_block &rhs) = default;
    simple_block &operator=(const simple_block &rhs) = default;
    simple_block(simple_block &&rhs) = default;
    simple_block &operator=(simple_block &&rhs) = default;

    char *data(void) { return &_data[0]; }

    std::size_t size(void) const { return _data.size(); }

    void resize(std::size_t n) {
        _data.resize(n, 0);
    }
  private:
    std::vector<char> _data;
};

} // namespace skywhale

#endif