use std::io::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;

use http::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4200").unwrap();
	let pool = ThreadPool::new(6);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});
    }
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();

	let get = b"GET / HTTP/1.1\r\n";

	let (status_line, filename) = if buffer.starts_with(get) {
		("HTTP/1.1 200 OK", "index.html")
	} else {
		("HTTP/1.1 404 NOT FOUND", "404.html")
	};
	let content = fs::read_to_string(filename).unwrap();

	let response = format!(
		"{}\r\nContent-Length: {}\r\n\r\n{}",
		status_line,
		content.len(),
		content	
	);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
