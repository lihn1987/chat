import {sc_storage} from "../sc_storage.js"
import { ethers } from 'ethers';
import { SubChainRest } from '../sc_rest.js'
import {InitFileUpload} from '../sc_assist.js'
import {
    sc_socket,
    MSG_LOGIN,
    MSG_TXT,
    MSG_AUDIO,
    MSG_AUDIO_TYPE_1,
    MSG_IMAGE
  } from '../sc_net'
var self = null
// 初始化
function Init () {
    self = this
    this.window_width = window.innerWidth;
    this.window_height = window.innerHeight;
    window.addEventListener('resize', (event) => {
        self.window_width = window.innerWidth;
        self.window_height = window.innerHeight;
    })
    this.ShowLogin(true)
    this.InitEmoji()
}

// 初始化emoji
function InitEmoji () {
    var emoji_list ="😀😁😂🤣😃😄😅😆😉😊😋😎😍😘😗😙😚🙂🤗🤩🤔🤨😐😑😶🙄😏😣😥😮🤐😯😪😫😴😌😛😜😝🤤😒😓😔😕🙃🤑😲☹️🙁😖😞😟😤😢😭😦😧😨😩🤯😬😰😱😳🤪😵😡😠🤬😷🤒🤕🤢🤮🤧😇🤠🤡🤥🤫🤭🧐🤓😈👿👹👺💀👻👽🤖💩😺😸😹😻😼😽🙀😿😾"
    for (var i = 0; i < emoji_list.length; i+=2) {
        self.emoji_list.push(emoji_list.slice(i, i+2))
    }
    console.log(emoji_list.length)
}

// 刷新所有的账户列表
function FlushAccounts () {
    this.accounts = sc_storage.GetAccountList()
}

// 显示页面，true显示登录页面，false显示聊天页面
function ShowLogin (bshow) {
    if (bshow) {
        self.b_show_login = true
        self.FlushAccounts()
    } else {
        InitFileUpload("image_upload", this.SendImage)
        self.b_show_login = false
        // return
        self.sc_rest.GetSelfInfo().then((d) => {
        var data = d.data;
        if ( data.error === 0 ) {
            console.log("获取个人信息成功")
            self.user_info = JSON.parse(data.data)
            self.self_dlg.self_dlg_nick_name = self.user_info.nick
            for (var i = 0; i < self.user_info.friends.length; i++) {
            console.log(self.user_info.friends[i])
            
            self.contract_info[self.user_info.friends[i]] = {
                nick_name: self.user_info.friends[i],
                unread_count: 0,
                msg_list:[]
            }
            }

            for (var i = 0; i < self.user_info.friends.length; i++) {
            console.log("===========================================")
            console.log(self.user_info.friends[i])
            self.sc_rest.GetUserInfoInfo(self.user_info.friends[i]).then((d) => {
                var data = d.data;
                console.log("aaaaaa", data)
                if ( data.error === 0 ) {
                console.log((new Date).getSeconds(), (new Date).getMilliseconds(),"获取好友用户信息成功")
                var user_info = JSON.parse(data.data)
                // console.log("ccccc", user_info.nick, self.user_info.friends[i], self.contract_info[self.user_info.friends[i]])
                self.contract_info[user_info.pubkey].nick_name = user_info.nick
                }
            }).catch((e) => {
                console.log("获取好友用户信息失败")
            }) 
            }
            self.GetAllChatMsgByLatestInfo()
        } else {
            console.log("获取个人信息失败")
        }

        }).catch((e)=>{
        console.log("获取个人信息失败", e)
        }) 
    }
}
// 显示创建账户页面
function OnShowNewAccount () {
    this.show_new_account = true
    this.new_account_name = ''
    this.new_account_pswd = ''
}
  // 用户创建新账户
function OnNewAccount () {
    this.show_new_account = false,
    sc_storage.CreateAccount(this.new_account_name, this.new_account_pswd)
    this.FlushAccounts()
}
// 登录
function Login () {
    var mnemonic_str = sc_storage.GetAccount(this.user_name, this.pswd)
    var wallet = ethers.Wallet.fromMnemonic(mnemonic_str)
    console.log(wallet.privateKey)
    sc_socket.Open("ws://172.19.18.65:9902", wallet.privateKey.substring(2), this.OnLogin, this.OnReceiveMsg)
    self.sc_rest = new SubChainRest(wallet.privateKey.substring(2))
}

// 登录完成
function OnLogin (msg) {
    console.log("收到login的回复", msg)
    self.ShowLogin(false)
}

export {
Init,
InitEmoji,
FlushAccounts,
ShowLogin,
OnShowNewAccount,
OnNewAccount,
Login,
OnLogin
}