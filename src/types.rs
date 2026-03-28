use std::io::{self, Result};
use std::path::{Path, PathBuf};
use std::thread::{self, Thread};
use strum::FromRepr;
use tokio::io::AsyncWriteExt;
use tokio::{
    self,
    net::{UnixListener, UnixStream},
};

// Chapter 6.1
#[derive(FromRepr, Debug, PartialEq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum msgSendType {
    SSH_AGENTC_REQUEST_IDENTITIES = 11,
    SSH_AGENTC_SIGN_REQUEST = 13,
    SSH_AGENTC_ADD_IDENTITY = 17,
    SSH_AGENTC_REMOVE_IDENTITY = 18,
    SSH_AGENTC_REMOVE_ALL_IDENTITIES = 19,
    SSH_AGENTC_ADD_SMARTCARD_KEY = 20,
    SSH_AGENTC_REMOVE_SMARTCARD_KEY = 21,
    SSH_AGENTC_LOCK = 22,
    SSH_AGENTC_UNLOCK = 23,
    SSH_AGENTC_ADD_ID_CONSTRAINED = 25,
    SSH_AGENTC_ADD_SMARTCARD_KEY_CONSTRAINED = 26,
    SSH_AGENTC_EXTENSION = 27,
}

// Chapter 6.1
#[derive(FromRepr, Debug, PartialEq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum msgReplyType {
    SSH_AGENT_FAILURE = 5,
    SSH_AGENT_SUCCESS = 6,
    SSH_AGENT_IDENTITIES_ANSWER = 12,
    SSH_AGENT_SIGN_RESPONSE = 14,
    SSH_AGENT_EXTENSION_FAILURE = 28,
    SSH_AGENT_EXTENSION_RESPONSE = 29,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum msgType {
    msgSendType(msgSendType),
    msgReplyType(msgReplyType),
}

#[derive(Debug)]
pub struct Msg {
    pub length: u32,
    pub msgtype: msgType,
    pub contents: Vec<u8>,
}

#[allow(clippy::ptr_arg)]
impl Msg {
    pub fn parse_reply(raw_msg: &Vec<u8>) -> Msg {
        // parses the reply, as specified in 6.1 and 3
        let length = u32::from_be_bytes([raw_msg[0], raw_msg[1], raw_msg[2], raw_msg[3]]); // First 4 bytes 
        let mut contents: Vec<u8> = Vec::new();
        if length > 1 {
            contents.extend_from_slice(&raw_msg[6..length as usize - 1]); // I would append this above to the new but because it mutates the vec, it returns nothing, so contents would be nothing
        }
        Msg {
            length,                                                                       // First 4 bytes
            msgtype: msgType::msgReplyType(msgReplyType::from_repr(raw_msg[4]).unwrap()), // TODO: get rid of this ugly unwrap
            contents,
        }
    }
    pub fn parse_sent(raw_msg: &Vec<u8>) -> Msg {
        // parses the request, as specified as 6.1 and 3
        let length = u32::from_be_bytes([raw_msg[0], raw_msg[1], raw_msg[2], raw_msg[3]]);
        let mut contents: Vec<u8> = Vec::new();
        if length > 1 {
            contents.extend_from_slice(&raw_msg[6..length as usize - 1]); // I would append this above to the new but because it mutates the vec, it returns nothing, so contents would be nothing
        }
        Msg {
            length,                                                                     // First 4 bytes
            msgtype: msgType::msgSendType(msgSendType::from_repr(raw_msg[4]).unwrap()), // TODO: get rid of this ugly unwrap
            contents,
        }
    }
    pub fn handle_reply(&mut self) {
        //match &self.msgtype {
        //    msgSendType::SSH_AGENTC_ADD_IDENTITY =>
        //}
    }
    pub fn handle_request(&mut self) {}
}

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
            vec![
                0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 11, 115, 115, 104, 45, 101, 100, 50, 53, 53, 49,
                57, 0, 0, 0, 32, 64, 5, 6, 0, 240, 201, 106, 56, 164, 13, 201, 184, 240, 26, 214,
                212, 237, 138, 225, 195, 38, 221, 171, 20, 9, 185, 12, 226, 120, 134, 224, 126, 0,
                0, 0, 7, 104, 112, 108, 101, 97, 114, 110, 0, 0, 0, 51, 0, 0, 0, 11, 115, 115, 104,
                45, 101, 100, 50, 53, 53, 49, 57, 0, 0, 0, 32, 60, 168, 193, 237, 39, 72, 132, 211,
                127, 169, 105, 237, 66, 138, 137, 115, 46, 172, 91, 160, 117, 231, 218, 24, 157,
                134, 188, 216, 62, 204, 150, 237, 0, 0, 0, 8, 114, 101, 110, 101, 103, 97, 100,
                101, 0, 0, 0, 51, 0, 0, 0, 11, 115, 115, 104, 45, 101, 100, 50, 53, 53, 49, 57, 0,
                0, 0, 32, 115, 243, 127, 67, 176, 129, 80, 87, 11, 106, 159, 140, 77, 63, 214, 245,
                123, 1, 206, 96, 69, 113, 159, 189, 1, 0, 214, 160, 89, 214, 48, 38, 0, 0, 0, 6,
                71, 73, 84, 72, 85, 66, 0, 0, 0, 51, 0, 0, 0, 11, 115, 115, 104, 45, 101, 100, 50,
                53, 53, 49, 57, 0, 0, 0, 32, 161, 176, 208, 34, 137, 57, 53, 169, 46, 105, 64, 173,
                176, 7, 139, 227, 60, 15, 127, 201, 167, 249, 188, 152, 245, 167, 226, 83, 87, 42,
                254, 198, 0, 0, 0, 6, 79, 114, 97, 99, 108, 101, 0, 0, 0, 51, 0, 0, 0, 11, 115,
                115, 104, 45, 101, 100, 50, 53, 53, 49, 57, 0, 0, 0, 32, 229, 116, 134, 57, 79,
                118, 8, 83, 52, 171, 22, 155, 199, 167, 68, 184, 229, 141, 65, 74, 171, 104, 196,
                159, 167, 204, 170, 208, 105, 158, 55, 210, 0, 0, 0, 8, 100, 101, 115, 107, 116,
                111, 112, 50, 0, 0, 0, 51, 0, 0, 0, 11, 115, 115, 104, 45, 101, 100, 50, 53, 53,
                49, 57, 0, 0, 0, 32, 156, 8, 57, 56, 230, 120, 141, 149, 206, 151, 146, 98, 37,
                219, 59, 4, 187, 52, 114, 94, 44, 126, 246, 150, 156, 58, 177, 21, 161, 151, 178,
                24, 0, 0, 0, 8, 75, 117, 98, 101, 114, 109, 97, 110, 0, 0, 0, 51, 0, 0, 0, 11, 115,
                115, 104, 45, 101, 100, 50, 53, 53, 49, 57, 0, 0, 0, 32, 189, 58, 218, 16, 62, 159,
                166, 43, 101, 212, 67, 218, 196, 122, 155, 186, 139, 0, 57, 72, 111, 44, 61, 247,
                65, 224, 147, 171, 88, 111, 118, 93, 0, 0, 0, 5, 116, 111, 119, 101, 114, 0, 0, 0,
                74, 0, 0, 0, 26, 115, 107, 45, 115, 115, 104, 45, 101, 100, 50, 53, 53, 49, 57, 64,
                111, 112, 101, 110, 115, 115, 104, 46, 99, 111, 109, 0, 0, 0, 32, 196, 123, 77,
                247, 186, 112, 75, 198, 252, 138, 54, 40, 224, 247, 35, 146, 12, 75, 244, 145, 102,
                238, 175, 60, 200, 245, 67, 57, 139, 44, 146, 115, 0, 0, 0, 4, 115, 115, 104, 58,
                0, 0, 0, 21, 121, 117, 98, 105, 95, 114, 101, 109, 111, 116, 101, 95, 115, 117,
                100, 111, 95, 109, 97, 105, 110, 0, 0, 0, 74, 0, 0, 0, 26, 115, 107, 45, 115, 115,
                104, 45, 101, 100, 50, 53, 53, 49, 57, 64, 111, 112, 101, 110, 115, 115, 104, 46,
                99, 111, 109, 0, 0, 0, 32, 216, 163, 239, 133, 87, 209, 151, 17, 216, 228, 74, 245,
                241, 29, 55, 182, 181, 253, 210, 189, 142, 248, 224, 146, 230, 89, 44, 83, 41, 165,
                183, 108, 0, 0, 0, 4, 115, 115, 104, 58, 0, 0, 0, 23, 121, 117, 98, 105, 95, 114,
                101, 109, 111, 116, 101, 95, 115, 117, 100, 111, 95, 98,
            ],
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
