use std::{
	fs::{self, File},
	io::prelude::*,
	net::{TcpListener, TcpStream},
	path::Path,
	rc::Rc,
};

const ADDRESS: &str = "127.0.0.1:8080";

fn main() {
	let root_path = Path::new("root");
	let root_path_buffer: Rc<std::path::PathBuf>;
	match fs::canonicalize(root_path) {
		Ok(result) => {
			root_path_buffer = result.into();
		}
		Err(e) => {
			eprintln!("ERROR: FAILED TO FIND ROOT DIRECTORY! {}", &e);
			std::process::exit(1);
		}
	}
	match TcpListener::bind(ADDRESS) {
		Ok(listener) => {
			println!("Listening on {ADDRESS}");
			for stream in listener.incoming() {
				match stream {
					Ok(mut stream) => {
						let request_copy = Rc::clone(&root_path_buffer);
						handle_request(&mut stream, request_copy);
					}
					Err(e) => {
						eprintln!("Failed to accept connection: {}", &e);
					}
				}
			}
		}
		Err(e) => {
			eprintln!("Failed to bind to {ADDRESS}: {}", &e);
		}
	}
}

fn handle_request(stream: &mut TcpStream, root_path_buffer: Rc<std::path::PathBuf>) {
	let mut buffer: [u8; 1024] = [0; 1024];
	match stream.read(&mut buffer) {
		Ok(_length) => {
			let request_string = String::from_utf8_lossy(&buffer);
			print!("{}", &request_string);
			let request_data: Vec<&str> = request_string.split_ascii_whitespace().collect();
			match (
				request_data.get(0),
				request_data.get(1),
				request_data.get(2),
			) {
				(Some(&"GET"), Some(request_path), Some(&"HTTP/1.1")) => {
					let request_path = format!("root{}", request_path);
					let file_path = Path::new(&request_path);
					let requested_path_buffer: std::path::PathBuf;
					match fs::canonicalize(&request_path) {
						Ok(result) => {
							requested_path_buffer = result;
						}
						Err(e) => {
							println!("Failed to canonicalize path: {}", &e);
							let header_content = "HTTP/1.1 500 Internal Server Error\r\nConnection: close\r\n\r\n";
							send_response(stream, &header_content);
							return;
						}
					}
					if !requested_path_buffer.starts_with(&*root_path_buffer) {
							let header_content = "HTTP/1.1 403 Forbidden\r\nConnection: close\r\n\r\n";
							send_response(stream, &header_content);
						return;
					}
					if file_path.exists() {
						match File::open(&file_path) {
							Ok(mut file) => {
								let mut file_content: Vec<u8> = Vec::new();
								match file.read_to_end(&mut file_content) {
									Ok(_length) => {
										let header_content = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
										send_response(stream, &header_content);
										match stream.write_all(&file_content) {
											Err(e) => {
												eprintln!("Fail to send content: {}", &e)
											}
											_ => {}
										}
									}
									Err(e) => {
										eprint!("Error reading file: {}", &e);
										let header_content = "HTTP/1.1 500 Internal Server Error\r\nConnection: close\r\n\r\n";
										send_response(stream, &header_content);
									}
								}
							}
							Err(e) => {
								eprintln!("Error opening file: {}", &e);
								let header_content = "HTTP/1.1 500 Internal Server Error\r\nConnection: close\r\n\r\n";
								send_response(stream, &header_content);
							}
						}
					} else {
						println!("Missing file: {}", &request_path);
						let header_content = "HTTP/1.1 404 Not Found\r\nConnection: close\r\n\r\n";
						send_response(stream, &header_content);
					}
				}
				_ => {
					eprintln!("Bad Request!");
					let response = "HTTP/1.1 405 Method Not Allowed\r\nAllow: GET, HEAD\r\n\r\n";
					send_response(stream, &response);
				}
			}
		}
		Err(e) => {
			eprint!("Failed to read stream: {e}");
		}
	}
}

fn send_response(stream: &mut TcpStream, response_content: &str) {
	match stream.write_all(&response_content.as_bytes()) {
		Err(e) => {
			eprintln!("Fail to send content: {}", &e)
		}
		_ => {}
	}
}