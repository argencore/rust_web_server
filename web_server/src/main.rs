use std::net::TcpListener;

fn main() {
    //create a listener and bind it to local host at port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        //stream is shadowing the original here, crashing on conflicts for simplicity sake
        let stream = stream.unwrap();
        println!("connection established!");
    }
}
