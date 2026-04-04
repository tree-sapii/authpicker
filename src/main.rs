//use std::io::prelude::*;
//use std::os::unix::net::UnixStream;
//

pub mod client;
pub mod key;
pub mod msg;
pub mod parse;
pub mod server;
pub mod types;
use crate::types::{Client, Server};
//
//fn main() -> std::io::Result<()> {
//    let mut stream = UnixStream::connect(std::env::var("SSH_AUTH_SOCK").unwrap())?;
//    let data: [u8; 5] = [0, 0, 0, 1, 11];
//    println!("{:?}", data);
//    stream.write_all(&data)?;
//    let mut response = [0u8; 4096];
//    stream.read(&mut response).unwrap();
//    println!("{}", String::from_utf8_lossy(&response));
//    Ok(())
//}

#[tokio::main]
async fn main() {
    //Server::new("./textsock").start().await.unwrap();
    Client::new(std::env::var("SSH_AUTH_SOCK").unwrap())
        .send_msg()
        .await
        .unwrap();
}
