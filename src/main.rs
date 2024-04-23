use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

/**
 * GET /index.html HTTP/1.1
 *
 * Host: localhost:4221
 * User-Agent: curl/7.64.1
 */
fn handle_request(mut stream: TcpStream) {
    println!("new client!");
    let mut reader = BufReader::new(&mut stream);
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();

    println!("first line: {}", first_line);
    let parts = first_line.split_whitespace().collect::<Vec<&str>>();
    let path = parts[1];

    match path {
        "/" => {
            let resp = "HTTP/1.1 200 OK\r\n\r\n".to_string();
            let _ = stream.write(resp.as_bytes());
        }
        "/index.html" => {
            let resp = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
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
        print!("new client!");
    }
}
