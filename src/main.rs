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

const ADDRESS: &str = "0.0.0.0:8080"; //This ip is for VirtualBox. To run locally on your machine use 127.0.0.1:8080

fn main() {
	let listener = TcpListener::bind(ADDRESS).expect("Error: Failed to start listener"); //Create a listener.
	for stream in listener.incoming(){ //Handle each incoming request.
		handle_request(&mut stream.expect("Error: Failed to load stream")); //Call request handler function.
	}
}

fn handle_request(stream: &mut TcpStream){ //Handles the response to the Tcp request.
	let mut buffer: [u8; 1024] = [0; 1024]; //Create a buffer to hold request data.
	stream.read(&mut buffer).expect("Error: Failed to read from stream"); //Read the request data into the buffer.

	let mut file = File::open("hello.html").expect("Error: Failed to open file"); //Open file.
	let mut contents = Vec::new(); //html buffer
	file.read_to_end(&mut contents).expect("Frrow: Failed to read file"); //Read file contents into html buffer.
	drop(file); //Close the file.

	let header = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n"; //Header data.

	stream.write_all(&header.as_bytes()).expect("Erorr: Failed to write to headers"); //Write out head back to client.
	stream.write_all(&contents).expect("Error: Failed to write file contents"); //Write out html data back to client.
}