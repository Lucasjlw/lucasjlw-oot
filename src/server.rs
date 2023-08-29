use std::{
    net::{TcpListener, TcpStream}, 
    io::{Read, Write, BufReader}
};
use std::fs::File;

pub fn serve() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut htmlContentString = String::new();
        let _ = File::open("base.html").unwrap().read_to_string(&mut htmlContentString);
        let htmlContentLength = htmlContentString.len();

        let response = format!(
            "HTTP/1.1 200 OK\r\n
            Content-Type: text/html; charset=UTF-8\r\n
            Content-Length: {htmlContentLength}\r\n\r\n
            {htmlContentString}",
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}