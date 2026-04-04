use std::io::{self, Result};
use std::path::{Path, PathBuf};
use std::thread::{self, Thread};
use tokio::io::AsyncWriteExt;
use tokio::{
    self,
    net::{UnixListener, UnixStream},
};

pub struct Server {
    socket_path: PathBuf,
}

impl Server {
    pub fn new(sock_path: impl AsRef<Path>) -> Server {
        // Create the server obj
        Server {
            socket_path: PathBuf::from(&sock_path.as_ref()),
        }
    }
    pub async fn start(&self) -> Result<()> {
        // This starts the server event loop that handles requests
        let socket_stream = UnixListener::bind(&self.socket_path)?;
        loop {
            // Loop over accepted connections
            let (connection, _) = socket_stream.accept().await?;
            connection.readable().await?;
            tokio::spawn(async move {
                // Spawn a new thread handler for each
                Self::handle_msg(connection).await.unwrap();
            });
        }
    }
    async fn handle_msg(connection: UnixStream) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(4096);
        connection.try_read_buf(&mut buf)?;
        println!("{:?}", String::from_utf8_lossy(&buf));
        println!("{:?}", Msg::parse_sent(&buf)); // Just decode it
        Self::send_resp(
            connection,
            msgReplyType::SSH_AGENT_IDENTITIES_ANSWER,
            vec![],
        )
        .await
        .unwrap();
        Ok(buf)
    }
    async fn send_resp(
        connection: UnixStream,
        resptype: msgReplyType,
        content: Vec<u8>,
    ) -> Result<()> {
        let mut buf: Vec<u8> = Vec::new();
        let length = content.len() + 1; // + 1 is the resptype as its u8
        buf.extend_from_slice(&length.to_be_bytes());
        buf.push(resptype as u8);
        buf.extend_from_slice(&content);
        connection.try_write(&buf)?;
        Ok(())
    }
}

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

impl App {}
