#ifndef __skw_basic_buffer_h__
#define __skw_basic_buffer_h__

#include <skywhale/shared_block.h>
#include <skywhale/memory_block.h>
#include <skywhale/fixed_block.h>

namespace skywhale {
template <typename BlockT> class basic_buffer {
  public:
    template <
        typename T0, typename... Ts,
        std::enable_if_t<sizeof...(Ts) || !std::is_same<std::decay_t<T0>,
                                                        basic_buffer>::value,
                         std::nullptr_t> = nullptr>
    basic_buffer(T0 &&t0, Ts &&...ts)
        : _block(std::forward<T0>(t0), std::forward<Ts>(ts)...), _gpos(0),
          _ppos(0) {}

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

        _block.resize(_ppos + n, gsize());
    }

    void commit(std::size_t n) { _ppos += std::min<std::size_t>(n, psize()); }

    void consume(std::size_t n) { _gpos += std::min<std::size_t>(n, gsize()); }

    template <typename T> void write(T &&t) {
        std::size_t n = sizeof(t);
        reserve(n);
        memcpy(pptr(), &t, n);
        commit(n);
    }

    void write(const char *ptr, std::size_t n) {
        reserve(n);
        memcpy(pptr(), ptr, n);
        commit(n);
        std::cerr << n << std::endl;
    }

    template <typename T> T peak(void) {
        T t;
        std::size_t n = sizeof(t);
        if (gsize() < n) {
            throw std::range_error("underflow data");
        }
        memcpy(&t, gptr(), n);
        return t;
    }

    template <typename T> T read(void) {
        T t = peak<T>();
        consume(sizeof(t));
        return t;
    }

    void peak(char *ptr, std::size_t n) {
        if (gsize() < n) {
            throw std::range_error("underflow data");
        }
        memcpy(ptr, gptr(), n);
    }

    void read(char *ptr, std::size_t n) {
        peak(ptr, n);
        consume(n);
    }

  private:
    BlockT _block;
    std::size_t _gpos;
    std::size_t _ppos;
};

using shared_buffer = basic_buffer<shared_block>;
using memory_buffer = basic_buffer<memory_block>;
using fixed_buffer = basic_buffer<fixed_block>;

} // namespace skywhale

#endif
