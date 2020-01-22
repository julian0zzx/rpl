
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;

extern crate chapter20;

fn main() {
    single_thread_server();
    // multi_thread_server();
}

// st_
fn single_thread_server() {
    let listener = TcpListener::bind("127.0.0.1:8890").unwrap();
    for stream in listener.incoming() {
        let strm = stream.unwrap();
        st_handle_connection(strm);
    }
}

fn st_handle_connection(mut strm : TcpStream) {
    let mut buf = [0; 512];
    strm.read(&mut buf).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&mut buf[..]));

    // let method = b"GET / HTTP/1.1\r\n";
    let mut resp = String::from("");
    if buf.starts_with(b"GET / HTTP/1.1\r\n") {
        let contents = fs::read_to_string("index.htm").unwrap();
        resp = format!("HTTP/1.1 200 OK \r\n\r\n {}", contents);
        // resp = format!("HTTP/1.1 200 OK \r\n\r\n {}", "contents");
    } else if buf.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(3));
        resp = String::from("HTTP/1.1 200 OK\r\n\r\n <html><body><h1>Rust, sleep 3 seconds.</h1></body></html>");
    } else if buf.starts_with(b"POST") {
        // POST doesn't work
        resp = String::from("HTTP/1.1 201 Created\r\n\r\n 
        Location:http://localhost:7890/\r\n\r\n 
        <html><head><title>Rust Http Server POST 0.1</title></head>
        <body><h1>Rust, Not a Get</h1></body></html>");
    } else {
        resp = String::from("HTTP/1.1 200 OK\r\n\r\n <html><head><title>Rust Http Server 0.1</title></head><body><h1>Rust, Not a Get</h1></body></html>");
    }
    strm.write(resp.as_bytes()).unwrap();
    strm.flush().unwrap();
}

// mt_
fn multi_thread_server() {
    let listener = TcpListener::bind("127.0.0.1:8899").unwrap();
    let thread_pool = chapter20::ThreadPool::new(3);
    
    for stream in listener.incoming() {
        let strm = stream.unwrap();
        thread_pool.execute(move || {
            mt_handle_connections(strm)
        })
    }
}

fn mt_handle_connections(mut strm : TcpStream) {
    let mut buf = [0; 512];
    strm.read(&mut buf).unwrap();
    let mut resp = String::from("");
    resp = String::from("HTTP/1.1 200 OK\r\n\r\n <html><body><h1>Rust, MUL-STYLE.</h1></body></html>");
    strm.write(resp.as_bytes()).unwrap();
    strm.flush().unwrap();
}
