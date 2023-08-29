use std::{
    net::{TcpListener}, 
    io::{Read, Write, BufReader, BufRead}
};
use std::fs;

pub fn serve(filePath: &str) {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let bufReader = BufReader::new(&mut stream);
        let request_line = bufReader.lines().next().unwrap().unwrap();

        println!("Request: {}", request_line);

        if (request_line == "GET /pkg/oot.js HTTP/1.1") {
            let jsContent = fs::read_to_string("pkg/oot.js").unwrap();
            let jsContentLength = jsContent.len();

            let mut response = String::new();
            response.push_str("HTTP/1.1 200 OK\r\n");
            response.push_str("Content-Type: text/javascript; charset=UTF-8\r\n");
            response.push_str(format!("Content-Length: {jsContentLength}\r\n\r\n").as_str());
            response.push_str(jsContent.as_str());

            stream.write_all(response.as_bytes()).unwrap();

            continue;
        }

        else if (request_line == "GET /pkg/oot_bg.wasm HTTP/1.1") {
            let wasmContent = fs::read("pkg/oot_bg.wasm").unwrap();
            let wasmContentLength = wasmContent.len();

            let mut response = String::new();
            response.push_str("HTTP/1.1 200 OK\r\n");
            response.push_str("Content-Type: application/wasm; charset=UTF-8\r\n");
            response.push_str(format!("Content-Length: {wasmContentLength}\r\n\r\n").as_str());
            
            stream.write_all(response.as_bytes()).unwrap();
            stream.write_all(&wasmContent).unwrap();

            continue;
        }

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