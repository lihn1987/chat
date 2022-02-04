<template>
  
</template>

<script>
export default {
  name: 'DefaultVue',
  props: {
    msg: String
  },
  data() {
    return {
      window_width: 0,
      window_height: 0
    }
  },
  mounted: function () {
    this.window_width = window.innerWidth;
    this.window_height = window.innerHeight;
    var self = this
    window.addEventListener('resize', (event) => {
      self.window_width = window.innerWidth;
      self.window_height = window.innerHeight;
    })
  }
}
</script>

<style lang="scss" scoped>
$left_width: 200px;
$box__title_high: 48px;
$box_toolbar_high: 36px;
$box_input_high: 164px;


.box_main {
  background:#ffff00;
  display: flex;
  flex-direction: row;
}
.box_left {
  background: #ff0000;
  width: $left_width;
  height: 100%;
}
.box_right {
  background: #00ff00;
  width: calc(100% - 200px);
  height: 100%;
}
.box_title {
  background: #00ffff;
  width: 100%;
  height: $box__title_high;
}

.box_history {
  background: #000fff;
  width: 100%;
  height: calc( 100% - $box__title_high - $box_toolbar_high - $box_input_high );
}

.box_toolbar {
  background: #afff0f;
  width: 100%;
  height: $box_toolbar_high;
}

.box_input {
  background: #a0ffff;
  width: 100%;
  height: $box_input_high;
}

</style>
