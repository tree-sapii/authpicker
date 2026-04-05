//use std::io::prelude::*;
//use std::os::unix::net::UnixStream;
//

pub mod msg;
pub mod parse;
pub mod types;
use crate::types::Server;
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
    let msg: Vec<u8> = vec![0, 0, 0, 1, 11];
    Server::forward_msg(msg, std::env::var("SSH_AUTH_SOCK").unwrap().to_string()).await;
}
