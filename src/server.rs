use std::{
    net::{TcpListener}, 
    io::{Read, Write}
};
use std::fs;

pub fn serve(filePath: &str) {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let htmlContent = fs::read_to_string(filePath).unwrap();
        let htmlContentLength = htmlContent.len();

        let mut response = String::new();
        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str("Content-Type: text/html; charset=UTF-8\r\n");
        response.push_str(format!("Content-Length: {htmlContentLength}\r\n\r\n").as_str());
        response.push_str(htmlContent.as_str());


        stream.write_all(response.as_bytes()).unwrap();
    }
}   