use std::{env, path};
use std::{
    fs,
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

static ROOT : &str = "src/site";

#[derive(Debug)]
enum ParseError {
    InvalidHeader,
    InvalidMethod,
    InvalidURI,
    InvalidVersion,
    InvalidRequest,
}

struct Request {
    method: String,
    uri: String,
    version: String,
    headers: HashMap<String, String>,
}

fn get_headers(lines: &[String]) -> Result<HashMap<String, String>, ParseError> {
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        } else if let Some(pos) = line.find(":") {
            let (key, value) = line.split_at(pos);
            headers.insert(String::from(key), String::from(value));
        } else {
            return Err(ParseError::InvalidHeader);
        }
    }
    Ok(headers)
}

fn parse_request(text: Vec<String>) -> Result<Request, ParseError>{
    
    // First line contains the method, path, and HTTP version
    let first_line: Vec<&str> = text[0].split_whitespace().collect();
    let method = *first_line.get(0).ok_or(ParseError::InvalidRequest)?;
    let mut uri = *first_line.get(1).ok_or(ParseError::InvalidRequest)?;
    let version = *first_line.get(2).ok_or(ParseError::InvalidRequest)?;
    if uri == "/" { uri = "/index.html"; }

    // Error handling for first line
    if method != "GET" {
        return Err(ParseError::InvalidMethod);
    } else if !uri.starts_with("/") {
        return Err(ParseError::InvalidURI);
    } else if version != "HTTP/1.1" && version != "HTTP/1.0"{
        return Err(ParseError::InvalidVersion);
    } else if text.len() < 2 {
        return Err(ParseError::InvalidRequest);
    }

    println!("Method: {}, Path: {}, Version: {}", method, uri, version);

    let headers = get_headers(&text[1..])?;
    println!("Headers: {:#?}", headers);

    let request = Request {
        method: String::from(method),
        uri: String::from(uri),
        version: String::from(version),
        headers: headers,
    };
    Ok(request)
}

fn handle_connection(mut stream: TcpStream) {
    
    // Read the request
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    // Parse request
    let request = parse_request(http_request).unwrap();
    let path = format!("{}{}", ROOT, request.uri);
    println!("Path: {}", path);

    // Create a response
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Write the response

    // Flush the stream
}

fn main() {
    
    // Parse command line arguments to set port and root directory
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() != 2 {
        panic!("Usage: <port>");
    }
    
    let port = &args[1];
    dbg!(port);

    // Create a TCP listener
    let listener= TcpListener::bind(format!("localhost:{}", port)).unwrap();

    // Listen for incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Spawn a thread to handle each connection
        handle_connection(stream);
    }

}


