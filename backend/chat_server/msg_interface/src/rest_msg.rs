
use serde::{Deserialize, Serialize};
use secp256k1::{Secp256k1, Message, PublicKey, Signature};
use sha256::digest;
extern crate hex;

use crate::mongo_msg::{UserInfo};

// 通用消息框架
#[derive(Deserialize, Serialize)]
pub struct RequestBase {
    pub from: String,
    pub timestamp: u64,
    pub param: String
}

pub const  ERROR_NO_POWER:u32 = 1;
pub const ERROR_NO_PUBKEY_FINDED:u32 = 2;
pub const ERROR_ADD_FRIEND: u32 = 3;
pub const ERROR_CHANGE_NICK_NAME: u32 = 4;
#[derive(Deserialize, Serialize)]
pub struct RequestComm {
    pub msg: RequestBase,
    pub sign: String
}

impl RequestComm {
    /// 验证签名是否正确
    pub fn check_sign(&self) -> bool{
        let pubkey = self.msg.from.to_string();
        let msg_base = serde_json::to_string(&self.msg).unwrap();
        let msg_sha = digest(msg_base);
        let secp = Secp256k1::new();
        let verify_msg = Message::from_slice(&hex::decode(&msg_sha).expect("Decoding failed")).unwrap();
        let verify_pubkey = PublicKey::from_slice(&hex::decode(&pubkey).expect("Decoding failed")).unwrap();
        let verify_sig = Signature::from_compact(&hex::decode(&self.sign).expect("Decoding failed")).unwrap();
        secp.verify(&verify_msg, &verify_sig, &verify_pubkey).is_ok()
    }
}

// 添加好友
#[derive(Deserialize, Serialize)]
pub struct AddFriendByPubkeyRequest {
    pub pubkey: String
}

// 修改昵称
#[derive(Deserialize, Serialize)]
pub struct ChangeNickNameRequest {
    pub nick_name: String
}

#[derive(Deserialize, Serialize)]
pub struct ResponseComm {
    pub error: u32,
    pub data: String
}
#[derive(Deserialize, Serialize)]
pub struct AddFriendByPubkeyResponse {
    pub pubkey: String,
    pub nick: String,
    pub sex: u8, // 0 unknow, 1 man, 2 woman 
    pub age: u8,
}
impl AddFriendByPubkeyResponse {
    pub fn from_user_info(user_info: &UserInfo) -> AddFriendByPubkeyResponse{
        AddFriendByPubkeyResponse {
            pubkey: user_info.pubkey.clone(),
            nick: user_info.nick.clone(),
            sex: user_info.sex,
            age: user_info.age
        }
    }
}
