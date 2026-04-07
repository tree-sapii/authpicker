use crate::key::Key;
use crate::msg::{Msg, msgReplyType, msgSendType, msgType};
use std::io::{self, BufReader, Result};
use std::path::{Path, PathBuf};
use std::thread::{self, Thread};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
    async fn handle_msg(mut connection: UnixStream) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(4096);
        connection.try_read_buf(&mut buf)?;

        let msg = Msg::server_parse_msg_recieved(&buf);
        println!("got {:?}", msg);
        let repsonse: (Msg, Vec<Key>);
        match msg.msgtype {
            msgType::msgSendType(ref reply) => match reply {
                msgSendType::SSH_AGENTC_REQUEST_IDENTITIES => {
                    println!("Forwarding");
                    repsonse = Server::forward_msg(
                        [0, 0, 0, 1, 11].try_into().unwrap(),
                        std::env::var("SSH_AUTH_SOCK").unwrap().to_string(),
                        "yubai".to_string(),
                    )
                    .await
                    .unwrap();
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
        let mut byte_reponse = msg.server_build_msg(repsonse.1).unwrap();

        connection.write(&byte_reponse);
        Ok(buf)
    }
    //async fn send_resp(connection: UnixStream, msg: Msg) -> Result<()> {
    //    let mut buf: Vec<u8> = Vec::new();
    //    let length = content.len() + 1; // + 1 is the resptype as its u8
    //    buf.extend_from_slice(&length.to_be_bytes());
    //    buf.push(resptype as u8);
    //    buf.extend_from_slice(&content);
    //    connection.try_write(&buf)?;
    //    Ok(())
    //}

    pub async fn forward_msg(
        buf: Vec<u8>,
        socket_path: String,
        filter: String,
    ) -> Result<(Msg, Vec<Key>)> {
        let mut socket_stream: UnixStream = UnixStream::connect(&socket_path).await?;
        println!("Client connected to {}, buf is {:?}", socket_path, buf);
        socket_stream.write(&buf).await.unwrap();
        let mut buf: [u8; 4096] = [0; 4096];
        socket_stream.read(&mut buf).await.unwrap();
        let msg = Msg::client_parse_msg_recieved(&buf.to_vec());
        println!("{:?}", msg);
        let mut keys: Vec<Key> = msg.filter_shown_keys(&filter).unwrap();

        Ok((msg, keys))
    }
}
