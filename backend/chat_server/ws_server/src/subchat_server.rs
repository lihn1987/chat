

use crate::subchat_ws_server::SubChatWsServer;
use crate::subchat_rest_server::SubChatRestServer;
use std::thread;
/// 所有的后台服务，在此处配置和使用
pub struct SubchatServer {
    // rest_server: SubChatRestServer,
    ws_server: SubChatWsServer
}

impl SubchatServer {
    // pub fn new (rest_addr: SocketAddr, ws_addr: SocketAddr) -> subchat_server {
    //     subchat_server {
    //         rest_addr: rest_addr,
    //         ws_addr: ws_addr
    //     } 
    // }
    pub fn new_default() -> SubchatServer {
        SubchatServer {
            // rest_server: SubChatRestServer::new(([0,0,0,0], 9901)),
            ws_server: SubChatWsServer::new(([0,0,0,0], 9902))
        } 
    }
    pub fn start(&self) {
        //self.rest_server.start();
        self.ws_server.start();
    }
}
