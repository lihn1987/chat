import CryptoJS from 'crypto-js';
const secp256k1 = require('secp256k1')
class SubChatCrypto {
    constructor () {}
    AesEncode (msg, iv, pswd) {
        // console.log("encode", msg, iv, pswd)
        var out = CryptoJS.AES.encrypt(msg, pswd, {mode:CryptoJS.mode.ECB, padding:CryptoJS.pad.Pkcs7, iv: iv});
        var str_out = out.toString();
        return str_out 
    }
    AesDecode (msg, iv, pswd){
        // console.log("decode", msg, iv, pswd)
        var out = CryptoJS.AES.decrypt(msg, pswd, {mode:CryptoJS.mode.ECB, padding:CryptoJS.pad.Pkcs7, iv: iv});
        var str_out = CryptoJS.enc.Utf8.stringify(out).toString();
        return str_out 
    }
    EcdhEncode (msg, iv, prikey_str, pubkey_str) {
        // console.log("开始加密信息", msg)
        // console.log(prikey_str, pubkey_str)
        var prikey = Buffer.from(prikey_str, "hex");
        var pubkey = Buffer.from(pubkey_str, "hex");
        var pswd = Buffer.from(secp256k1.ecdh(pubkey, prikey)).toString("hex")
        var rtn = sc_crypto.AesEncode(msg, iv, pswd)
        return rtn
    }
    EcdhDecode (msg, iv, prikey_str, pubkey_str) {
        // console.log("开始解密信息", msg)
        // console.log(prikey_str, pubkey_str)
        var prikey = Buffer.from(prikey_str, "hex");
        var pubkey = Buffer.from(pubkey_str, "hex");
        var pswd = Buffer.from(secp256k1.ecdh(pubkey, prikey)).toString("hex")
        var rtn = sc_crypto.AesDecode(msg, iv, pswd)
        return rtn
    }
    Hash(input){
        var hash = 0, i, chr;
        if (input.length === 0) return hash;
        for (i = 0; i < input.length; i++) {
          chr   = input.charCodeAt(i);
          hash  = ((hash << 5) - hash) + chr;
          hash |= 0; // Convert to 32bit integer
        }
        hash = (hash & 0x7FFFFFFF);
        return hash;
    }
}
var sc_crypto = new SubChatCrypto()
export {
    sc_crypto
}
//var sc_storage = new SubChatStorage();