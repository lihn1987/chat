
import { ethers } from 'ethers';


function CreateMsg(pri_key, msg_type, to, msg) {
    var wallet = ethers.Wallet.fromMnemonic(mnemonic);
    // var msg_base = {
    //     msg_type: msg_type,
    //     from: 
    // }
}

function CreateLoginMsg (pri_key, msg_type,) {
    var msg = {
        msg_type: 0,
        
    }
}


export {
    CreateMsg,
    CreateLoginMsg
}