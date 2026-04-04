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
