use clap::Parser;
use dotenv::dotenv;

use std::{
    env,
    io::{Read, Write},
    net::TcpStream,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    dotenv().ok();

    let addr = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");

    let args = Args::parse();

    if let Ok(mut stream) = TcpStream::connect(addr) {
        println!("Connected to tcp server...");

        const SIZE: usize = 1018;
        if args.name.len() > SIZE {
            panic!(
                "input size of {:?} exceed maximum size {:?}",
                args.name.len(),
                SIZE
            );
        }

        let msg = format!("Hello {}", args.name);

        stream.write(msg.as_bytes()).unwrap();

        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let res = std::str::from_utf8(&buffer).expect("Found invalid UTF-8");

        println!(
            "Got response from server:{:?}",
            res.trim_end_matches(char::from(0))
        );
    } else {
        println!("Couldn't connect tcp server...");
    }
}
