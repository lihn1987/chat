<template>
  <div>
    asdf
  </div>
</template>

<script>

var self = null

export default {
  name: 'Login',
  props: {
    msg: String
  },
 
  data () {
    return {
    }
  },
  mounted () {
    this.Init()
  },
  methods: {
    Init: function () {
      self = this
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="scss" scoped>
@import '../assets/css/main.scss';
@import '../assets/css/main.css';
</style>
