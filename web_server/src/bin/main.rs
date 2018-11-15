extern crate web_server;

use web_server::ThreadPool;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use std::fs;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    //create a listener and bind it to local host at port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        //stream is shadowing the original here, crashing on conflicts for simplicity sake
        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream){
    //create mutable byte stream
    let mut buffer = [0; 512];

    //read stream connection request into buffer
    stream.read(&mut buffer).expect("failed to read buffer");

    let get = b"GET / HTTP/1.1\r\n";

    //dirty way to simulate long process
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","index.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n","index.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    };

    let content = fs::read_to_string(filename).expect("failed to get content");

    let response = format!("{}{}",status_line,content);
    stream.write(response.as_bytes()).expect("failed to write stream");
    stream.flush().unwrap();
}
