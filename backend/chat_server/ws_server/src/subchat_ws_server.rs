
use std::thread::{self};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use websocket::sync::Server;
use websocket::OwnedMessage;
use websocket::sync::client::Writer;

extern crate hex;

use std::net::{SocketAddr, TcpStream};
use msg_interface::ws_msg::*;
use mongo_interface::{MongoInterface};
use msg_interface::mongo_msg::UserInfo;
//use crate::subchat_ws_server



pub struct SubChatWsServer{
    links: Arc<Mutex<HashMap<String, String>>>,
    senders_manager: Arc<Mutex<HashMap<String, Arc<Mutex<Writer<TcpStream>>>>>>,
    addr: SocketAddr
}
impl SubChatWsServer {
    pub fn new ( addr: impl Into<SocketAddr> ) -> SubChatWsServer {
        SubChatWsServer{
            links: Arc::new(Mutex::new(HashMap::new())),
            senders_manager: Arc::new(Mutex::new(HashMap::new())),
            addr: addr.into()
        }
    }
    pub fn start ( &self ) {
        println!("websocket start listen {:?}", self.addr);
        
        let server = Server::bind(self.addr).unwrap();
        for request in server.filter_map(Result::ok) {
            // Spawn a new thread for each connection.
            let links_mutex = Arc::clone(&self.links);
            let senders_mutex = Arc::clone(&self.senders_manager);
            //let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                // if !request.protocols().contains(&"rust-websocket".to_string()) {
                //     request.reject().unwrap();
                //     return;
                // }
                let client = request.accept().unwrap();
                // let mut client = request.use_protocol("rust-websocket").accept().unwrap();

                let ip = client.peer_addr().unwrap();
                let mut pubkey = String::from("");
                println!("Connection from {}", ip);
                let (mut receiver, sender) = client.split().unwrap();
                let shared_sender = Arc::new(Mutex::new(sender));
                // 线程循环
                loop {
                    // 处理所有网络信息
                    println!("start_read");
                    let message = receiver.recv_message();
                    println!("end_read");
                    if message.is_err() {
                        break
                    }
                    let message = message.unwrap();

                    match message {
                        OwnedMessage::Close(_) => {
                            let message = OwnedMessage::Close(None);
                            shared_sender.lock().unwrap().send_message(&message).unwrap();
                            println!("Client {} disconnected", ip);
                            if &pubkey != "" {
                                //如果已经登录，则移除
                                let mut links = links_mutex.lock().unwrap();
                                links.remove(&pubkey);
                            }
                        }
                        OwnedMessage::Ping(ping) => {
                            let message = OwnedMessage::Pong(ping);
                            shared_sender.lock().unwrap().send_message(&message).unwrap();
                        }
                        OwnedMessage::Text(text) => {
                            // println!("收到消息:{}", &text);
                            println!("收到消息");
                            // 消息解析
                            let msg:MsgSigned = serde_json::from_str(&text).expect("接收到无法识别的信息");
                            // 消息验证
                            if msg.check_sign() {
                                println!("验证成功,消息是本人发送");
                            } else {
                                panic!("验证失败,消息不是本人发送");
                            }
                            match msg.base_msg.msg_type {
                                MSG_LOGIN => {
                                    let mut links = links_mutex.lock().unwrap();
                                    let mut senders = senders_mutex.lock().unwrap();
                                    match links.get(&msg.base_msg.from) {
                                        Some(_v) => {
                                            // TODO已经链接
                                        },
                                        None => {
                                            //pubkey = String::new(&(msg.base_msg.from.to_string());
                                            pubkey = msg.base_msg.from.to_string();
                                            
                                            links.insert(pubkey.clone(), ip.to_string());
                                            // sender_option = 
                                            senders.insert(pubkey.clone(), shared_sender.clone());

                                            println!("登录成功！{},{}", &msg.base_msg.from, ip.to_string());
                                            for (k, v) in links.iter() {
                                                println!("链接:{},{}", k, v);
                                            }
                                            let response = serde_json::to_string(&Response {
                                                msg_type: MSG_LOGIN,
                                                result: true,
                                                data: "".to_string()
                                            }).unwrap();
                                            {// 插入数据库
                                                let mutex_db = MongoInterface::get_instance();
                                                println!("获取锁");
                                                let mut db = mutex_db.lock().unwrap();
                                                db.insert_user_info(UserInfo{
                                                    pubkey: pubkey.clone(),
                                                    nick: pubkey.clone(),
                                                    sex: 0, // 0 unknow, 1 man, 2 woman 
                                                    age: 0,
                                                    friends: vec![]
                                                });
                                                println!("释放锁");
                                            }
                                            println!("1111111111113");
                                            // 回复

                                            println!("1111111111114");
                                            match senders.get_mut(&pubkey) {
                                                Some(v) => {
                                                    let mut sender = v.lock().unwrap();
                                                    sender.send_message(&OwnedMessage::Text(response)).expect("消息发送失败");
                                                },
                                                _=>{}
                                            }
                                            
                                            
                                        }
                                    }
                                },
                                MSG_TXT | MSG_AUDIO => {
                                    if pubkey != msg.base_msg.from {
                                        panic!("收到非本账户信息，危险异常");
                                    }
                                    {
                                        let mut senders = senders_mutex.lock().unwrap();
                                        match senders.get_mut(&msg.base_msg.to) {
                                            Some(v) => {
                                                let mut sender = v.lock().unwrap();
                                                match sender.send_message(&OwnedMessage::Text(serde_json::to_string(&msg).unwrap())) {
                                                    Ok(_) => {
                                                        println!("消息转发成功")
                                                    }
                                                    Err(_)=>{
                                                        println!("消息发送失败");
                                                    }
                                                }
                                                
                                            },
                                            _=>{
                                                println!("无法转发文本信息，对方不在线")
                                            }
                                        }
                                        // 发送后计入数据库
                                        let mutex_db = MongoInterface::get_instance();
                                        println!("获取锁");
                                        let mut db = mutex_db.lock().unwrap();
                                        db.insert_chat_msg(&msg);
                                    }
                                    
                                },   
                                MSG_CLEAR_UNREAD => {
                                    if pubkey != msg.base_msg.from {
                                        panic!("收到非本账户信息，危险异常");
                                    }
                                    {
                                        // 发送后计入数据库
                                        let mutex_db = MongoInterface::get_instance();
                                        println!("获取锁");
                                        let mut db = mutex_db.lock().unwrap();
                                        db.clear_unread(&msg.base_msg.from, &msg.base_msg.msg);
                                        println!("清空未读信息：{},{}", &msg.base_msg.from, &msg.base_msg.msg);
                                    }
                                }    
                                _ =>{
                                    println!("收到未知消息")
                                }
                            }

                            
                        },
                        _ => {   
                            // sender.send_message(&message).unwrap();
                        }
                    }
                }
            });
        }
    }
}

// fn main() {
// 	let server = Server::bind("127.0.0.1:2794").unwrap();

// 	for request in server.filter_map(Result::ok) {
// 		// Spawn a new thread for each connection.
// 		thread::spawn(|| {
// 			if !request.protocols().contains(&"rust-websocket".to_string()) {
// 				request.reject().unwrap();
// 				return;
// 			}
//       request.accept()
// 			let mut client = request.use_protocol("rust-websocket").accept().unwrap();

// 			let ip = client.peer_addr().unwrap();

// 			println!("Connection from {}", ip);

// 			let message = OwnedMessage::Text("Hello".to_string());
// 			client.send_message(&message).unwrap();

// 			let (mut receiver, mut sender) = client.split().unwrap();

// 			for message in receiver.incoming_messages() {
// 				let message = message.unwrap();

// 				match message {
// 					OwnedMessage::Close(_) => {
// 						let message = OwnedMessage::Close(None);
// 						sender.send_message(&message).unwrap();
// 						println!("Client {} disconnected", ip);
// 						return;
// 					}
// 					OwnedMessage::Ping(ping) => {
// 						let message = OwnedMessage::Pong(ping);
// 						sender.send_message(&message).unwrap();
// 					}
// 					_ => sender.send_message(&message).unwrap(),
// 				}
// 			}
// 		});
// 	}
// }