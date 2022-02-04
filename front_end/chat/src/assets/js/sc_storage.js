
import { ethers } from 'ethers';
import { sc_crypto } from './sc_crypto'

class SubChatStorage {
    constructor () {}
    CreateAccount (name, pswd) {
        var wallet = ethers.Wallet.createRandom();
        var mnemonic = wallet.mnemonic.phrase;
        this.AddAccount({
            name: name,
            key: sc_crypto.AesEncode( mnemonic, "subchat", pswd )
        })
    }
    GetAccountList () {
        var rtn = []
        var accounts_str = localStorage.getItem("accounts")
        if (!accounts_str) {
            return rtn
        }
        rtn = JSON.parse(accounts_str)
        return rtn
    }
    AddAccount (account)  {
        console.log(this)
        var account_list = this.GetAccountList()
        console.log(account_list)
        for (var i = 0; i < account_list.length; i++) {
            if (account_list[i].name == account.name || account_list[i].key == account.key) {
            return false
            }
        }
        account_list.push(account)
        localStorage.setItem("accounts", JSON.stringify(account_list))
        return true
    }
    GetAccount (account_name, pswd) {
        var account_list = this.GetAccountList()
        var account_mnemonic = ""
        for ( var i = 0; i < account_list.length; i++ ) {
            if ( account_list[i].name == account_name ) {
                account_mnemonic = account_list[i].key
            }
        }
        console.log(account_mnemonic)
        if ( !account_mnemonic ) {
            return null
        } else {
            return sc_crypto.AesDecode(account_mnemonic, "subchat", pswd)
        }
    }
}
var sc_storage = new SubChatStorage()
export {
    sc_storage
}
//var sc_storage = new SubChatStorage();