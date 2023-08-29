use std::{
    net::{TcpListener}, 
    io::{Read, Write, BufReader, BufRead}
};
use std::fs;

fn createResponse(status: String, contentType: String, mut content: Vec<u8>) -> Vec<u8> {
    let contentLength = content.len();

    let mut response: Vec<u8> = Vec::new();
    response.write_all("HTTP/1.1 200 OK\r\n".as_bytes());
    response.write_all(format!("Content-Type: {contentType}\r\n").as_bytes());
    response.write_all(format!("Content-Length: {contentLength}\r\n\r\n").as_bytes());

    response.append(&mut content);

    return response;
}

pub fn serve(filePath: &str) {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let bufReader = BufReader::new(&mut stream);
        let request_line = bufReader.lines().next().unwrap().unwrap();

        println!("Request: {}", request_line);

        let response: Vec<u8>;
        if (request_line == "GET /pkg/oot.js HTTP/1.1") {
            let content = fs::read("pkg/oot.js").unwrap();
            response = createResponse(
                "".to_string(), 
                "text/javascript".to_string(), 
                content
            );
        }

        else if (request_line == "GET /pkg/oot_bg.wasm HTTP/1.1") {
            let content = fs::read("pkg/oot_bg.wasm").unwrap();
            response = createResponse(
                "".to_string(), 
                "application/wasm".to_string(), 
                content
            );
        }

        else {
            let content = fs::read("src/base.html").unwrap();
            response = createResponse(
                "".to_string(), 
                "text/html".to_string(), 
                content
            );
        }

        stream.write_all(&response).unwrap();
    }
}   