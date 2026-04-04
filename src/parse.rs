use std::vec;

use crate::{
    key::{self, Key},
    msg::msgSendType,
    types::{Msg, msgReplyType, msgSendType, msgType},
};
use log::{error, info, warn};

impl Msg {
    pub fn create_reply(&mut self) {
        match &self.msgtype {
            // Splits the message handling in 2 paths
            msgType::msgSendType(msgSendType) => {}
            msgType::msgReplyType(msgReplyType) => {}
        }
    }
    fn create_send_reply(&mut self, send_type: msgSendType) -> Vec<u8> {
        match &send_type {
            // Match everything to its respective function
            msgSendType::SSH_AGENTC_REQUEST_IDENTITIES => return self.add_identities_handle(),
            msgSendType::SSH_AGENTC_SIGN_REQUEST => return self.sign_req_handle(),
            msgSendType::SSH_AGENTC_ADD_IDENTITY => return self.add_identities_handle(),
            msgSendType::SSH_AGENTC_REMOVE_IDENTITY => return self.remove_identities_handle(),
            msgSendType::SSH_AGENTC_REMOVE_ALL_IDENTITIES => {
                return self.remove_all_identities_handle();
            }
            msgSendType::SSH_AGENTC_ADD_SMARTCARD_KEY => return self.add_smartcard_handle(),
            msgSendType::SSH_AGENTC_REMOVE_SMARTCARD_KEY => return self.remove_smartcard_handle(),
            msgSendType::SSH_AGENTC_LOCK => return self.lock_handle(),
            msgSendType::SSH_AGENTC_UNLOCK => return self.unlock_handle(),
            msgSendType::SSH_AGENTC_ADD_ID_CONSTRAINED => return self.add_id_c_handle(),
            msgSendType::SSH_AGENTC_ADD_SMARTCARD_KEY_CONSTRAINED => {
                return self.add_smartcard_c_handle();
            }
            msgSendType::SSH_AGENTC_EXTENSION => return self.extenstion_handle(),
        }
    }
    fn create_reply_reply(&mut self, reply_type: msgReplyType) {
        match reply_type {
            msgReplyType::SSH_AGENT_FAILURE => return self.success_handle(),
            msgReplyType::SSH_AGENT_SUCCESS => return self.fail_handle(),
            msgReplyType::SSH_AGENT_IDENTITIES_ANSWER => return self.identities_answer_handle(),
            msgReplyType::SSH_AGENT_SIGN_RESPONSE => return self.sign_response_handle(),
            msgReplyType::SSH_AGENT_EXTENSION_FAILURE => return self.extension_fail(),
            msgReplyType::SSH_AGENT_EXTENSION_RESPONSE => return self.extension_response_handle(),
        }
        // TODO: please have better naming for this function
    }

    fn create_ident_req_msg() -> Vec<u8> {
        let length: u32 = 1;
        let mut msg: Vec<u8> = Vec::new();
        msg.extend(length.to_ne_bytes());
        msg.push(msgSendType::SSH_AGENTC_REQUEST_IDENTITIES as u8);
        msg
    }
    fn create_agent_sign_req_msg(data: Vec<u8>, key: Key, flags: u32) -> Vec<u8> {
        let mut msg: Vec<u8> = Vec::new();
        msg.insert(4, msgSendType::SSH_AGENTC_SIGN_REQUEST as u8);
        msg.extend(key.to_ssh_str());
        msg.extend(data); // Passtrhu data
        msg.extend(flags.to_be_bytes()); // Passtrhu flags
        msg
    }

    fn create_lock_req(msg: Vec<u8>) -> Vec<u8> {
        msg // Just pass thru the msg
    }

    fn success_handle(&self) {
        info!("Req success");
    }
    fn fail_handle(&self) {
        error!("Req failed");
    }
    fn identities_answer_handle(&self) {}
    fn sign_response_handle(&self) {}
    fn extension_fail(&self) {}
    fn extension_response_handle(&self) {}

    fn identities_req_handle(&self) -> Vec<u8> {}
    fn sign_req_handle(&self) -> Vec<u8> {}
    fn add_identities_handle(&self) -> Vec<u8> {}
    fn remove_identities_handle(&self) -> Vec<u8> {}
    fn remove_all_identities_handle(&self) -> Vec<u8> {}
    fn add_smartcard_handle(&self) -> Vec<u8> {}
    fn remove_smartcard_handle(&self) -> Vec<u8> {}
    fn lock_handle(&self) -> Vec<u8> {}
    fn unlock_handle(&self) -> Vec<u8> {}
    fn add_id_c_handle(&self) -> Vec<u8> {}
    fn add_smartcard_c_handle(&self) -> Vec<u8> {}
    fn extenstion_handle(&self) -> Vec<u8> {}
}
