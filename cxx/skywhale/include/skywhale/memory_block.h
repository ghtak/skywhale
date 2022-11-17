#ifndef __sw_memory_block_h__
#define __sw_memory_block_h__

#include <memory>

namespace skywhale {

class memory_block {
  public:
    explicit memory_block(std::size_t N) : _data(N, 0) {}

    memory_block(const memory_block &rhs) = default;
    memory_block &operator=(const memory_block &rhs) = default;
    memory_block(memory_block &&rhs) = default;
    memory_block &operator=(memory_block &&rhs) = default;

    char *data(void) { return &_data[0]; }

    std::size_t size(void) const { return _data.size(); }

    void resize(std::size_t n, std::size_t) { _data.resize(n, 0); }

  private:
    std::vector<char> _data;
};

} // namespace skywhale

#endif