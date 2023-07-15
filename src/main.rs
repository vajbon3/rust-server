use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response;

    if request_line == "GET / HTTP/1.1" {
        response = Response {
            status: "200 OK".to_string(),
            headers: HashMap::new(),
            body: render_view(&"index.html".to_string()),
        };
    } else {
        response = Response {
            status: "404 NOT FOUND".to_string(),
            headers: HashMap::new(),
            body: render_view(&"404.html".to_string()),
        };
    }

    stream.write_all(response.to_string().as_bytes()).unwrap();
}

struct Response {
    status: String,
    headers: HashMap<String, String>,
    body: String,
}

impl ToString for Response {
    fn to_string(&self) -> String {
        // add Content-Length header based on body length if it's not already set
        let mut filled_headers = self.headers.clone();

        filled_headers
            .entry(String::from("Content-Length"))
            .or_insert(self.body.len().to_string());

        // transform status, headers and body into a single string
        format!("HTTP/1.1 {}\r\n{}\r\n{}", self.status, &map_to_string(&filled_headers), &self.body)
    }
}

fn map_to_string(map: &HashMap<String, String>) -> String {
    let mut response = String::new();

    map.iter().for_each(|(key, value)| {
        response.push_str(format!("{}: {}\r\n", key, value).as_str());
    });

    response
}

fn render_view(view_path: &String) -> String {
    fs::read_to_string(view_path).unwrap()
}