use std::cell::RefCell;
#[allow(unused_imports)]
use std::ops::Add;

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

pub mod task{
    use crate::future::NOTIFY;

    pub struct Context<'a>{
        waker: &'a Waker
    }

    impl<'a> Context<'a> {
        pub fn from_waker(waker: &'a Waker) -> Self {
            Context{ waker }
        }

        pub fn waker(&self) -> &'a Waker {
            &self.waker
        }
    }

    pub struct Waker;

    impl Waker {
        pub fn wake(&self) {
            NOTIFY.with(|f| *f.borrow_mut() = true)
        }
    }
}

pub mod future{
    use crate::future::task::Context;
    pub enum Poll<T>{
        Ready(T),
        Pending,
    }

    pub trait Future {
        type Output;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output>;
    }

    pub struct Ready<T>(Option<T>);

    impl<T> Future for Ready<T> {
        type Output = T;

        fn poll(&mut self, _ctx: &Context) -> Poll<Self::Output> {
            Poll::Ready(self.0.take().unwrap())
        }
    }

    pub fn ready<T>(val: T) -> Ready<T> {
        Ready(Some(val))
    }
}

use crate::future::future::{ Future, Poll };
use crate::future::task::{ Context, Waker };

fn block_on<F>(mut f: F) -> F::Output
where
    F: Future
{
    NOTIFY.with(|n| {
        loop {
            if *n.borrow() {
                *n.borrow_mut() = false;
                let ctx = Context::from_waker(&Waker);
                if let Poll::Ready(val) = f.poll(&ctx) {
                    return val;
                }
            }
        }
    })
}

pub(crate) fn local_main(){
    let fut = future::ready(1);
    println!("{}", block_on(fut));
    /*
    let my_future = MyFuture::default();
    println!("Output {}", run(
        AddOneFuture(my_future))
    );*/

    /*
        let my_future = future::ready(1)
        .map(|x| x + 3)
        .map(Ok)
        .map_err(|e: ()| format!("Error: {:?}", e))
        .and_then(|x| future::ready(Ok(x - 3)))
        .then(|res| {
            future::ready(match res {
                Ok(val) => Ok(val + 3),
                err => err,
            })
        });

    let val = block_on(my_future);
    assert_eq!(val, Ok(4));
     */
}



#[derive(Default)]
struct MyFuture{
    count: i32,
}

impl Future for MyFuture{
    type Output = i32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.count {
            3 => Poll::Ready(3),
            _ => {
                println!("{}", self.count);
                self.count += 1;
                ctx.waker().wake();
                Poll::Pending
            }
        }
    }
}

struct AddOneFuture<T>(T);

impl<T> Future for AddOneFuture<T>
    where
        T : Future,
        T::Output: std::ops::Add<i32, Output = i32>,
{
    type Output = i32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.0.poll(ctx) {
            Poll::Ready(cnt) => Poll::Ready(cnt+1),
            _ => Poll::Pending
        }
    }
}
