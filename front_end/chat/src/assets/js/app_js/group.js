
var self = null
import {SubChainRest} from "../sc_rest.js"
// 初始化
function InitGroup (self_in) {
    self = self_in
}

// will create group
function OnOpenCreateGroupDlg() {
    console.log("===>")
    console.log(self)
    console.log(self.$refs)
    self.$refs.create_group_dlg.Show()
}

// 创建群
function CreateGroup() {
    var group_name = self.$refs.create_group_dlg.group_name
    // pub const GROUP_ACCESS_ANYONE: u32 = 1;             // 群进入权限，所以人可加入
    // pub const GROUP_ACCESS_NEED_OWNER_ALLOW: u32 = 2;   // 群进入权限，需要拥有者允许
    var access = self.$refs.create_group_dlg.access?1:2
    var visible = self.$refs.create_group_dlg.visible
    
    console.log(group_name, access, visible)
    self.sc_rest.CreateGroup(group_name, access, visible)
}
export {
    InitGroup,
    OnOpenCreateGroupDlg,
    CreateGroup
}