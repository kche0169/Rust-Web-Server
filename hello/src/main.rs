use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;

fn main() {
    let addr = "127.0.0.1:7878".to_string();
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        pool.execute(|| {handle_connection(_stream)});
        // thread::spawn(|| {handle_connection(_stream)});
        // println!("Hey! Hey! Hey!");
    }
    
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = reader.lines()
    // .map(|result| result.unwrap())
    // .take_while(|line| !line.is_empty())
    // .collect();
    // println!("Request: {http_request:#?}"); 

    // let response = "HTTP/1.1 200 OK\r\n\r\nHello World!!";
    // stream.write_all(response.as_bytes()).unwrap();

    // 相应一个HTML页面
    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("src/hello.html").unwrap();
    // let length = contents.len();

    // let response =
    //     format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    // stream.write_all(response.as_bytes()).unwrap();
    
    
    // 验证请求并有选择的进行响应
    let request_line = reader.lines() // 调用 lines() 方法，返回一个迭代器，该迭代器逐行读取 stream 的内容。
                    .next() //  获取迭代器的下一个元素，也就是请求的第一行。
                    .unwrap() // 第一个 unwrap() 是用于处理 Option 类型（如果没有更多行则返回 None）
                    .unwrap(); // 第二个 unwrap() 是处理 Result 类型（读取行时可能出错）。如果读取成功，则获取到请求行的字符串；如果发生错误，将导致程序崩溃。
    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    let (status_line, filename)  = match &request_line[..] {
        "GET / HTTP/1.1" => {("HTTP/1.1 200 OK", "hello.html")},
        "GET /SLEEP / HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        } 
        _ => {("HTTP/1.1 404 NOT FOUND", "404.html")}
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    
}


