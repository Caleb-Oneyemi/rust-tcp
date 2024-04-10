use dotenv::dotenv;
use std::{
    env,
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    dotenv().ok();

    let addr = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");

    let conn = TcpListener::bind(&addr).unwrap();

    println!("tcp server running on {}", addr);

    for stream in conn.incoming() {
        let mut stream = stream.unwrap();

        println!("connection established");

        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        stream.write(&mut buffer).unwrap();

        stream.flush().unwrap();
    }
}
