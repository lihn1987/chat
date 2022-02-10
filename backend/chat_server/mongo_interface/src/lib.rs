use mongodb::bson::to_vec;
use mongodb::options::IndexOptions;
use msg_interface::mongo_msg::{MsgHistory, MsgHistoryAllByUnread};
use mongodb::{
    bson::doc,
    sync::Client,
    sync::Database,
    IndexModel
};
use std::borrow::BorrowMut;
use std::sync::Arc;
use std::sync::Mutex;

use msg_interface::ws_msg::*;
use msg_interface::mongo_msg::UserInfo;

use serde_json::json;
pub struct MongoInterface{
    db: Database,
    user_info_index_pubkey: bool,
    msg_history_index_user_pubkey: bool
}

impl MongoInterface {
    pub fn get_instance() -> Arc<Mutex<MongoInterface>> {
        static mut INSTANCE: Option<Arc<Mutex<MongoInterface>>> = None;
        unsafe {
            if INSTANCE.is_none() {
                let client:Client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
                Arc::clone(INSTANCE.insert(Arc::new(Mutex::new(
                    MongoInterface {
                        db: client.database("chat"),
                        user_info_index_pubkey: false,
                        msg_history_index_user_pubkey: false
                    }
                ))))
            } else {
                match INSTANCE.clone() {
                    Some(v) => { Arc::clone(&v)}
                    _ =>{ panic!()}
                }
            }
        }
    }

    pub fn init (&mut self){
        if !self.user_info_index_pubkey {
            let collection = self.db.collection::<UserInfo>("user_info");
            let index = IndexModel::builder()
                .keys(doc! { "pubkey": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build();
            match collection.create_index( index,  None) {
                Ok(_) => {
                    println!("成功创建用户pubkey索引");
                },
                Err(_) => {
                    println!("创建用户索引pubkey失败");
                }
            };

            for iter in collection.list_index_names().unwrap() {
               if &iter == "pubkey_1" {
                   self.user_info_index_pubkey = true;
               }
            }
        }

        if !self.msg_history_index_user_pubkey {
            let collection = self.db.collection::<MsgHistory>("msg_history");
            let index = IndexModel::builder()
                .keys(doc! { "user_pubkey": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build();
            match collection.create_index( index,  None) {
                Ok(_) => {
                    println!("成功创建聊天记录pubkey索引");
                },
                Err(_) => {
                    println!("创建用户聊天记录pubkey失败");
                }
            };

            for iter in collection.list_index_names().unwrap() {
                if &iter == "user_pubkey_1" {
                    self.msg_history_index_user_pubkey = true;
                }
             }
        }
        
    }

    pub fn insert_user_info(&mut self, doc: UserInfo) -> bool {
        self.init();
        let collection = self.db.collection::<UserInfo>("user_info");
        match collection.insert_one(doc, None) {
            Ok(_v) => { true},
            _ => {false}
        }
        //collection.create_index(index, options)
    }

    pub fn find_user_by_pubkey(&self, pubkey: &str) -> Option<UserInfo> {
        let collection = self.db.collection::<UserInfo>("user_info");
        let cursor = collection.find(doc!{ "pubkey":pubkey }, None).unwrap();
        let mut rtn:Option<UserInfo> = None;
        for result in cursor {
            rtn = match result {
                Ok(v) => Some(v),
                Err(_) => None
            }
        }
        return rtn;
    }

    pub fn add_friend(&mut self, user_pubkey: &str, friend_pubkey: &str) -> bool{
        self.init();
        // 不可添加自己好友
        if user_pubkey == friend_pubkey {
            println!("插入好友失败!不可添加自己为好友");
            return false
        }

        // 查询用户数据
        let collection = self.db.collection::<UserInfo>("user_info");
        let cursor = collection.find(doc!{ "pubkey":user_pubkey }, None).unwrap();
        let mut user_info_option:Option<UserInfo> = None;
        for result in cursor {
            user_info_option = match result {
                Ok(v) => Some(v),
                Err(_) => None
            }
        }
        // 用户不存在
        if user_info_option.is_none() {
            println!("插入好友失败!用户不存在");
            return false;
        }
        
        // 查询好友信息
        let cursor = collection.find(doc!{ "pubkey":friend_pubkey }, None).unwrap();
        let mut friend_info_option:Option<UserInfo> = None;
        for result in cursor {
            friend_info_option = match result {
                Ok(v) => Some(v),
                Err(_) => None
            }
        }
        if friend_info_option.is_none() {
            // 好友不存在
            println!("插入好友失败!好友不存在");
            return false;
        }

        let mut user_info = user_info_option.unwrap();
        // 用户在联系人中，返回失败
        for item in &user_info.friends {
            if item == friend_pubkey {
                println!("插入好友失败!用户在联系人中");
                return false;
            }
        }
        user_info.friends.push(friend_pubkey.to_string());
        
        match collection.update_one(doc!{ "pubkey":user_pubkey }, doc!{ "$set":{"friends":user_info.friends }}, None) {
            Ok(_) => true,
            Err(_) => {
                println!("插入好友失败!更新数据库失败");
                false
            }
        }
    }

    pub fn change_nick_name(&mut self, pubkey: &str, nick_name: &str) -> bool {
        self.init();
        println!("修改用户名称{},{}", pubkey, nick_name);
        let collection = self.db.collection::<UserInfo>("user_info");
        match collection.update_one(doc!{"pubkey": pubkey}, doc!{"$set":{"nick":nick_name}}, None)  {
            Ok(_v) => { true},
            _ => {false}
        }
    }

    // 插入聊天历史
    pub fn insert_chat_msg(&mut self, msg: &MsgSigned) -> bool {
        self.init();
        let collection = self.db.collection::<MsgHistory>("msg_history");

        // 将创建用户的聊天数据
        let _ = collection.insert_one(MsgHistory::new(msg.base_msg.from.clone()), None);
        let _ = collection.insert_one(MsgHistory::new(msg.base_msg.to.clone()), None);
        // 插入发送者
        let mut result = collection.update_one(
            doc!{
                "user_pubkey":msg.base_msg.from.clone(),
                "history.".to_string()+&msg.base_msg.to: {
                    "$exists":false
                }
            }, 
            doc!{
                "$set": {
                    "history.".to_string()+&msg.base_msg.to :{
                        "unread_count": 0,
                        "msg_list": []
                    }
                }
            }, 
            None);
        println!("插入发送者->接收者历史记录,{:?}", result);
        result = collection.update_one(
            doc!{
                "user_pubkey":msg.base_msg.to.clone(),
                "history.".to_string()+&msg.base_msg.from: {
                    "$exists":false
                }
            }, 
            doc!{
                "$set": {
                    "history.".to_string()+&msg.base_msg.from :{
                        "unread_count": 0,
                        "msg_list": []
                    }
                }
            }, 
            None);
            println!("接收者->插入发送者历史记录,{:?}", result);
        // 插入发送者
        let x = collection.update_one(
            doc!{"user_pubkey":msg.base_msg.from.clone()}, 
            doc!{
                "$push": {
                    "history.".to_string()+&msg.base_msg.to+".msg_list" :serde_json::to_string(&msg).unwrap()
                }
            }, 
            None);
        println!("插入聊天历史{:?}", x);
        // 插入接收者
        let _ = collection.update_one(
            doc!{"user_pubkey":msg.base_msg.to.clone()}, 
            doc!{
                "$push": {"history.".to_string()+&msg.base_msg.from+".msg_list" :serde_json::to_string(&msg).unwrap()},
                "$inc":{"history.".to_string()+&msg.base_msg.from+".unread_count": 1}
            }, 
            None);
        
        return true;
        //collection.create_index(index, options)
    }

    // 获取聊天记录
    pub fn get_chat_msg_by_unread(&mut self, owner_pubkey: &str, peer_pubkey: &str, befor_count: u64, after_count: u64) {
        self.init();
        let collection = self.db.collection::<MsgHistory>("msg_history");
        /*
        db.msg_history.find({
            "user_pubkey":"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e", 
            "history.035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e":{"$exists":true}
        },{
            "history.035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e":{"$slice":[100,110]}
        })
        **/
        //db.msg_history.find({"user_pubkey":"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e", "history.03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c":{"$exists":true}},{"history.03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c":{"$slice":[100,110]}})
        // db.msg_history.find({"user_pubkey" : "035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e"},{"history.03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c":{"$slice":[100,110]}})
    }
    // 获取所有聊天记录
    pub fn get_all_chat_msg_by_unread(&mut self, owner_pubkey: &str,  befor_count: u64, after_count: u64) -> MsgHistoryAllByUnread {
        self.init();
        let collection = self.db.collection::<MsgHistory>("msg_history");
        let info = collection.find_one(doc!{
            "user_pubkey":owner_pubkey, 
        }, None).unwrap();
        if info.is_none() {
            println!("is_none");
            return Default::default()
        }

        let info = info.unwrap();
        // info.
        let msg_return = MsgHistoryAllByUnread::from_history(&info, befor_count, after_count);
        return msg_return;
        /*
        db.msg_history.find({
            "user_pubkey":"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e", 
            "history.035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e":{"$exists":true}
        },{
            "history.035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e":{"$slice":[100,110]}
        })
        **/
        //db.msg_history.find({"user_pubkey":"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e", "history.03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c":{"$exists":true}},{"history.03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c":{"$slice":[100,110]}})
        // db.msg_history.find({"user_pubkey" : "035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e"},{"history.03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c":{"$slice":[100,110]}})

        //db.msg_history.find({"user_pubkey":"035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e"},{"history:{unread_count}":1})
    }

    // 获取所有聊天记录的最后几条
    pub fn get_all_chat_msg_by_latest(&mut self, owner_pubkey: &str) -> MsgHistory {
        self.init();
        println!("get_all_chat_msg_by_latest:{}", owner_pubkey);
        let max_len: usize = 100;
        let collection = self.db.collection::<MsgHistory>("msg_history");
        let info = collection.find_one(doc!{
            "user_pubkey":owner_pubkey, 
        }, None).unwrap();
        if info.is_none() {
            println!("is_none");
            return Default::default()
        }
        let mut info = info.unwrap();
        for item in info.history.borrow_mut() {
            let start:usize = match item.1.msg_list.len() > max_len {
                true => {
                    max_len.wrapping_sub(item.1.msg_list.len())
                },
                false => {
                    0
                }
            };
            item.1.msg_list = item.1.msg_list[start..item.1.msg_list.len()].to_vec();
        }
        return info;
    }

    // 清零unread
    pub fn clear_unread(&mut self, owner_pubkey: &str,  from: &str) {
        self.init();
        println!("clear_unread{},{}", owner_pubkey, from);
        if from == "" {
            return
        }
        let tmp = json!({
            "user_pubkey": owner_pubkey, 
            "history.".to_string() + from:{"$exists":true}
        });
        println!("{}", tmp.to_string());

        let collection = self.db.collection::<MsgHistory>("msg_history");
        println!("clear_unread: {} {}", owner_pubkey, from);
        let _ = collection.update_one(doc!{
            "user_pubkey": owner_pubkey, 
            "history.".to_string() + from:{"$exists":true}
        }, doc!{
            "$set": {
                "history.".to_string() +from+".unread_count": 0
            }
        }, None);
    }
}
