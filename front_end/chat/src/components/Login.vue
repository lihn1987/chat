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
              <div class = "contact_recent">{{
                GetRecentMsg(i-1)
                }}
              </div>
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
              <div class="his_msg" v-if="contract_info[current_user.pubkey].msg_list[i-1].msg_type == 3">
                <el-image
                  style="width: 100%; height: 300px"
                  :src="contract_info[current_user.pubkey].msg_list[i-1].content"
                  fit="scale-down"></el-image>
              </div>
            </div>
            <div style="clear:both"></div>
          </el-row>
        </div>
        <el-row  class="box_toolbar" >
          <el-col :span="12">
            <el-row>
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
              <div class="image_btn" onclick="image_upload.click()"></div>
              <input type="file"  style="display:none" id="image_upload" accept="image/jpeg, image/png, image/jpg">
            </el-row>
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
        <div class="tool_item" @click="OnOpenCreateGroupDlg()">创建群</div>
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

    <Group ref="create_group_dlg"/>

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
import {InitFileUpload} from '../assets/js/sc_assist.js'
import {app_data} from '../assets/js/app_js/data.js'
import {
  Init,
  InitEmoji,
  FlushAccounts,
  ShowLogin,
  OnShowNewAccount,
  OnNewAccount,
  Login,
  OnLogin
} from '../assets/js/app_js/login_page.js'
import {
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
} from '../assets/js/app_js/chat_page.js'
import {
  sc_socket,
  MSG_LOGIN,
  MSG_TXT,
  MSG_AUDIO,
  MSG_AUDIO_TYPE_1,
  MSG_IMAGE
} from '../assets/js/sc_net'
import axios from "axios"
import {
  InitGroup,
  OnOpenCreateGroupDlg,
  CreateGroup
} from '../assets/js/app_js/group' 
import Group from "./Group.vue"
var self = null

export default {
  name: 'Login',
  props: {
    msg: String
  },
  components: {
    Group
  },
  data () {
    return app_data
  },
  mounted () {
    // this.$refs.create_group_dlg.Show()
    self = this
    this.Init()
    InitGroup(this)
    InitChatPage(this)
    this.$refs.create_group_dlg.Init(this.CreateGroup)
  },
  methods: {
    // 初始化
    Init: Init,
    // 初始化emoji
    InitEmoji: InitEmoji,
    // 刷新所有的账户列表
    FlushAccounts: FlushAccounts,
    // 显示页面，true显示登录页面，false显示聊天页面
    ShowLogin: ShowLogin,
    // 显示创建账户页面
    OnShowNewAccount: OnShowNewAccount,
    // 用户创建新账户
    OnNewAccount: OnNewAccount,
    // 登录
    Login: Login,
    // 登录完成
    OnLogin: OnLogin,


    ShowAddFriend:ShowAddFriend,
    ClickContact:ClickContact,
    Send:Send,
    SendImage:SendImage,
    OnReceiveMsg:OnReceiveMsg,
    OnChatInputKeyDown:OnChatInputKeyDown,
    OnChatInputBlur:OnChatInputBlur,
    ClickEmoji:ClickEmoji,
    ClickSelfLogo:ClickSelfLogo,
    SelfInfoChange:SelfInfoChange,
    GetMsgHistory:GetMsgHistory,
    GetRecentMsg:GetRecentMsg,
    AddMsgToContract:AddMsgToContract,
    GetAllChatMsgByLatestInfo:GetAllChatMsgByLatestInfo,
    StartRecord:StartRecord,
    StopRecord:StopRecord,
    StartPlay:StartPlay,
    StopPlay:StopPlay,
    PlayHistoryAudio:PlayHistoryAudio,
    DrawHistoryAudio:DrawHistoryAudio,
    ResetAudio:ResetAudio,
    // 当打开创建群的界面
    OnOpenCreateGroupDlg:OnOpenCreateGroupDlg,
    // 当点击创建群
    CreateGroup: CreateGroup,
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="scss" scoped>
@import '../assets/css/main.scss';
@import '../assets/css/main.css';
</style>
