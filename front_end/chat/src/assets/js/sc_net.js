
const secp256k1 = require('secp256k1')
const crypto = require('crypto')
import { sc_crypto } from './sc_crypto'

const MSG_LOGIN = 0;
const MSG_TXT = 1;
const MSG_AUDIO = 2;
const MSG_AUDIO_TYPE_1 = 1;//json，无压缩
const MSG_IMAGE = 3;
const MSG_CLEAR_UNREAD = 101;
// struct MsgBase{
//     msg_type: u32,
//     from: String,
//     to: String,
//     msg: String,
//     time_stamp: u64
// }

// struct MsgSigned {
//     base_msg: MsgBase,
//     sign: String
// }

class SubChainSocket {
    constructor() {
        this.ws = null
        this.pri_key = null
        this.login_cb = null
        this.msg_cb
    }
    Open(url, pri_key, login_cb, msg_cb) {
        if ( this.ws != null) {
            this.Close()
        } 
        this.login_cb = login_cb
        this.ws = new WebSocket(url);
        this.pri_key = pri_key
        this.msg_cb = msg_cb

        this.ws.onopen = this.OnOpen
        this.ws.onmessage = this.OnMessage
        this.ws.onclose = this.OnClose
        this.ws.onerror = this.OnError
    }
    Close() {
        this.ws.close()
        this.ws.onopen = ()=>{}
        this.ws.onmessage = ()=>{}
        this.ws.onclose = ()=>{}
        this.ws.onerror = ()=>{}
        this.ws = null
    }
    OnOpen () {
        console.log("OnOpen")
        sc_socket.Login()
    }
    OnClose (e) {
    　　console.log("OnClose", e);
    }
    OnError (e){
    　　console.log("OnError", e);
    }
    OnMessage (e) {
        console.log("OnMessage", e.data)
        var msg_obj = JSON.parse(e.data);
        switch (msg_obj.msg_type) {
            case MSG_LOGIN: {
                sc_socket.login_cb(msg_obj);
                break;
            }
            default: {
                if(!sc_socket.CheckSign(msg_obj)){
                    console.log("校验签名失败")
                    break;
                }
                console.log("校验签名成功")
                msg_obj.base_msg.content = sc_crypto.EcdhDecode(msg_obj.base_msg.msg, msg_obj.base_msg.time_stamp, sc_socket.pri_key, msg_obj.base_msg.from)
                sc_socket.msg_cb(msg_obj);
                break;
            }
        }

    }

    MsgBase2Msg(msg_base) {
        var pri_key = this.pri_key;
        var prikey_buf = Buffer(pri_key, "hex")
        var hash = crypto.createHash('sha256');
        hash.update(JSON.stringify(msg_base))
        var hash_buffer = hash.digest()
        console.log("hash", hash_buffer.toString("hex"))
        var pubkey_buf = Buffer.from(secp256k1.publicKeyCreate(prikey_buf))
        var msg = {
            base_msg: msg_base,
            sign: Buffer(secp256k1.ecdsaSign(hash_buffer, prikey_buf).signature).toString("hex")
        }
        return msg
    }
    CheckSign(msg_obj) {
        var hash = crypto.createHash('sha256');
        hash.update(JSON.stringify(msg_obj.base_msg))
        var hash_buffer = hash.digest()
        return secp256k1.ecdsaVerify(Buffer(msg_obj.sign, "hex"), hash_buffer,  Buffer(msg_obj.base_msg.from, "hex"))
    }
    Login() {
        var prikey_buf = Buffer(this.pri_key, "hex")
        var pubkey_buf = Buffer.from(secp256k1.publicKeyCreate(prikey_buf))

        var msg_base = {
            msg_type: MSG_LOGIN,
            from: pubkey_buf.toString("hex"),
            to: "",
            msg: "",
            time_stamp: Date.now()
        }

        var msg = this.MsgBase2Msg(msg_base)
        // console.log("发送消息", JSON.stringify(msg))
        sc_socket.ws.send(JSON.stringify(msg))
    }
    
    SendText(to, txt) {
        var prikey_buf = Buffer(this.pri_key, "hex")
        var pubkey_buf = Buffer.from(secp256k1.publicKeyCreate(prikey_buf))
        var time_now = Date.now()
        var msg_base = {
            msg_type: MSG_TXT,
            from: pubkey_buf.toString("hex"),
            to: to,
            msg: "",
            time_stamp: time_now
        }
        // 对消息的msg进行加密处理
        msg_base.msg = sc_crypto.EcdhEncode(txt, msg_base.time_stamp, sc_socket.pri_key, to)
        console.log("msg_base", msg_base)
        console.log(Buffer.from(JSON.stringify(msg_base), "utf8"))

        var msg = this.MsgBase2Msg(msg_base)
        // console.log("发送消息", JSON.stringify(msg))
        sc_socket.ws.send(JSON.stringify(msg))
        return time_now
    }
    SendAudio(to, audio, audio_type) {
        var prikey_buf = Buffer(this.pri_key, "hex")
        var pubkey_buf = Buffer.from(secp256k1.publicKeyCreate(prikey_buf))
        var time_now = Date.now()
        var msg_base = {
            msg_type: MSG_AUDIO,
            from: pubkey_buf.toString("hex"),
            to: to,
            msg: "",
            time_stamp: time_now
        }
        // 对语音进行编码
        var audio_msg = JSON.stringify({
            audio_type: audio_type,
            audio_msg: JSON.stringify(audio)
        })
        // 对消息的msg进行加密处理
        msg_base.msg = sc_crypto.EcdhEncode(audio_msg, msg_base.time_stamp, sc_socket.pri_key, to)
        console.log("msg_base", msg_base)
        console.log(Buffer.from(JSON.stringify(msg_base), "utf8"))

        var msg = this.MsgBase2Msg(msg_base)
        // console.log("发送消息", JSON.stringify(msg))
        sc_socket.ws.send(JSON.stringify(msg))
        return time_now
    }
    SendImage(to, image_src) {
        var prikey_buf = Buffer(this.pri_key, "hex")
        var pubkey_buf = Buffer.from(secp256k1.publicKeyCreate(prikey_buf))
        var time_now = Date.now()
        var msg_base = {
            msg_type: MSG_IMAGE,
            from: pubkey_buf.toString("hex"),
            to: to,
            msg: image_src,
            time_stamp: time_now
        }
        // 对消息的msg进行加密处理
        msg_base.msg = sc_crypto.EcdhEncode(image_src, msg_base.time_stamp, sc_socket.pri_key, to)
        console.log("msg_base", msg_base)
        console.log(Buffer.from(JSON.stringify(msg_base), "utf8"))

        var msg = this.MsgBase2Msg(msg_base)
        // console.log("发送消息", JSON.stringify(msg))
        sc_socket.ws.send(JSON.stringify(msg))
        return time_now
    }
    ClearUnread(to) {
        var prikey_buf = Buffer(this.pri_key, "hex")
        var pubkey_buf = Buffer.from(secp256k1.publicKeyCreate(prikey_buf))
        var time_now = Date.now()
        var msg_base = {
            msg_type: MSG_CLEAR_UNREAD,
            from: pubkey_buf.toString("hex"),
            to: "",
            msg: to,
            time_stamp: time_now
        }
        var msg = this.MsgBase2Msg(msg_base)
        // console.log("发送消息", JSON.stringify(msg))
        sc_socket.ws.send(JSON.stringify(msg))
    }
    
}

var sc_socket = new SubChainSocket()
export {
    sc_socket,
    MSG_LOGIN,
    MSG_TXT,
    MSG_AUDIO,
    MSG_AUDIO_TYPE_1,
    MSG_IMAGE
}