use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

/**
 * GET /index.html HTTP/1.1
 *
 * Host: localhost:4221
 * User-Agent: curl/7.64.1
 */

fn extract_path(req: Vec<String>) -> (String, String) {
    let first_line = req.get(0).unwrap();
    let mut first_line = first_line.split_whitespace();
    let method = first_line.next().unwrap();
    let path = first_line.next().unwrap();
    return (method.to_string(), path.to_string());
}

//
fn extract_request(mut stream: &TcpStream) -> Vec<String> {
    let reader = BufReader::new(&mut stream);
    let req: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();
    req
}
fn handle_request(mut stream: TcpStream) {
    println!("new client!");
    let request = extract_request(&stream);
    let (_method, path) = extract_path(request);

    match path.as_str() {
        "/" => {
            let resp = "HTTP/1.1 200 OK\r\n\r\n".to_string();
            let _ = stream.write(resp.as_bytes());
        }
        "/index.html" => {
            let resp = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
            let _ = stream.write(resp.as_bytes());
        }
        _ if path.starts_with("/echo/") => {
            let message: Vec<&str> = path.split("/").collect();
            let message = message[1];
            let resp = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n{}", message);
            let _ = stream.write(resp.as_bytes());
        }
        _ => {
            let resp = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
            let _ = stream.write(resp.as_bytes());
        }
    }
}

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_request(stream);
        println!("new client!");
    }
}
