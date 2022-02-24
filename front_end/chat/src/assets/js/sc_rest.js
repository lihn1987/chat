import axios from "axios";
const secp256k1 = require('secp256k1')
const crypto = require('crypto')
const MSG_LOGIN = 0;
const MSG_TXT = 1;

// struct RequestBase {
//     msg_type: u32,
//     from: String,
//     timestamp: u64,
//     param: String
// }

// RequestComm {
//     msg: RequestBase,
//     sign: String
// }
class SubChainRest { 
    constructor(pri_key) {
        this.pri_key = pri_key
    }
    Request(param, url) {
        var prikey_buf = Buffer(this.pri_key, "hex")
        var pubkey_buf = Buffer.from(secp256k1.publicKeyCreate(prikey_buf))

        var msg_base = {
            from: pubkey_buf.toString("hex"),
            timestamp: Date.now(),
            param: param
        }
        console.log("msg_base", msg_base)

        var hash = crypto.createHash('sha256');
        hash.update(JSON.stringify(msg_base))
        var hash_buffer = hash.digest()
        console.log("hash", hash_buffer.toString("hex"))
        var msg = {
            msg: msg_base,
            sign: Buffer(secp256k1.ecdsaSign(hash_buffer, prikey_buf).signature).toString("hex")
        }
        // console.log("发送消息", JSON.stringify(msg))
        return axios.post("/subchain/"+url, msg, {
            'Content-Type':'application/json'
        }) 
    }
    GetSelfInfo() {
        return this.Request("", "get_self_info")
    }
    GetUserInfoInfo(pubkey) {
        return this.Request(pubkey, "get_user_info")
    }
    AddFriendByPubkey(pubkey) {
        return this.Request(JSON.stringify({
            pubkey: pubkey
        }), "add_friend_by_pubkey")
    }
    ChangeNickName(nick_name) {
        return this.Request(JSON.stringify({
            nick_name: nick_name
        }), "change_nick_name")
    }
    GetAllChatMsgByLatestInfo(){
        return this.Request("", "get_all_chat_msg_by_latestinfo");
    }
    ClearUnreadInfo(pubkey){
        return this.Request(pubkey, "clear_unreadinfo");
    }
    CreateGroup( group_name, access, visible){
        return this.Request(JSON.stringify({
            name: group_name,           // 群名称  
            access: access,            // 进入许可
            visible_by_name: visible
        }), "create_group")
    }
}

export {
    SubChainRest
}
