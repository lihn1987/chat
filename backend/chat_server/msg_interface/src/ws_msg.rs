
use serde::{Deserialize, Serialize};
use secp256k1::{Secp256k1, Message, PublicKey, Signature};
use sha256::digest;
extern crate hex;
// #[derive(Serialize, Deserialize)]
// pub enum MsgType {
//     MsgLogin = 0,
//     MsgTxt = 1
// }

pub const MSG_LOGIN:u32 = 0;
pub const MSG_TXT:u32 = 1;
pub const MSG_AUDIO:u32 = 2;
pub const MSG_CLEAR_UNREAD:u32 = 101;
pub const MSG_IMAGE:u32 = 3;
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MsgBase{
    pub msg_type: u32,
    pub from: String,
    pub to: String,
    pub msg: String,
    pub time_stamp: u64
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MsgSigned {
    pub base_msg: MsgBase,
    pub sign: String
}

impl MsgSigned {
    /// 验证签名是否正确
    pub fn check_sign(&self) -> bool{
        let pubkey = self.base_msg.from.to_string();
        let msg_base = serde_json::to_string(&self.base_msg).unwrap();
        let msg_sha = digest(msg_base);
        let secp = Secp256k1::new();
        let verify_msg = Message::from_slice(&hex::decode(&msg_sha).expect("Decoding failed")).unwrap();
        let verify_pubkey = PublicKey::from_slice(&hex::decode(&pubkey).expect("Decoding failed")).unwrap();
        let verify_sig = Signature::from_compact(&hex::decode(&self.sign).expect("Decoding failed")).unwrap();
        secp.verify(&verify_msg, &verify_sig, &verify_pubkey).is_ok()
    }
}
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub msg_type: u32,
    pub result: bool,
    pub data: String
}

