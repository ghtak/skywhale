#ifndef __skw_basic_buffer_h__
#define __skw_basic_buffer_h__

#include <skywhale/shared_block.h>

namespace skywhale {
template <typename BlockT> class basic_buffer {
  public:
    template <typename... Args>
    basic_buffer(Args &&...args)
        : _block(std::forward<Args>(args)...), _gpos(0), _ppos(0) {}

    basic_buffer(const basic_buffer &rhs) = default;
    basic_buffer &operator=(const basic_buffer &rhs) = default;
    basic_buffer(basic_buffer &&rhs) = default;
    basic_buffer &operator=(basic_buffer &&rhs) = default;

    char *gptr(void) { return _block.data() + _gpos; }

    char *pptr(void) { return _block.data() + _ppos; }

    std::size_t gsize(void) { return _ppos - _gpos; }
    std::size_t psize(void) { return _block.size() - _ppos; }

    void shift_data(void) {
        std::memmove(_block.data(), gptr(), gsize());
        _ppos = gsize();
        _gpos = 0;
    }

    void reserve(std::size_t n) {
        std::size_t ps = psize();
        if (ps >= n)
            return;

        shift_data();

        ps = psize();
        if (ps >= n)
            return;

        _block.resize(_ppos + n);
    }

    void commit(std::size_t n) { _ppos += std::min<std::size_t>(n, psize()); }

    void consume(std::size_t n) { _gpos += std::min<std::size_t>(n, gsize()); }

  private:
    BlockT _block;
    std::size_t _gpos;
    std::size_t _ppos;
};

using shared_buffer = basic_buffer<shared_block>;

} // namespace skywhale

#endif