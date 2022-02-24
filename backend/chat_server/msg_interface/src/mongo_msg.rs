use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Sub};
use std::convert::From;
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub pubkey: String,
    pub nick: String,
    pub sex: u8, // 0 unknow, 1 man, 2 woman 
    pub age: u8,
    pub friends: Vec<String/*pubkey */>,
    pub groups: Vec<String/*pubkey */>
}
 
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserInfoCouldPublic {
    pub pubkey: String,
    pub nick: String,
    pub sex: u8, // 0 unknow, 1 man, 2 woman 
    pub age: u8,
}

impl UserInfoCouldPublic {
    pub fn from(info: UserInfo) -> UserInfoCouldPublic {
        return UserInfoCouldPublic {
            pubkey: info.pubkey,
            nick: info.nick,
            sex: info.sex,
            age: info.age
        }
    }
}

#[derive(Debug, Default,Serialize, Deserialize)]
pub struct MsgHistory {
    pub user_pubkey: String,
    // pub friend_pubkey: String,
    pub history: HashMap<String/* friend pubkey */, MsgHistoryItem>
}

impl MsgHistory {
    pub fn new(user_pubkey: String) -> MsgHistory {
        return MsgHistory{
            user_pubkey: user_pubkey,
            history: [].into(),
        };
    }
}

#[derive(Debug, Default,Serialize, Deserialize)]
pub struct MsgHistoryItem {
    pub unread_count: u64,
    pub msg_list: std::vec::Vec<String>/*MsgSigned json */
}

impl MsgHistoryItem {
    pub fn new(unread_count: u64) -> MsgHistoryItem {
        return MsgHistoryItem {
            unread_count: unread_count,
            msg_list: std::vec::Vec::new()
        };
    }
}

// 获取一个人所有unread附近的聊天记录
#[derive(Debug, Default,Serialize, Deserialize)]
pub struct MsgHistoryAllByUnread {
    list: Vec<MsgHistoryAllByUnreadItem>
}

impl MsgHistoryAllByUnread {
    pub fn from_history(history: &MsgHistory, before: u64, after: u64) -> MsgHistoryAllByUnread{
        let mut  rtn = MsgHistoryAllByUnread {
            list: Vec::new()
        };
        for item in &history.history {
            let read_end: usize  = item.1.msg_list.len().wrapping_sub(item.1.unread_count.try_into().unwrap());
            let read_start: usize = match read_end > before.try_into().unwrap() {
                true => {
                    read_end.wrapping_sub(before.try_into().unwrap())
                },
                false => {
                    0
                }
            };
            let unread_start: usize = item.1.msg_list.len().wrapping_sub(item.1.unread_count.try_into().unwrap());
            let unread_end: usize = std::cmp::min(unread_start.wrapping_add(after.try_into().unwrap()) , item.1.msg_list.len());

            println!("start1,end1:{},{}", read_start, read_end);
            println!("start2,end2:{},{}", unread_start, unread_end);
            rtn.list.push(MsgHistoryAllByUnreadItem {
                pubkey: item.0.clone(),
                unread: item.1.unread_count,
                msg_list_read: item.1.msg_list[read_start.try_into().unwrap()..read_end].to_vec(),
                msg_list_unread: item.1.msg_list[unread_start.try_into().unwrap()..unread_end].to_vec(),
                latest_timestamp: 0
            });
        }
        return rtn;
    }
}

#[derive(Debug, Default,Serialize, Deserialize)]
pub struct MsgHistoryAllByUnreadItem {
    pubkey: String,
    unread: u64,
    msg_list_read: Vec<String>,
    msg_list_unread: Vec<String>,
    latest_timestamp: u64
}

#[derive(Debug, Default,Deserialize, Serialize)]
pub struct GroupInfo {
    pub pubkey: String,         // 群公钥
    pub name: String,           // 群名称  
    pub access: u32,            // 进入许可 pub const GROUP_ACCESS_ANYONE: u32 = 1; GROUP_ACCESS_NEED_OWNER_ALLOW: u32 = 2; 
    pub visible_by_name: bool,  // 是否允许通过群名被查找到
    pub owner: String,          // 群主
    pub members: Vec<String>    // 群员
}

/*
{ "_id" : ObjectId("61e6417713980495c566c244"), 
"user_pubkey" : "03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c", 
"history" : { 
    "035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e" : 
    { "unread_count" : NumberLong(0), "msg_list" : [ 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX1/8Cow7gJKkv5woydO0uRhATM9JQXJe5H0=\",\"time_stamp\":1642479991333},\"sign\":\"fd24eaa1d420f4431da6d48fe582297de58fd44db9e33b519e2fa76f60e7efa364f6a1198d56482e3191bfa965e84d85878f2e889444779f13840abcc48d13ce\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX1/35MGm5S7wT+NR5tD1Dm7Fh1YYJ8JODDc=\",\"time_stamp\":1642479991708},\"sign\":\"a577e7672fbb053b7015be005ab1960c81c7170a5e386ab28635d8da65b44559165427bcd3c753514ab48d9ec091f1c311c04d9a3cba49cf05d4fadf42e09c85\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX19oc73sMUDXjdY4QI+of3QpC3uvbmz6dok=\",\"time_stamp\":1642479992011},\"sign\":\"9aaf60377c9acfd0f13a5ee840a44bed536f634e203d3562c8aeb6c953bb12b16959e60a6357f554f582866e768c03aaff79ac1aa5032e358851a9487f65d23b\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX1/l+SPy7nNwOqqf5Pl9NBIfepVNN2lFs4Y=\",\"time_stamp\":1642479992321},\"sign\":\"f0aaab4358683c4bf3555fb18796d8b78395915f3e822cea107f9e2fc3a773b91bc448ad0274d6976ad05964c6bbcc2d58a0ba411d2d89b361beba811a48351d\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX18UOX/SPvG90qC7uEJurttWb1zpyFTBU9o=\",\"time_stamp\":1642479992750},\"sign\":\"85c7a3a93c8876b29dd939cdd3eb24ed7aef3e4490fa96bba0f6d9bf266d93bb3a42f20d4b3fa5ef6b149fd0ff41eecdb9f2b2c123df84d65f0a13ecc6ed98a7\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX1/QbomNVmGL7Qp8OHXy8xRIbTzsvI4Wb0g=\",\"time_stamp\":1642479993088},\"sign\":\"15d69e79e099e34a627cc2017d299525ba3d526b1d87fabf15c98d43d6fe52bf05e2a73b51ce94fd57b492dd04ce4e30a66a9933b5843f1d8ff487ba3cab76d9\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX1+ieoMhXaDHRpkhEeSD3pqrwuTMVntG37k=\",\"time_stamp\":1642479993407},\"sign\":\"749e70c8bf5e6a121a7d0f7450a9fa20b348b7786e0c97f594caba048bc13c87310e499a48be84a2019371bb402d2c4930c92b66aa97d34bf1d331607ac21bb1\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX19VX5Wc4vRhKDBB3Plx1OwksCfDJK1kISc=\",\"time_stamp\":1642479993713},\"sign\":\"217b2cadcd9a48f0d5a218ce540c730dd4740996f798ef93e84e1fb40c80f15d4db4f17d95b9f35a81be462c50d74cf04f41fe82099810d1e6db1e0708ee0e1c\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX19zoZ0VMRckLjq74M3kf+ooYIjXCO6Pi+8=\",\"time_stamp\":1642479994135},\"sign\":\"fddf6f02b669d858c83d9c0f6ca274de73fdeccd470d23de79e1c25cb9952c0e5024bb63e5db109302e829f8159201abaeb058b7f13e661d5b6e06cf92f45711\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"to\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"msg\":\"U2FsdGVkX1/mCxH8GqfKnrdE53tONQn0/nWjgSqfbZw=\",\"time_stamp\":1642479994570},\"sign\":\"958a8d5d7db0b353026970bde1b17294281b5161e48c3390e9d133c0ab0debe60dc8c5af7c20f5eecb86ca3e2cc61712e751b174bcdbec63a353af73817257aa\"}", 
        "{\"base_msg\":{\"msg_type\":1,\"from\":\"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c\",\"to\":\"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e\",\"msg\":\"U2FsdGVkX18+pKN4VhpEf00SXOoa4UY5UFXt3SdxYxI=\",\"time_stamp\":1642505762416},\"sign\":\"afc3854484f697f69abb6ee1dab6f5b400493145565c9d1b99b99746021f1b6505c14247b90940d90341b24d7244641324a0fe137f285947b875a04d3e0410b2\"}" ] 
        } }, "unread_count" : 10 }

*/