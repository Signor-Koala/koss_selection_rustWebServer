use std::io::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_css = b"GET /styles.css HTTP/1.1\r\n";
    let get_js = b"GET /index.js HTTP/1.1\r\n";
    let get_html = b"GET /index.html HTTP/1.1\r\n";
    let get_gre = b"GET /grievous.png HTTP/1.1\r\n";
    let get_ken = b"GET /kenobi.png HTTP/1.1\r\n";

    if buffer.starts_with(get_html) {
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else if buffer.starts_with(get_js) {

        let contents = fs::read_to_string("index.js").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else if buffer.starts_with(get_css) {

        let contents = fs::read_to_string("styles.css").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else if buffer.starts_with(get_gre) {

        let contents = fs::read("grievous.png").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        );
        let b_contents = &contents;

        stream.write(response.as_bytes()).unwrap();
        stream.write(b_contents).unwrap();
        stream.flush().unwrap();

    } else if buffer.starts_with(get_ken) {

        let contents = fs::read("kenobi.png").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        );
        let b_contents = &contents;

        stream.write(response.as_bytes()).unwrap();
        stream.write(b_contents).unwrap();
        stream.flush().unwrap();
    } else {
        stream.write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
    }


}
