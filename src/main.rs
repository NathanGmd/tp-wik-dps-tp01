use std::{
    env,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};


fn main() {
    let env_port: Result<String, env::VarError> = env::var("PORT");
    let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{}", env_port.unwrap())).unwrap();
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let json = http_request_to_json(http_request.clone());
    println!("Request: {json:#?}");
    let request_line = &http_request[0];
    println!("{}", http_request[0]);

    if request_line == "GET /ping HTTP/1.1" || request_line == "GET /ping HTTP/1.2" {
        let json = http_request_to_json(http_request.clone());
        let status_line: &str = "HTTP/1.1 200 OK";
        let content_type: &str = "Content-Type: application/json";
        let content_length: String = format!("Content-Length: {}", json.len());
        let response: String = format!(
            "{status_line}\r\n{content_type}\r\n{content_length}\r\n\r\n{json}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content_length = format!("Content-Length: {}", 0);
        let response = format!(
            "{status_line}\r\n{content_length}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
}


fn http_request_to_json(request: Vec<String>) -> String {
    let mut json = String::from("{\n");
    for (i, line) in request.iter().enumerate() {
            if let Some((key, value)) = line.split_once(": ") {
                json.push_str(&format!("  \"{}\": \"{}\"", key, value));
                if i < request.len() - 1 {
                    json.push_str(",\n");
                } else {
                    json.push_str("\n");
                }
            }
    }
    json.push_str("}");
    json
}