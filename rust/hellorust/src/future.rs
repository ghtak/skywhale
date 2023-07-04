use std::cell::RefCell;

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

        fn map<F, U>(self, f: F) -> Map<Self, F>
        where
            F: FnOnce(Self::Output) -> U,
            Self: Sized,
        {
            Map {
                future: self,
                f
            }
        }

        fn then<Fut, F>(self,f: F) -> Then<Self, F>
        where
            F: FnOnce(Self::Output) -> Fut,
            Fut: Future,
            Self: Sized,
        {
            Then{
                future: self,
                f
            }
        }
    }

    pub trait TryFuture {
        type Ok;
        type Error;

        fn try_poll(&mut self, ctx: &Context) -> Poll<Result<Self::Ok, Self::Error>>;

        fn and_then<Fut, F>(self, f: F) -> AndThen<Self, F>
        where
            F: FnOnce(Self::Ok) -> Fut,
            Fut: Future,
            Self: Sized,
        {
            AndThen {
                future: self,
                f
            }
        }

        fn map_error<R, F>(self, f: F) -> MapErr<Self, F>
        where
            F: FnOnce(Self::Error) -> R,
            Self: Sized,
        {
            MapErr{
                future: self,
                f
            }
        }
    }

    impl<F, T, E> TryFuture for F where F : Future<Output=Result<T,E>> {
        type Ok =T;
        type Error =E;

        fn try_poll(&mut self, ctx: &Context) -> Poll<F::Output> {
            self.poll(ctx)
        }
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

    pub struct Map<Fut, F> {
        future: Fut,
        f: F
    }

    impl<Fut, F, U> Future for Map<Fut, F>
    where
        Fut: Future,
        F: FnOnce(Fut::Output) -> U + Copy
    {
        type Output = U;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
            match self.future.poll(ctx) {
                Poll::Ready(val) => {
                    let v = (self.f)(val);
                    Poll::Ready(v)
                },
                _ => Poll::Pending
            }
        }
    }

    pub struct Then<Fut, F>{
        future: Fut,
        f: F
    }

    impl<Fut, NextFut, F> Future for Then<Fut,F>
    where
        Fut: Future,
        NextFut: Future,
        F: FnOnce(Fut::Output) -> NextFut + Copy,
    {
        type Output = NextFut::Output;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
            match self.future.poll(ctx) {
                Poll::Ready(val) => {
                    ((self.f)(val)).poll(ctx)
                },
                _ => Poll::Pending
            }
        }
    }

    pub struct AndThen<Fut, F> {
        future: Fut,
        f: F,
    }

    impl<Fut, NextFut, F> Future for AndThen<Fut, F>
    where
        Fut: TryFuture,
        NextFut: TryFuture<Error = Fut::Error>,
        F: FnOnce(Fut::Ok) -> NextFut + Copy,
    {
        type Output = Result<NextFut::Ok, Fut::Error>;

        fn poll(&mut self, cx: &Context) -> Poll<Self::Output> {
            match self.future.try_poll(cx) {
                Poll::Ready(Ok(val)) => {
                    (self.f)(val).try_poll(cx)
                }
                Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                Poll::Pending => Poll::Pending,
            }
        }
    }

    pub struct MapErr<Fut, F>{
        future: Fut,
        f: F
    }

    impl<Fut, F, R> Future for MapErr<Fut, F>
    where
        Fut: TryFuture,
        F: FnOnce(Fut::Error) -> R + Copy,
    {
        type Output = Result<Fut::Ok, R>;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
            match self.future.try_poll(ctx) {
                Poll::Ready(result) => {
                    //Poll::Ready(result.map_err((self.f)))
                    match result {
                        Ok(v) => Poll::Ready(Ok(v)),
                        Err(e) => Poll::Ready(Err((self.f)(e)))
                    }
                },
                _ => Poll::Pending
            }
        }
    }
}

use crate::future::future::{Future, Poll, TryFuture};
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
    let fut = future::ready(1)
        .map(|val| val + 1)
        .then(|val| future::ready(val + 1))
        .map(Ok::<i32, ()>)
        .and_then(|val| future::ready(Ok(val * 4)))
        .map_error(|_:()| 5);
    println!("{:?}", block_on(fut));
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
