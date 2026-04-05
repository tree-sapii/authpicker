use core::panic;
use std::io::Result;
use strum::FromRepr;
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

pub struct KeyListMsg {
    pub length: u32,
    pub msgtype: msgType,
    pub nkeys: u32,
    pub keyList: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct Msg {
    pub length: u32,
    pub msgtype: msgType,
    pub contents: Vec<u8>,
}

#[allow(clippy::ptr_arg)]
impl Msg {
    pub fn client_parse_msg_recieved(raw_msg: &Vec<u8>) -> Msg {
        // parses the reply, as specified in 6.1 and 3
        let length = u32::from_be_bytes([raw_msg[0], raw_msg[1], raw_msg[2], raw_msg[3]]); // First 4 bytes 
        let mut contents: Vec<u8> = raw_msg[5..(length + 4) as usize].to_vec(); // The +4 skips the
        // length field becase it the specified a the beggining of the message is no included in the
        // length number, only the content's length is calculated

        Msg {
            length,                                                                       // First 4 bytes
            msgtype: msgType::msgReplyType(msgReplyType::from_repr(raw_msg[4]).unwrap()), // TODO: get rid of this ugly unwrap
            contents,
        }
    }

    pub fn client_build_msg(&self) -> Result<Vec<u8>> {
        match &self.msgtype {
            _ => panic!(),
            msgType::msgSendType(sendType) => {
                let mut reply: Vec<u8> = Vec::new();
                reply.extend(self.length.to_be_bytes());
                reply.push(std::ptr::addr_of!(sendType) as u8); // need to cast to a pointer to
                // move out of reference works to be able to cast it as u8 TODO: Make this look
                // better
                reply.extend(self.contents.clone());
                Ok(reply)
            }
        }
    }

    pub fn server_build_msg(&self) -> Result<Vec<u8>> {
        match &self.msgtype {
            msgType::msgReplyType(replyType) => {
                let mut reply: Vec<u8> = Vec::new();
                reply.extend(self.length.to_be_bytes());
                reply.push(std::ptr::addr_of!(replyType) as u8); // need to cast to a pointer to
                // move out of reference works to be able to cast it as u8 TODO: Make this look
                // better
                reply.extend(self.contents.clone());
                Ok(reply)
            }
            _ => panic!(),
        }
    }
    pub fn server_parse_msg_recieved(raw_msg: &Vec<u8>) -> Msg {
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

    fn extract_n_bytes<I>(buf_iter: &mut I, num: usize) -> Vec<u8>
    where
        I: Iterator<Item = u8>,
    {
        buf_iter.take(num).collect()
    }

    pub fn destructure_key_list_resp(self) -> Result<Vec<(String, String)>> {
        match &self.msgtype {
            msgType::msgReplyType(replyType) => {
                let mut list = self.contents.into_iter();
                let nkeys: u32 =
                    u32::from_be_bytes(Msg::extract_n_bytes(&mut list, 4).try_into().unwrap());

                // From inspecting, [0,0,0,51] is used to tell the entire length of the key blob,
                // excluding the comment
                //

                for i in 0..nkeys {
                    let total_key_blob_length =
                        u32::from_be_bytes(Msg::extract_n_bytes(&mut list, 4).try_into().unwrap());

                    let key_type_blob_length =
                        u32::from_be_bytes(Msg::extract_n_bytes(&mut list, 4).try_into().unwrap())
                            as usize;

                    let keytypeblob = String::from_utf8_lossy(&Msg::extract_n_bytes(
                        &mut list,
                        key_type_blob_length,
                    ))
                    .to_string();

                    let keyblob_length =
                        u32::from_be_bytes(Msg::extract_n_bytes(&mut list, 4).try_into().unwrap())
                            as usize;

                    let keyblob: String =
                        String::from_utf8_lossy(&Msg::extract_n_bytes(&mut list, keyblob_length))
                            .to_string();

                    let comment_length =
                        u32::from_be_bytes(Msg::extract_n_bytes(&mut list, 4).try_into().unwrap())
                            as usize;

                    let commentblob: String =
                        String::from_utf8_lossy(&Msg::extract_n_bytes(&mut list, comment_length))
                            .to_string();

                    if !commentblob.contains("yubi") {}

                    if commentblob.contains("ssh:") && keytypeblob.contains("@openssh.com") {
                        let extra_length = u32::from_be_bytes(
                            Msg::extract_n_bytes(&mut list, 4).try_into().unwrap(),
                        ) as usize;

                        let extrablob: String =
                            String::from_utf8_lossy(&Msg::extract_n_bytes(&mut list, extra_length))
                                .to_string();

                        if !extrablob.contains("yubi") {
                            println!(
                                "Total length: {} \n Keyblob type len: {} \n Keyblob type: {} \n Keyblob len: {} \n Keyblob: {} \n Comment len: {}, \n Comment: {}, \n Extra len: {} \n Extra blob {} \n ",
                                total_key_blob_length,
                                key_type_blob_length,
                                keytypeblob,
                                keyblob_length,
                                keyblob,
                                comment_length,
                                commentblob,
                                extra_length,
                                extrablob
                            );
                        };
                    } else {
                        println!(
                            "Total length: {} \n Keyblob type len: {} \n Keyblob type: {} \n Keyblob len: {} \n Keyblob: {} \n Comment len: {}, \n Comment: {}",
                            total_key_blob_length,
                            key_type_blob_length,
                            keytypeblob,
                            keyblob_length,
                            keyblob,
                            comment_length,
                            commentblob,
                        );
                    };
                }

                //let key_str: String =
                //    String::from_utf8_lossy(&self.contents[3..self.contents.len() - 1]).to_string();

                //let key_vec: Vec<&str> = key_str.split("\n").collect();

                //for str in key_vec {
                //    println!("{}", str);
                //}
                Ok(Vec::new())
            }
            _ => panic!(),
        }
    }
}
