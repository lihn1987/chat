
import {sc_crypto} from "../sc_crypto.js"
import {
    sc_socket,
    MSG_LOGIN,
    MSG_TXT,
    MSG_AUDIO,
    MSG_AUDIO_TYPE_1,
    MSG_IMAGE
  } from '../sc_net'
var self = null
function InitChatPage(self_) {
    self = self_
}
function ShowAddFriend () {
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
}

function ClickContact (pubkey) {
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
}
function Send () {
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

}
function SendImage (src) {
    console.log("send image", src)
    var time_now = sc_socket.SendImage(self.current_user.pubkey, src)
    // 将消息插入对话框
    self.contract_info[self.current_user.pubkey].msg_list.push({
        from: self.user_info.pubkey,
        to: self.current_user.pubkey,
        msg_type: MSG_IMAGE,
        time: time_now,
        content: src
    })
    // 将消息移动到最下方
    self.$nextTick(()=>{
        var height = self.$refs.box_history.scrollHeight;
        self.$refs.box_history.scrollTo(0, height)
    })
}
function OnReceiveMsg (msg) {
    console.log("收到信息", msg)
    self.AddMsgToContract(msg)
    // 把历史消息滚动条移动到最下方
    self.$nextTick(()=>{
        var height = self.$refs.box_history.scrollHeight;
        console.log("高度", height)
        self.$refs.box_history.scrollTo(0, height)
    })
}
function OnChatInputKeyDown(event) {
    console.log(event)
    if (event.keyCode == 13) {
        if (!event.metaKey) {
        event.preventDefault();
        self.Send();
        } else {
        this.msg_input = this.msg_input + '\n';
        }
    }
}
function OnChatInputBlur(e) {
    this.chat_input_index = e.srcElement.selectionStart; 
}
function ClickEmoji(emoji) {
    this.msg_input = this.msg_input.slice(0, this.chat_input_index) + emoji+this.msg_input.slice(this.chat_input_index, this.msg_input.length)
    this.chat_input_index += 2
}
function ClickSelfLogo() {
    console.log("click logo")
    self.self_dlg.show_setting = true
}
// 修改个人信息，确定
function SelfInfoChange() {
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
}
function GetMsgHistory() {
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
}
function GetRecentMsg(friend_index) {
    if ( this.contract_info[this.user_info.friends[friend_index]].msg_list.length === 0) {
        // 没有历史消息
        return  ""
    } else if (this.contract_info[this.user_info.friends[friend_index]].msg_list[this.contract_info[this.user_info.friends[friend_index]].msg_list.length - 1].msg_type == 2) {
        return "音频"
    } else if (this.contract_info[this.user_info.friends[friend_index]].msg_list[this.contract_info[this.user_info.friends[friend_index]].msg_list.length - 1].msg_type == 3) {
        return "图片"
    } else {
        return this.contract_info[this.user_info.friends[friend_index]].msg_list[this.contract_info[this.user_info.friends[friend_index]].msg_list.length - 1].content
    }
}
function AddMsgToContract(msg) {
    var pubkey_in = ""
    if (msg.base_msg.from == self.user_info.pubkey) {
      pubkey_in = msg.base_msg.to
    } else if(msg.base_msg.to == self.user_info.pubkey) {
      pubkey_in = msg.base_msg.from
    } else {
      console.log("收到一条奇怪的消息！！！！！！！！！！！！")
      return
    }
    console.log((new Date).getSeconds(), (new Date).getMilliseconds(), "插入一条信息", msg)
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
}
// 初始获取所有聊天记录
function GetAllChatMsgByLatestInfo() {
    self.sc_rest.GetAllChatMsgByLatestInfo().then((d) => {
        //OnReceiveMsg
        console.log((new Date).getSeconds(), (new Date).getMilliseconds(),"获取聊天历史记录成功", JSON.parse(d.data.data))
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
}
function StartRecord () {
    this.recorder = new ScAudio();
    this.recorder.StartRecord("canvas")
}
function StopRecord () {
    this.recorder.StopRecord()
    console.log(this.recorder.buffer)
    //this.recorder.Draw("canvas")
}
function StartPlay() {
    this.recorder.StartPlay()
}
function StopPlay() {
    this.recorder.StopPlay()
}
function PlayHistoryAudio(audio_content) {
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
}
function DrawHistoryAudio(audio_content) {
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
}
function ResetAudio() {
    if (this.recorder){
        this.recorder.Reset();
    }
}
  
export {
    InitChatPage,
    ShowAddFriend,
    ClickContact,
    Send,
    SendImage,
    OnReceiveMsg,
    OnChatInputKeyDown,
    OnChatInputBlur,
    ClickEmoji,
    ClickSelfLogo,
    SelfInfoChange,
    GetMsgHistory,
    GetRecentMsg,
    AddMsgToContract,
    GetAllChatMsgByLatestInfo,
    StartRecord,
    StopRecord,
    StartPlay,
    StopPlay,
    PlayHistoryAudio,
    DrawHistoryAudio,
    ResetAudio
}