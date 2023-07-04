use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio;

async fn worker(){
    println!("this is worker")
}

#[derive(Debug)]
struct Worker{
    message: String
}

impl Future for Worker{
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.message.clone())
    }
}

pub fn local_main(){
    let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
    //let _ = runtime.block_on(worker());
    let val = runtime.block_on(Worker{
        message: String::from("Worker")
    });
    println!("{}", val);
}