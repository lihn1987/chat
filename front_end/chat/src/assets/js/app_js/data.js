var app_data = {
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
export {
    app_data
}