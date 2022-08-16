#ifndef __sw_shared_block_h__
#define __sw_shared_block_h__

#include <cstring>
#include <memory>

namespace skywhale {

class shared_block {
  public:
    struct impl {
        std::atomic<int> ref;

        static impl *alloc(std::size_t N) {
            void *ptr = operator new(sizeof(impl) + N, std::align_val_t(8));
            impl *pimpl = new (ptr) impl();
            pimpl->ref.store(1);
            return pimpl;
        }

        static void free(impl *pimpl) {
            if (pimpl)
                operator delete(pimpl, std::align_val_t(8));
        }

        static char *data(impl *pimpl) {
            return pimpl ? reinterpret_cast<char *>(pimpl + 1) : nullptr;
        }

        static void addref(impl *pimpl) {
            if (pimpl)
                pimpl->ref.fetch_add(1);
        }

        static void release(impl *pimpl) {
            if (pimpl && pimpl->ref.fetch_sub(1) == 0)
                impl::free(pimpl);
        }
    };

    explicit shared_block(std::size_t N) : _pimpl(impl::alloc(N)), _size(N) {}

    ~shared_block(void) { impl::release(_pimpl); }

    shared_block(const shared_block &rhs)
        : _pimpl(rhs._pimpl), _size(rhs._size) {
        impl::addref(_pimpl);
    }

    shared_block &operator=(const shared_block &rhs) {
        auto p = _pimpl;
        _pimpl = rhs._pimpl;
        _size = rhs._size;
        impl::addref(_pimpl);
        impl::release(p);
        return *this;
    }

    shared_block(shared_block &&rhs) : _pimpl(rhs._pimpl), _size(rhs._size) {
        rhs._pimpl = nullptr;
        rhs._size = 0;
    }
    shared_block &operator=(shared_block &&rhs) {
        auto p = _pimpl;

        _pimpl = rhs._pimpl;
        _size = rhs._size;

        rhs._pimpl = nullptr;
        rhs._size = 0;
        impl::release(p);
        return *this;
    }

    char *data(void) { return impl::data(_pimpl); }

    std::size_t size(void) const { return _size; }

    void resize(std::size_t n, std::size_t ds) {
        if (_size >= n)
            return;
        
        auto np = impl::alloc(n);
        if (ds > 0)
            std::memcpy(impl::data(np), impl::data(_pimpl), ds);
            
        std::swap(_pimpl, np);
        _size = n;
        impl::release(np);
    }

    int use_count(void) { return _pimpl->ref.load(); }

  private:
    impl *_pimpl;
    std::size_t _size;
};

/*
class shared_block {
  public:
    explicit shared_block(std::size_t N)
        : _ptr(std::make_shared<char[]>(N)), _size(N) {}

    shared_block(const shared_block &rhs) = default;
    shared_block &operator=(const shared_block &rhs) = default;
    shared_block(shared_block &&rhs) = default;
    shared_block &operator=(shared_block &&rhs) = default;

    char *data(void) { return _ptr.get(); }

    std::size_t size(void) const { return _size; }

    std::shared_ptr<char[]> ptr(void) const { return _ptr; }

    void resize(std::size_t n, std::size_t ds) {
        auto nptr = std::make_shared<char[]>(n);
        if (ds > 0)
            std::memcpy(nptr.get(), _ptr.get(), ds);
        _ptr = nptr;
        _size = n;
    }

  private:
    std::shared_ptr<char[]> _ptr;
    std::size_t _size;
};
*/
} // namespace skywhale

#endif
