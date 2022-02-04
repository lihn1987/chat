
mod subchat_rest_server;
use crate::subchat_rest_server::SubChatRestServer;
use mongo_interface::{MongoInterface};
#[tokio::main]
async fn main() {
  {
    println!("获取锁");
    let mutex_db = MongoInterface::get_instance();
    let mut db = mutex_db.lock().unwrap();
    db.init();
    // let test = db.add_friend("035b87009c9462de2dd1c260b5e3e1f53f7938d7ed470d2961a4436d2f6626961e", "03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c");
    // println!("插入好友:{}", test);
    println!("释放锁");
  }

  println!("开启restfull服务");
  let rest_server = SubChatRestServer::new(([0,0,0,0], 9901));
  println!("等待restfull服务");
  rest_server.start().await;
  println!("结束restfull服务");

}

