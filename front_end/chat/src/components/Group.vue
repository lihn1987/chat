<template>
  <el-dialog title="创建群组" v-model="show">
    <div class="clearfix">
      <div class="label left">群组名 </div>
      <el-input  class="content left" v-model="group_name" />
    </div>

    <div class="clearfix">
      <div class="label left">入群审核 </div>
      <el-checkbox v-model="access" class="left"></el-checkbox>
    </div>
    <div class="clearfix">
      <div class="label left">对外可见 </div>
      <el-checkbox v-model="visible" class="left"></el-checkbox>
    </div>
    <el-button type="primary" @click="OnCreate">创建群</el-button>
  </el-dialog>
</template>

<script>

var self = null
export default {
  name: 'Group',
  props: {
    msg: String
  },
 
  data () {
    return {
      show: false,
      group_name: "",
      access: false,
      visible: true,
      create_cb: function(){}
    }
  },
  mounted () {
    self = this
  },
  methods: {
    Init(create_cb) {
      self.create_cb = create_cb
    },
    Show() {
      this.show = true
      console.log("ShowCreateGroup")
    },
    OnCreate() {
      self.create_cb()
    }

  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="scss" scoped>
@import '../assets/css/clearfix.scss';
$label_width: 100px;
.label {
  width: $label_width;
}
.content {
  width: calc( 100% - $label_width );
  box-sizing: border-box;
}
</style>
