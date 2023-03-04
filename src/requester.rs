use std::io::{BufRead, BufReader};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

#[derive(Debug)]
pub enum HttpError {
    NotFound,
    Unknown,
}

pub fn request(url: &str) -> Result<String, HttpError> {
    // Determine the protocol of the URL (http or https)
    let protocol: &str;
    let host: &str;
    let path: &str;
    if url.starts_with("http://") {
        protocol = "HTTP/1.1";
        host = &url[7..];
        path = "/";
    } else if url.starts_with("https://") {
        protocol = "HTTPS/1.1";
        host = &url[8..];
        path = "/";
    } else {
        return Err(HttpError::Unknown);
    }

    // Establish a TCP connection to the server
    let port = if protocol == "HTTP/1.1" { 80 } else { 443 };
    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();

    // Send an HTTP GET request for the specified URL
    let request = format!(
        "GET {} {}{}",
        path,
        protocol,
        "\r\nHost: ".to_string() + host + "\r\n\r\n"
    );
    stream.write(request.as_bytes()).unwrap();

    // Read the response from the server into a string
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    if response.starts_with("HTTP/1.1 404") {
        return Err(HttpError::NotFound);
    }
    if !response.starts_with(protocol) {
        return Err(HttpError::Unknown);
    }
    while let Ok(len) = reader.read_line(&mut response) {
        if len == 0 || response == "\r\n" {
            break;
        }
    }
    let mut body = String::new();
    reader.read_to_string(&mut body).unwrap();
    Ok(body)
}
