use crate::key::Key;
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
            msgType::msgReplyType(replyType) => {
                let mut reply: Vec<u8> = Vec::new();
                reply.extend(self.length.to_be_bytes());
                reply.push(std::ptr::addr_of!(reply) as u8); // need to cast to a pointer to
                // move out of reference works to be able to cast it as u8 TODO: Make this look
                // better
                reply.extend(self.contents.clone());
                Ok(reply)
            }
        }
    }

    pub fn server_build_msg(&self, keys: Vec<Key>) -> Result<Vec<u8>> {
        match &self.msgtype {
            msgType::msgSendType(sendType) => {
                let mut reply: Vec<u8> = Vec::new();
                println!("{}", &self.length);
                reply.extend(self.length.to_be_bytes());
                reply.push(12 as u8); // need to cast to a pointer to
                // move out of reference works to be able to cast it as u8 TODO: Make this look
                // better
                println!("{:?}", keys);
                reply.extend(keys.len().to_be_bytes());
                for key in keys {
                    reply.extend(&key.keyblob.len().to_be_bytes());
                    reply.extend(&key.keyblob);
                    reply.extend(&key.to_bytes());
                }
                println!("{:?}", reply);
                println!("{:?}", Msg::client_parse_msg_recieved(&reply));
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

    fn get_keys_from_answ(&self) -> Vec<Key> {
        let mut contents_iter = self.contents.clone().into_iter();
        let nkeys: u32 = u32::from_be_bytes(
            Msg::extract_n_bytes(&mut contents_iter, 4)
                .try_into()
                .unwrap(),
        );
        let mut keylist: Vec<Key> = Vec::with_capacity(nkeys as usize);
        for key in 1..nkeys {
            keylist.push(Key::from(&mut contents_iter));
        }
        keylist
    }

    pub fn filter_shown_keys(&self, filter: &String) -> Result<Vec<Key>> {
        match &self.msgtype {
            msgType::msgReplyType(replyType) => {
                return Ok(self
                    .get_keys_from_answ()
                    .into_iter()
                    .filter(|key| !key.commentstr.contains(filter))
                    .collect());
            }
            _ => panic!("Message is the wrong type"),
        }
    }
}
