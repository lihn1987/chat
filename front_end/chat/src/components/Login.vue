<template>
  <div :class="{login_page1: b_show_login}" >
    <div class="login_page2" v-show="b_show_login">
      <div class="login_page3" :style="{width: window_width + 'px', height: window_height + 'px'}">
        <div class="login_box"  >
          <el-row >
            <div class = "label">账户</div>
            <div class = "input">
              <el-select v-model="user_name" placeholder="请选择" style="width:100%">
                <el-option
                  v-for="item in accounts"
                  :key="item.name"
                  :label="item.name"
                  :value="item.name">
                </el-option>
              </el-select>
            </div>
          </el-row>
          <el-row style="margin-top:24px;">
            <div class = "label">密码</div>
            <div class = "input">
              <el-input v-model="pswd"></el-input>
            </div>
          </el-row>
          <el-row style="margin-top:24px;">
            <el-col :span="15">
              <el-button type="primary" style="width:100%" @click="Login">登录</el-button>
            </el-col>
            <el-col :span="3"></el-col>
            <el-col :span="6">
              <el-button type="info" style="width:100%" @click="OnShowNewAccount">新账户</el-button>
            </el-col>
          </el-row>
        </div>
      </div>
    </div>
    <el-dialog
      title="创建账户"
      style="width:400px;"
      width="500px"
      v-model="show_new_account">
      <el-row style="margin-top:24px;">
        <div class = "label">用户名</div>
        <div class = "input">
          <el-input v-model="new_account_name"></el-input>
        </div>
      </el-row>
      <el-row style="margin-top:24px;">
        <div class = "label">密码</div>
        <div class = "input">
          <el-input v-model="new_account_pswd"></el-input>
        </div>
      </el-row>
      <el-button type="primary" style="width:100%; margin-top:24px" @click="OnNewAccount">确  定</el-button>
    </el-dialog>
    <div class="box_main" :style="{width: window_width + 'px', height: window_height + 'px'}" v-if = "!b_show_login">
      <div class="box_left">
        <el-row class = "self">
          <div class = "self-logo" @click="ClickSelfLogo"> </div>
          <div class = "self-info">
            <div class = "self-nick"> {{user_info.nick}} </div>
            <div class = "self-pubkey"> {{user_info.pubkey}} </div>
          </div>
        </el-row>
        <div class = "chat_contact">
          <div class = "add_friend" @click="ShowAddFriend()">添加新联系人</div>
          <el-row  v-bind:class="{ 'contact_item' : user_info.friends[i-1]!==current_user.pubkey , 'contact_item_active' : user_info.friends[i-1]===current_user.pubkey }"  v-for="i in user_info.friends.length"> 
            <el-badge :value="contract_info[user_info.friends[i-1]].unread_count" :max="99" class="item" :hidden="contract_info[user_info.friends[i-1]].unread_count == 0">
              <div class = "contact_logo"> </div>
            </el-badge>
            
            <div class = "contact_info" @click="ClickContact(user_info.friends[i-1])">
              <div class = "contact_nick"> {{contract_info[user_info.friends[i-1]].nick_name}} </div>
              <div class = "contact_recent">{{contract_info[user_info.friends[i-1]].msg_list.length?(contract_info[user_info.friends[i-1]].msg_list[contract_info[user_info.friends[i-1]].msg_list.length - 1].msg_type == 2?"音频":contract_info[user_info.friends[i-1]].msg_list[contract_info[user_info.friends[i-1]].msg_list.length - 1].content): ""}}</div>
            </div>
          </el-row>
        </div>
      </div>
      <div class="box_right">
        <el-row class="box_title">
          <div class="cur_logo"> </div>
          <div class="cur_info"> 
            <div class="cur_nick">{{current_user.nick_name}}</div>
            <div class="cur_pubkey">{{current_user.pubkey}}</div>
          </div>
        </el-row>
        <div class="box_history" id="box_history" ref="box_history">
          <!-- v-bind:class="{ 'his_box_self' : user_info.friends[i-1]!==current_user.pubkey , 'contact_item_active' : user_info.friends[i-1]===current_user.pubkey }" -->
          <el-row v-bind:class="{ 'his_box_self' : contract_info[current_user.pubkey].msg_list[i-1].from===user_info.pubkey , 'his_box_other' : contract_info[current_user.pubkey].msg_list[i-1].from!==user_info.pubkey }"  v-for="i in contract_info[current_user.pubkey].msg_list.length" v-if="current_user.pubkey != ''">
            <div class="his_logo" style="float:left"> </div>
            <div class="his_info" style="float:left"> 
              <div class="his_nick">{{
                contract_info[current_user.pubkey].msg_list[i-1].from===user_info.pubkey?
                '我':contract_info[contract_info[current_user.pubkey].msg_list[i-1].from].nick_name
                }}</div>
              <div class="his_msg" v-if="contract_info[current_user.pubkey].msg_list[i-1].msg_type == 1">{{contract_info[current_user.pubkey].msg_list[i-1].content}}</div>
              <div class="his_msg" v-if="contract_info[current_user.pubkey].msg_list[i-1].msg_type == 2">
                <el-row>
                  <el-button type="primary" @click="PlayHistoryAudio(contract_info[current_user.pubkey].msg_list[i-1].content)" >播放音频</el-button>
                </el-row>
              </div>
            </div>
            <div style="clear:both"></div>
          </el-row>
        </div>
        <el-row  class="box_toolbar" >
          <el-col :span="12">
            <el-popover
              placement="top-start"
              width="160"
              v-model:visible="show_emoji">
                <div class="emoji_box">
                  <div class="emoji" v-for="i in emoji_list.length" @click="ClickEmoji(emoji_list[i-1])">{{emoji_list[i-1]}}</div>
                </div>
              <template #reference>
              <div class="emoji_btn" @click="show_emoji=true"></div>
              </template>
            </el-popover>
          </el-col>
          <el-col :span="12" class="toolbar_right">
            <div :class="{audio_btn: !is_audio, text_btn: is_audio}" @click="is_audio = !is_audio"> </div>
          </el-col>
        </el-row>
        <div class="box_input">
          <el-input
              type="textarea"
              placeholder="请输入内容"
              v-model="msg_input"
              resize=none
              maxlength="10000"
              show-word-limit
              class="msg_input"
              @keydown.enter.native="OnChatInputKeyDown($event)"
              @blur="OnChatInputBlur"
              v-if = "!is_audio"
            />
          <div class="box_input_audio" v-if = "is_audio">
            <div class="box_inner">
              <canvas id="canvas" width="240" height="32"></canvas>
              <el-button @click="StartPlay()" :disabled="current_user.pubkey == '' || (recorder?(recorder.recording||!recorder.buffer.length):true)" class="record_play">播放</el-button>
              <el-button @click="StartRecord()" :disabled="current_user.pubkey == '' || (recorder?recorder.recording:false)" class="record_start">开始录制</el-button>
              <el-button @click="StopRecord()" :disabled="current_user.pubkey == '' || (recorder?!recorder.recording:true)" class="record_stop">停止录制</el-button>
            </div>
          </div>
          <div class="enter_bar">
            <el-button type="primary" :disabled="current_user['pubkey'] == '' || (is_audio && recorder == null) || (is_audio && recorder != null && (recorder.buffer.length == 0 || recorder.recording == true))" @click="Send">发送</el-button>
          </div>
        </div>
      </div>
    </div>
    <el-drawer style="width:100px"
      title="我是标题"
      v-model="self_dlg.show_setting"
      size="200px"
      :with-header="false"
      direction="ltr">
      <div  class="tool_box">
        <div class="tool_item">交易所</div>
        <div class="tool_item">新闻</div>
        <div class="tool_item">频道</div>
        <div class="tool_item">资产</div>
        <div class="tool_item">我的关注</div>
        <div class="tool_item">DAPP</div>
        <div class="tool_item" @click="self_dlg.show_self_config = true">个人设置</div>
      </div>
    </el-drawer>

    <el-dialog title="个人信息" v-model="self_dlg.show_self_config">
      <div class="dlg_self_box">
        <div class="dlg_self_row">
          <div class="dlg_self_label"> 公钥 </div>
          <div class="dlg_self_value"> {{user_info.pubkey}}</div>
        </div>
        <div class="dlg_self_row">
          <div class="dlg_self_label"> 昵称 </div>
          <el-input v-model="self_dlg.self_dlg_nick_name" class="dlg_self_value" @input="self_dlg.edited = true"> </el-input>
        </div>
        <el-button type="primary" style="margin-top: 24px;" :disabled="!self_dlg.edited" @click="SelfInfoChange">确认修改</el-button>
      </div>
    </el-dialog>
  </div>
</template>

<script>
/*contract_info[contract_info[current_user.pubkey].msg_list[i-1].from].nick_name*/
/*contract_info[current_user.pubkey].msg_list[i-1]==user_info.pubkey?'我':contract_info[current_user.pubkey].msg_list[i-1].content*/
import {sc_storage} from "../assets/js/sc_storage.js"
import {sc_crypto} from "../assets/js/sc_crypto.js"
import { ethers } from 'ethers';
import { CreateMsg } from '../assets/js/sc_msg'
import { SubChainRest } from '../assets/js/sc_rest.js'
import {ScAudio} from '../assets/js/sc_audio.js'
import {
  sc_socket,
  MSG_LOGIN,
  MSG_TXT,
  MSG_AUDIO,
  MSG_AUDIO_TYPE_1
} from '../assets/js/sc_net'
import axios from "axios"
var self = null

export default {
  name: 'Login',
  props: {
    msg: String
  },
 
  data () {
    return {
      b_show_login: true,
      accounts: [],
      user_name: '',
      pswd: '',
      show_new_account: false,
      new_account_name: '',
      new_account_pswd: '',
      ws: null,

      window_width: 0,
      window_height: 0,
      sc_rest: null,
      user_info: {
        age: 0,
        nick: "",//"034b6661104faa38afd9840b7eb2e01055eca43ee0b3aa64ec2cf0f78d9e74260f",
        pubkey: "", //"034b6661104faa38afd9840b7eb2e01055eca43ee0b3aa64ec2cf0f78d9e74260f",//"034b6661104faa38afd9840b7eb2e01055eca43ee0b3aa64ec2cf0f78d9e74260f",
        sex: 0,
        friends: [
          //"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c","029ccf9775826f5911af0036550cf80ebd29855b1144eab8d500900d8e4a444dd2"
        ]
      },
      current_user: {
        nick_name: "",//"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c",
        pubkey: ""//"03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c"
      },
      
      contract_info:  {}/*{
       "03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c": {
          nick_name: "03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c",
          unread_count: 10,
          msg_list:[{
            from: "03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c",
            to: "aaa",
            msg_type: 1,
            content: "123456",
            time: 1641712208
          },{
            from: "034b6661104faa38afd9840b7eb2e01055eca43ee0b3aa64ec2cf0f78d9e74260f",
            to: "aaa",
            msg_type: 1,
            content: "aaabbb",
            time: 1641712208
          },{
            from: "03848f4284c4d8bdaa43ce52304cfac8a20fb54e64dcee877e1d10402ef971099c",
            to: "aaa",
            msg_type: 1,
            content: "cccddd",
            time: 1641712208
          }]
        },
      }*/,
      msg_input:'',
      show_emoji: false,
      visible: false,
      emoji_list:[],
      chat_input_index: 0,
      self_dlg:{
        show_setting: false,
        show_self_config: false,
        self_dlg_nick_name: "",
        edited: false
      },
      is_audio: false,
      recorder: null
    }
  },
  mounted () {
    this.Init()
  },
  methods: {
    Init: function () {
      self = this
      this.window_width = window.innerWidth;
      this.window_height = window.innerHeight;
      window.addEventListener('resize', (event) => {
        self.window_width = window.innerWidth;
        self.window_height = window.innerHeight;
      })
      this.ShowLogin(true)
      this.InitEmoji()
    },
    InitEmoji: function () {
      var emoji_list ="😀😁😂🤣😃😄😅😆😉😊😋😎😍😘😗😙😚🙂🤗🤩🤔🤨😐😑😶🙄😏😣😥😮🤐😯😪😫😴😌😛😜😝🤤😒😓😔😕🙃🤑😲☹️🙁😖😞😟😤😢😭😦😧😨😩🤯😬😰😱😳🤪😵😡😠🤬😷🤒🤕🤢🤮🤧😇🤠🤡🤥🤫🤭🧐🤓😈👿👹👺💀👻👽🤖💩😺😸😹😻😼😽🙀😿😾"
      for (var i = 0; i < emoji_list.length; i+=2) {
        self.emoji_list.push(emoji_list.slice(i, i+2))
      }
      console.log(emoji_list.length)
    },
    FlushAccounts: function () {
      this.accounts = sc_storage.GetAccountList()
    },
    ShowLogin: function (bshow) {
      if (bshow) {
        self.b_show_login = true
        self.FlushAccounts()
      } else {
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
                  console.log("获取好友用户信息成功")
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
    },
    OnShowNewAccount: function () {
      this.show_new_account = true
      this.new_account_name = ''
      this.new_account_pswd = ''
    },
    OnNewAccount: function () {
      this.show_new_account = false,
      sc_storage.CreateAccount(this.new_account_name, this.new_account_pswd)
      this.FlushAccounts()
    },
    Login: function () {
      var mnemonic_str = sc_storage.GetAccount(this.user_name, this.pswd)
      var wallet = ethers.Wallet.fromMnemonic(mnemonic_str)
      console.log(wallet.privateKey)
      sc_socket.Open("ws://172.19.18.65:9902", wallet.privateKey.substring(2), this.OnLogin, this.OnReceiveMsg)
      self.sc_rest = new SubChainRest(wallet.privateKey.substring(2))
    },
    OnLogin: function (msg) {
      console.log("收到login的回复", msg)
      self.ShowLogin(false)
    },
    ShowAddFriend: function () {
      this.$prompt('请输入对方公钥', '添加好友', {
        confirmButtonText: '确定添加',
        cancelButtonText: '取消'
      }).then(({ value }) => {
        console.log("开始添加好友")
        self.sc_rest.AddFriendByPubkey(value).then(function(d) {
          var data = d.data
          if (data.error !== 0){
            self.$alert('添加好友失败:'+data.error, '发生错误', {
              confirmButtonText: '确定',
            });
          } else {
            self.user_info.friends.unshift(JSON.parse(data.data).pubkey)
          }
        }).catch(function (e) {
          self.$alert('添加好友失败，'+e.toString(), '发生错误', {
            confirmButtonText: '确定',
          });
        })
      })
    },
    ClickContact: function (pubkey) {
      if ( pubkey != self.current_user.pubkey ) {
        self.current_user.pubkey = pubkey
        self.current_user.nick_name = self.contract_info[pubkey].nick_name;
        if( self.contract_info[self.current_user.pubkey].unread_count !== 0) {
          self.contract_info[self.current_user.pubkey].unread_count = 0
          sc_socket.ClearUnread(pubkey)
        }
        
        console.log(pubkey)
        console.log(self.user_info.friends[0])
        console.log(self.current_user.pubkey)
        console.log(self.user_info.friends[0]===self.current_user.pubkey )
        // 对话移动到最下方
        self.$nextTick(()=>{
          var height = self.$refs.box_history.scrollHeight;
          console.log("高度", height)
          self.$refs.box_history.scrollTo(0, height)
        })
        // 重置音频
        self.ResetAudio();
      }
    },
    test: function(){

    },
    Send: function() {
      if (!this.is_audio) {
        var time_now = sc_socket.SendText(self.current_user.pubkey, this.msg_input)
        // 将消息插入对话框
        self.contract_info[self.current_user.pubkey].msg_list.push({
          from: self.user_info.pubkey,
          to: self.current_user.pubkey,
          msg_type: MSG_TXT,
          time: time_now,
          content: this.msg_input
        })
        this.msg_input = ""
        this.chat_input_index = 0
        if (this.recorder){
          this.recorder.Reset()
        }
      } else {
        var time_now = sc_socket.SendAudio(self.current_user.pubkey, this.recorder.buffer, MSG_AUDIO_TYPE_1)
        // 将消息插入对话框
        self.contract_info[self.current_user.pubkey].msg_list.push({
          from: self.user_info.pubkey,
          to: self.current_user.pubkey,
          msg_type: MSG_AUDIO,
          time: time_now,
          content: JSON.stringify({
              audio_type: MSG_AUDIO_TYPE_1,
              audio_msg: JSON.stringify(this.recorder.buffer)
          })
        })
        this.msg_input = ""
        this.chat_input_index = 0
        if (this.recorder){
          this.recorder.Reset()
        }
      }
      // 将消息移动到最下方
      self.$nextTick(()=>{
        var height = self.$refs.box_history.scrollHeight;
        self.$refs.box_history.scrollTo(0, height)
      })

    },
    OnReceiveMsg: function(msg) {
      console.log("收到信息", msg)
      self.AddMsgToContract(msg)
      // 把历史消息滚动条移动到最下方
      self.$nextTick(()=>{
        var height = self.$refs.box_history.scrollHeight;
        console.log("高度", height)
        self.$refs.box_history.scrollTo(0, height)
      })
    }, 
    OnChatInputKeyDown(event) {
      console.log(event)
      if (event.keyCode == 13) {
        if (!event.metaKey) {
          event.preventDefault();
          self.Send();
        } else {
          this.msg_input = this.msg_input + '\n';
        }
      }
    },
    OnChatInputBlur(e) {
      this.chat_input_index = e.srcElement.selectionStart; 
    },
    ClickEmoji(emoji) {
      this.msg_input = this.msg_input.slice(0, this.chat_input_index) + emoji+this.msg_input.slice(this.chat_input_index, this.msg_input.length)
      this.chat_input_index += 2
    },
    ClickSelfLogo() {
      console.log("click logo")
      self.self_dlg.show_setting = true
    },
    // 修改个人信息，确定
    SelfInfoChange() {
      // console.log(self.self_dlg.self_dlg_nick_name)
      self.sc_rest.ChangeNickName(self.self_dlg.self_dlg_nick_name).then((d) => {
        var data = d.data;
        if ( data.error === 0 ) {
          console.log("修改用户名成功")
          self.user_info.nick = self.self_dlg.self_dlg_nick_name;
          self.self_dlg.self_dlg_nick_name = "";
        } else {
          console.log("获取用户信息失败")
        }

      }).catch((e)=>{
        console.log("获取用户信息失败", e)
      }) 
    },
    GetMsgHistory() {
      self.sc_rest.GetMsgHistory(self.self_dlg.self_dlg_nick_name).then((d) => {
        var data = d.data;
        if ( data.error === 0 ) {
          console.log("修改用户名成功")
          self.user_info.nick = self.self_dlg.self_dlg_nick_name;
          self.self_dlg.self_dlg_nick_name = "";
        } else {
          console.log("获取用户信息失败")
        }

      }).catch((e)=>{
        console.log("获取用户信息失败", e)
      }) 
    },
    AddMsgToContract(msg) {
      var pubkey_in = ""
      if (msg.base_msg.from == self.user_info.pubkey) {
        pubkey_in = msg.base_msg.to
      } else if(msg.base_msg.to == self.user_info.pubkey) {
        pubkey_in = msg.base_msg.from
      } else {
        console.log("收到一条奇怪的消息！！！！！！！！！！！！")
        return
      }
      console.log("插入一条信息", msg)
      if ( !self.contract_info.hasOwnProperty(pubkey_in) ){
        // 先添加一个
        self.contract_info[pubkey_in] = {
          nick_name: pubkey_in,
          unread_count: 1,
          msg_list: [
            {
              from: msg.base_msg.from,
              to: msg.base_msg.to,
              msg_type: msg.base_msg.msg_type,
              time: msg.base_msg.time_stamp,
              content: msg.base_msg.content
            }
          ]
        }
        // 获取用户信息
        self.sc_rest.GetUserInfoInfo(pubkey_in).then((d) => {
          var data = d.data;
          if ( data.error === 0 ) {
            console.log("获取用户信息成功")
            var user_info = JSON.parse(data.data)
            self.contract_info[pubkey_in].nick_name = user_info.nick
            self.user_info.friends.push(pubkey_in)
            // self.$set(self.contract_info, self.contract_info);
            console.log(self.contract_info)
            console.log(self.user_info)
          } else {
            console.log("获取用户信息失败")
          }

        }).catch((e)=>{
          console.log("获取用户信息失败", e)
        }) 
      } else {
        if ( pubkey_in != self.current_user.pubkey) {
          // 如果不是当前聊天窗口则添加一个未读消息
          console.log("add unread_count", self.contract_info[pubkey_in].unread_count)
          self.contract_info[pubkey_in].unread_count += 1
        } else {
          // 如果是当前聊天则清空服务器聊天信息
          sc_socket.ClearUnread(self.current_user.pubkey)
        }
        // 信息插入历史消息中
        self.contract_info[pubkey_in].msg_list.push({
          from: msg.base_msg.from,
          to: msg.base_msg.to,
          msg_type: msg.base_msg.msg_type,
          time: msg.base_msg.time_stamp,
          content: msg.base_msg.content
        })
      }
    },
    // 初始获取所有聊天记录
    GetAllChatMsgByLatestInfo() {
      self.sc_rest.GetAllChatMsgByLatestInfo().then((d) => {
        //OnReceiveMsg
        console.log("获取聊天历史记录成功", JSON.parse(d.data.data))
        var data = JSON.parse(d.data.data)
        for (var friend_pubkey in data.history){
          for (var i = 0; i < data.history[friend_pubkey].msg_list.length; i++) {
            var obj_json = JSON.parse(data.history[friend_pubkey].msg_list[i])
            // 检查消息是否合法
            if (sc_socket.CheckSign(obj_json)) {
              console.log("添加的消息合法")
              if (obj_json.base_msg.from == self.user_info.pubkey){
                obj_json.base_msg.content = sc_crypto.EcdhDecode(obj_json.base_msg.msg, obj_json.base_msg.time_stamp, sc_socket.pri_key, obj_json.base_msg.to)
                self.AddMsgToContract(obj_json);
              }
              else if (obj_json.base_msg.to == self.user_info.pubkey) {
                obj_json.base_msg.content = sc_crypto.EcdhDecode(obj_json.base_msg.msg, obj_json.base_msg.time_stamp, sc_socket.pri_key, obj_json.base_msg.from)
                self.AddMsgToContract(obj_json);
              }
              else {
                console.log("收到了诡异的消息")
              }
            } else {
              console.log("添加的消息非法!!!!!!!!!!!!!!!!!")
              continue
            } 
          }
          console.log("开始设置未读！！！！！！！！！！！！！！！！！！！！！！！！！！！！！")
          console.log(data.history[friend_pubkey].unread_count)
          console.log(self.contract_info[friend_pubkey].unread_count)
          
          self.contract_info[friend_pubkey].unread_count = data.history[friend_pubkey].unread_count
        }
      }).catch((e)=>{
        console.log("获取聊天历史记录出错", e)
      })
    },
    StartRecord: function() {
      this.recorder = new ScAudio();
      this.recorder.StartRecord("canvas")
    },
    StopRecord: function() {
      this.recorder.StopRecord()
      console.log(this.recorder.buffer)
      //this.recorder.Draw("canvas")
    },
    StartPlay() {
      this.recorder.StartPlay()
    },
    StopPlay() {
      this.recorder.StopPlay()
      
    },
    PlayHistoryAudio(audio_content) {
      var audio_json = JSON.parse(audio_content)
      audio_json.audio_msg = JSON.parse(audio_json.audio_msg)
      console.log(audio_json)
      // console.log(self.recorder)
      var audio = new ScAudio()
      audio.Init("")
      var buf = []
      for(var i = 0; i < audio_json.audio_msg.length; i++ ) {
        var array = new Float32Array(Object.keys(audio_json.audio_msg[i]).length)

        for ( var k in audio_json.audio_msg[i]){
          array[k] = audio_json.audio_msg[i][k] 
        }
        buf.push(array)
      }
      audio.StartPlayBuffer(audio_json.audio_type, buf)
    },
    DrawHistoryAudio(audio_content) {
      var hash = this.Hash(audio_content)
      var audio_json = JSON.parse(audio_content)
      audio_json.audio_msg = JSON.parse(audio_json.audio_msg)
      console.log(audio_json)

      var buf = []
      for(var i = 0; i < audio_json.audio_msg.length; i++ ) {
        var array = new Float32Array(Object.keys(audio_json.audio_msg[i]).length)

        for ( var k in audio_json.audio_msg[i]){
          array[k] = audio_json.audio_msg[i][k] 
        }
        buf.push(array)
      }
      ScAudio.DrawByBuf("canvas_"+hash, buf)
    },
    ResetAudio() {
      if (this.recorder){
        this.recorder.Reset();
      }
    },
    Hash(input) {
      return sc_crypto.Hash(input)
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="scss" scoped>
@import '../assets/css/main.scss';
@import '../assets/css/main.css';
</style>
