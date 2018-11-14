use std::net::TcpListener;
use std::fs;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    //create a listener and bind it to local host at port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        //stream is shadowing the original here, crashing on conflicts for simplicity sake
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    //create mutable byte stream
    let mut buffer = [0; 512];

    //read stream connection request into buffer
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","../index.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","../404.html")
    };

    let content = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}",status_line,content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
