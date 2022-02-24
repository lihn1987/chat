
use serde::{Deserialize, Serialize};
use secp256k1::{Secp256k1, Message, PublicKey, Signature};
use sha256::digest;
extern crate hex;

use crate::mongo_msg::{UserInfo, GroupInfo};

// 通用消息框架
#[derive(Deserialize, Serialize)]
pub struct RequestBase {
    pub from: String,
    pub timestamp: u64,
    pub param: String
}

pub const ERROR_NO_POWER:u32 = 1;
pub const ERROR_NO_PUBKEY_FINDED:u32 = 2;
pub const ERROR_ADD_FRIEND: u32 = 3;
pub const ERROR_CHANGE_NICK_NAME: u32 = 4;
pub const ERROR_GROUP_NAME_SYMBOL_ERROR: u32 = 5;   // 群创建错误，格式不正确
pub const ERROR_GROUP_NAME_REPEAT: u32 = 6;         // 群创建错误，群名重复
pub const ERROR_JOIN_GROUP_NO_GROUP: u32 = 7;       // 加群时，群不存在
pub const ERROR_JOIN_GROUP_ALREADY_EXIST: u32 = 8;  // 加群时已在群中

pub const ERROR_PARAM_ERROR: u32 = 21;               // 参数错误


pub const GROUP_ACCESS_ANYONE: u32 = 1;             // 群进入权限，所以人可加入
pub const GROUP_ACCESS_NEED_OWNER_ALLOW: u32 = 2;   // 群进入权限，需要拥有者允许


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

#[derive(Deserialize, Serialize)]
pub struct ResponseComm {
    pub error: u32,
    pub data: String
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

// 创建群
#[derive(Deserialize, Serialize)]
pub struct CreateGroupRequest {
    pub name: String,           // 群名称  
    pub access: u32,            // 进入许可
    pub visible_by_name: bool   // 是否允许通过群名被查找到
}

impl CreateGroupRequest {
    pub fn ToGroupInfo(&self, pubkey: &str, owner: &str) -> GroupInfo {
        GroupInfo{
            pubkey: pubkey.to_string(),
            name: self.name.clone(),
            access: self.access,
            visible_by_name: self.visible_by_name,
            owner: owner.to_string(),
            members: vec![]
        }
    }
}

/* 
返回：
    error: 错误号
    data: 成功时返回群pubkey
*/

// 加入群
/*
请求：
    param 要加入的群pubkey

返回
    error: 错误号
    data: 成功时返回群GroupInfo
*/


// 踢某人出群
#[derive(Deserialize, Serialize)]
pub struct CreateKickRequest {
    pub group_pubkey: String,           // 群pubkey
    pub user_pubkey: String,            // 用户pubkey
}

/*
返回
    error: 错误号
    data: 无
*/