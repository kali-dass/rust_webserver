use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listner.incoming(){
        let streams= stream.unwrap();

        handle_connections(streams)
    }
}

fn handle_connections(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result |result.unwrap())
        .take_while(|line | !line.is_empty())
        .collect();

    println!("request: {:#?}", http_request)
}