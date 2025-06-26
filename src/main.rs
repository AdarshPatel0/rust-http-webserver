use std::{
	io::prelude::*,
	net::{
		TcpListener,
		TcpStream
	},
	fs::{
		File,
	}
};

const ADDRESS: &str = "0.0.0.0:8080";
fn main() {
	let listener = TcpListener::bind(ADDRESS).expect("Error: Failed to start listener");
	for stream in listener.incoming(){
		handle_request(&mut stream.expect("Error: Failed to load stream"));
	}
}

fn handle_request(stream: &mut TcpStream){
	let mut buffer: [u8; 1024] = [0; 1024];
	stream.read(&mut buffer).expect("Error: Failed to read from stream");

	let mut file = File::open("hello.html").expect("Error: Failed to open file");
	let mut contents = Vec::new();
	file.read_to_end(&mut contents).expect("Frrow: Failed to read file");

	let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";

	stream.write_all(response.as_bytes()).expect("Erorr: Failed to write to headers");
	stream.write_all(&contents).expect("Error: Failed to write file contents");
}