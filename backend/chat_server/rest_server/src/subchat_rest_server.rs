

use std::net::{SocketAddr};
use warp::Filter;
extern crate hex;
use mongo_interface::{MongoInterface};
use msg_interface::rest_msg::*;
use msg_interface::mongo_msg::*;

pub struct SubChatRestServer{
    addr: SocketAddr
}

impl SubChatRestServer {
    pub fn new(addr: impl Into<SocketAddr>) -> SubChatRestServer {
        return SubChatRestServer{
            addr: addr.into()
        };
    }
    // 构建整个rest server
    pub async fn start (&self) {
        let get_self_info = warp::post()
            .and(warp::path!("subchain"/"get_self_info"))
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|info| SubChatRestServer::get_self_info(info));

        let get_user_info = warp::post()
            .and(warp::path!("subchain"/"get_user_info"))
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|info| SubChatRestServer::get_user_info(info));
            
        let add_friend_by_pubkey = warp::post()
            .and(warp::path!("subchain"/"add_friend_by_pubkey"))
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|info| SubChatRestServer::add_friend_by_pubkey(info));
        
        let change_nick_name = warp::post()
            .and(warp::path!("subchain"/"change_nick_name"))
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|info| SubChatRestServer::change_nick_name(info));

        let get_all_chat_msg_by_latestinfo = warp::post()
            .and(warp::path!("subchain"/"get_all_chat_msg_by_latestinfo"))
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|info| SubChatRestServer::get_all_chat_msg_by_latestinfo(info));

        let clear_unreadinfo = warp::post()
            .and(warp::path!("subchain"/"clear_unreadinfo"))
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|info| SubChatRestServer::clear_unreadinfo(info));

        warp::serve(
            get_self_info
            .or(get_user_info)
            .or(add_friend_by_pubkey)
            .or(change_nick_name)
            .or(get_all_chat_msg_by_latestinfo)
            .or(clear_unreadinfo)
        )
        .run(self.addr).await
    }

    // 获取个人的所有信息
    fn get_self_info(info: RequestComm) -> warp::reply::Json{
        if !info.check_sign() {
            return warp::reply::json(&ResponseComm{
                error: ERROR_NO_POWER,
                data: String::from("")
            });
        }
        let mutex_db = MongoInterface::get_instance();
        let db = mutex_db.lock().unwrap();
        match db.find_user_by_pubkey(&info.msg.from) {
            Some(v) => {
                warp::reply::json(&ResponseComm{
                    error: 0,
                    data: serde_json::to_string(&v).unwrap()
                })
            },
            None => {
                warp::reply::json(&ResponseComm{
                    error: ERROR_NO_PUBKEY_FINDED,
                    data: String::from("")
                })
            }
        }
    }

    // 获取某个用户的可公开信息
    fn get_user_info(info: RequestComm) -> warp::reply::Json{
        if !info.check_sign() {
            return warp::reply::json(&ResponseComm{
                error: ERROR_NO_POWER,
                data: String::from("")
            });
        }
        let mutex_db = MongoInterface::get_instance();
        let db = mutex_db.lock().unwrap();
        match db.find_user_by_pubkey(&info.msg.param) {
            Some(v) => {
                warp::reply::json(&ResponseComm{
                    error: 0,
                    data: serde_json::to_string(&UserInfoCouldPublic::from(v)).unwrap()
                })
            },
            None => {
                warp::reply::json(&ResponseComm{
                    error: ERROR_NO_PUBKEY_FINDED,
                    data: String::from("")
                })
            }
        }
    }

    // 添加一个好友，并返回该好友的公开信息
    fn add_friend_by_pubkey(info: RequestComm) -> warp::reply::Json{
        if !info.check_sign() {
            println!("添加好友失败，没有权限");
            return warp::reply::json(&ResponseComm{
                error: ERROR_NO_POWER,
                data: String::from("")
            });
        }
        let request:AddFriendByPubkeyRequest = serde_json::from_str(&info.msg.param).unwrap();
        let mutex_db = MongoInterface::get_instance();
        let mut db = mutex_db.lock().unwrap();
        match db.find_user_by_pubkey(&request.pubkey) {
            Some(v) => {
                if !db.add_friend(&info.msg.from, &request.pubkey) {
                    return warp::reply::json(&ResponseComm{
                        error: ERROR_ADD_FRIEND,
                        data: String::from("")
                    });
                }
                warp::reply::json(&ResponseComm{
                    error: 0,
                    data: serde_json::to_string(&AddFriendByPubkeyResponse::from_user_info(&v)).unwrap()
                })
            },
            None => {
                warp::reply::json(&ResponseComm{
                    error: ERROR_NO_PUBKEY_FINDED,
                    data: String::from("")
                })
            }
        }
    }
    
    // 修改用户昵称
    fn change_nick_name(info: RequestComm) -> warp::reply::Json{
        if !info.check_sign() {
            println!("修改昵称失败，没有权限");
            return warp::reply::json(&ResponseComm{
                error: ERROR_NO_POWER,
                data: String::from("")
            });
        }
        let request:ChangeNickNameRequest = serde_json::from_str(&info.msg.param).unwrap();
        let mutex_db = MongoInterface::get_instance();
        let mut db = mutex_db.lock().unwrap();
        if db.change_nick_name(&info.msg.from, &request.nick_name) {
            warp::reply::json(&ResponseComm{
                error: 0,
                data: String::new()
            })
        } else {
            warp::reply::json(&ResponseComm{
                error: ERROR_CHANGE_NICK_NAME,
                data: String::from("")
            })
        }
    }

    // 获取所有最新聊天记录
    fn get_all_chat_msg_by_latestinfo(info: RequestComm) -> warp::reply::Json {
        if !info.check_sign() {
            println!("获取聊天记录失败，没有权限");
            return warp::reply::json(&ResponseComm{
                error: ERROR_NO_POWER,
                data: String::from("")
            });
        }
        let mutex_db = MongoInterface::get_instance();
        let mut db = mutex_db.lock().unwrap();
        let history = db.get_all_chat_msg_by_latest(&info.msg.from);
        warp::reply::json(&ResponseComm{
            error: 0,
            data: serde_json::to_string(&history).unwrap()
        })
        
    }
    // 清空unread
    fn clear_unreadinfo(info: RequestComm) -> warp::reply::Json {
        if !info.check_sign() {
            println!("获取聊天记录失败，没有权限");
            return warp::reply::json(&ResponseComm{
                error: ERROR_NO_POWER,
                data: String::from("")
            });
        }
        let mutex_db = MongoInterface::get_instance();
        let mut db = mutex_db.lock().unwrap();
        db.clear_unread(&info.msg.from, &info.msg.param);
        return warp::reply::json(&ResponseComm{
            error: 0,
            data: String::new()
        });
    }
}
