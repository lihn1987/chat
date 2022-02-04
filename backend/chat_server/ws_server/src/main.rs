
//mod subchat_ws_msg;
mod subchat_ws_server;
//mod SubChatWs;
use crate::subchat_ws_server::SubChatWsServer;
use mongo_interface::{MongoInterface};
//#[tokio::main]
fn main() {
  {
    println!("获取锁");
    let mutex_db = MongoInterface::get_instance();
    let mut db = mutex_db.lock().unwrap();
    db.init();
    let x = db.get_all_chat_msg_by_unread("03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c", 2, 2);
    println!(" db:{:?}", x);
    // db.insert_chat_msg(&Default::default());
    println!("释放锁");
  }
  let ws_server = SubChatWsServer::new(([0,0,0,0], 9902));
  ws_server.start();
}