use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}
};


fn main() {
    let addr = "127.0.0.1:7878".to_string();
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
        // println!("Hey! Hey! Hey!");
    }
    
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = reader.lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
    println!("Request: {http_request:#?}"); 

    // let response = "HTTP/1.1 200 OK\r\n\r\nHello World!!";
    // stream.write_all(response.as_bytes()).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("src/hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();

}
