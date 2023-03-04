use std::{
    fs,
    env,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    thread,
    time::Duration,
};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    env::set_var("RUST_BACKTRACE", "full");

    for stream in listner.incoming(){
        let streams= stream.unwrap();

        thread::spawn(|| {
            handle_connections(streams);
        });
    }
}

/*
fn handle_connections(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result |result.unwrap())
        .take_while(|line | !line.is_empty())
        .collect();

    println!("request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    //let contents = fs::read_to_string("hello.html").unwrap();
    let contents: String = http_request.into_iter().collect();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
*/

fn handle_connections(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..]{
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK","hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK","hello.html")
        }
        _ =>   ("HTTP/1.1 404 NOT FOUND","404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
