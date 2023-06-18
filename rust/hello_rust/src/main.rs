use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs::File;
use std::thread;
extern crate hello_rust;
use hello_rust::ThreadPool;

fn main(){
    let listener = TcpListener::bind(
        "127.0.0.1:7878").unwrap();
    
    let pool = ThreadPool::new(4);

    for strm in listener.incoming().take(2) {
        let strm = strm.unwrap();
        pool.execute(|| {
            handle_connection(strm);
        });
        
    }
}

fn handle_connection(mut strm: TcpStream){
    let mut buffer = [0; 512];
    strm.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    strm.write(response.as_bytes()).unwrap();
    strm.flush().unwrap();

}