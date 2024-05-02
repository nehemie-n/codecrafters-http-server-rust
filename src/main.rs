use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

/**
 * GET /index.html HTTP/1.1
 *
 * Host: localhost:4221
 * User-Agent: curl/7.64.1
 */

fn extract_path(req: &Vec<String>) -> (String, String) {
    println!("REQ PATH: {:?}", req);
    let first_line = req.get(0).unwrap();
    let mut first_line = first_line.split_whitespace();
    let method = first_line.next().unwrap();
    let path = first_line.next().unwrap();
    return (method.to_string(), path.to_string());
}

/**
 * Extracts the user agent header
 * req: ["GET /echo/abc HTTP/1.1", "Host: 127.0.0.1:4221", "User-Agent: curl/8.4.0", "Accept: *\/\*"]
 */
fn extract_user_agent(req: &Vec<String>) -> String {
    println!("REQ AGENT: {:?}", req);
    let default = ":".to_string();
    let agent = req.get(2).unwrap_or(&default);
    let agent = agent.split(":").last().unwrap().trim();
    return agent.to_string();
}

/**
 * Extracts the request
 */
fn extract_request(mut stream: &TcpStream) -> Vec<String> {
    let reader = BufReader::new(&mut stream);
    let req: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();
    req
}

fn handle_file_req(mut stream: &TcpStream, path: String, directoy: String) {
    let filename = path.split("/files/").last().unwrap().to_string();
    let mut resp = String::new();
    if let Ok(mut file) = fs::File::open(format!("{}/{}", directoy, filename)) {
        let mut file_string = String::new();
        let _ = file.read_to_string(&mut file_string);
        resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            file_string.len(),
            file_string
        );
    } else {
        resp = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            "".len(),
            ""
        );
    }
    let _ = stream.write(resp.as_bytes());
}
/***
 * Handle a single request
 */
fn handle_request(mut stream: TcpStream, directory: String) {
    println!("new client!");
    let request = extract_request(&stream);
    let (_method, path) = extract_path(&request);
    let user_agent = extract_user_agent(&request);

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
            let message: Vec<&str> = path.split("/echo/").collect();
            println!("Message {:?}", message);
            let message = message[1].to_string();
            println!("Message {}", message);
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            let _ = stream.write(resp.as_bytes());
        }
        _ if path.starts_with("/user-agent") => {
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent.len(),
                user_agent
            );
            let _ = stream.write(resp.as_bytes());
        }
        _ if path.starts_with("/files/") => handle_file_req(&stream, path, directory),
        _ => {
            let resp = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
            let _ = stream.write(resp.as_bytes());
        }
    }
    stream.flush().unwrap();
}

fn main() {
    let directory = env::args().nth(2).unwrap_or("".to_string());
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        let directory = directory.clone();
        thread::spawn(move || {
            let stream = stream.unwrap();
            handle_request(stream, directory);
        });
    }
}
