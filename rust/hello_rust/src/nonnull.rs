extern crate alloc;

use alloc::boxed::Box;
use std::mem;
use core::ptr;

struct Foo(NonNull<()>);

const TAG_MASK: usize = 0b11;
const TAG_SIMPLE_MESSAGE: usize = 0b00;
const TAG_CUSTOM: usize = 0b01;
const TAG_OS: usize = 0b10;
const TAG_SIMPLE: usize = 0b11;

pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}

impl<T: ?Sized> NonNull<T> {
    pub const fn as_ptr(&self) -> *mut T {
        self.pointer as *mut T
    }
}

fn static_message_check(m : &'static str) {
    let v = m as *const _ as *const () as i64;
    println!("{:#34b} {:#x} {:?} {:?} {:#34b}",v, v, v, mem::size_of_val(&m), TAG_MASK);
    unsafe {
        let ptr = NonNull{
            pointer: (m as *const _ as *mut ()) as _
        };
        let p:usize = unsafe { mem::transmute(ptr.as_ptr()) };
        print!("{:?} {:?}", p, p & TAG_MASK);
    }
}

pub(crate) fn address_check(){
    let ap = Box::into_raw(Box::new(10));
    let pp:usize = unsafe { mem::transmute(ap) };
    println!("{:?} {:?} {:?}", ap, mem::size_of_val(&ap), pp);
    static_message_check("static message");
    let cleanup_box = unsafe { Box::from_raw(ap) };
}