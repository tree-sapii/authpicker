use crate::msg::{msgReplyType, msgSendType, msgType};
use std::io::{self, Result};
use std::path::{Path, PathBuf};
use tokio::{
    self,
    net::{UnixListener, UnixStream},
};

pub struct Client {
    socket_path: PathBuf,
}

impl Client {
    pub fn new(sock_path: impl AsRef<Path>) -> Client {
        // Create the server obj
        Client {
            socket_path: PathBuf::from(&sock_path.as_ref()),
        }
    }
    pub async fn send_msg(&self) -> Result<()> {
        // This starts the server event loop that handles requests
        let socket_stream: UnixStream = UnixStream::connect(&self.socket_path).await?;
        socket_stream.try_write(&[0, 0, 0, 1, 11]);
        loop {
            // Loop over accepted connections
            socket_stream.readable().await?;
            println!("{:?}", Self::handle_response(&socket_stream).await.unwrap());
        }
    }
    async fn handle_response(connection: &UnixStream) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(4096);
        connection.try_read_buf(&mut buf)?;
        println!("{:?}", String::from_utf8_lossy(&buf));
        println!("{:?}", Msg::parse_reply(&buf)); // Just decode it
        Ok(buf)
    }
}

struct App {
    client: Client,
    server: Server,
}
