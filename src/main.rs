use std::net::TcpListener;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listner.incoming(){
        let _streams= stream.unwrap();

        println!("Connection established!!")
    }
}
