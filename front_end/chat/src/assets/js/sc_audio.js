class ScAudio {
    constructor () {
        this.Reset()
    }
    Reset() {
        this.context = new AudioContext({ sampleRate: 8000, sampleBits: 16 })
        this.buffer = []
        this.recording = false
        if (this.stream ) {
            this.stream.getTracks().forEach((track) => {
                track.stop()
            })
        }
        this.stream = null
        if (this.id) {
            this.Draw(this.id)
        }
        
        this.id = ""
    }
    Init(id) {
        let self = this
        this.id = id
        navigator.mediaDevices.getUserMedia({
            audio: true,
            video: false
          }).then(stream => {
            self.stream = stream
            const sourceNode = self.context.createMediaStreamSource(stream)
            const scriptNode = self.context.createScriptProcessor(8192, 1, 1)
            sourceNode.connect(scriptNode)
            scriptNode.connect(self.context.destination)
            // 监听录音的过程
            scriptNode.onaudioprocess = (event) => {
            
              if (self.recording){
                var tmp = new Float32Array(8192)
                var data = event.inputBuffer.getChannelData(0)
                for (var i = 0; i < data.length; i++){
                    tmp[i] = data[i]
                }
                self.buffer.push(tmp)
                self.Draw(id)
              } else {
                  stream.getTracks().forEach((track) => {
                      track.stop()
                  })
              }
            }
          }).catch(err => {
            console.log(err)  // 错误回调
            return false
          })
    }
    StartRecord(id) {
        this.recording = true
        this.Init(id)
        console.log("start record")
        
        //this.buffer = []
    }
    StopRecord() {
        console.log("stop record")
        this.recording = false
       
        // console.log(this.buffer[0],this.buffer[1])
        // console.log(this.buffer[0][0] == this.buffer[1][0])
        // console.log(this.buffer[0][0], this.buffer[1][0])
    }
    StartPlay() {
        var audio_buffer = new AudioBuffer({
            length: this.buffer.length*8192,
            sampleRate: 8000
        });
        for(var i = 0; i < this.buffer.length; i++){
            audio_buffer.copyToChannel(this.buffer[i], 0, i*8192)
        }
        
        console.log("===============================================================")
        var source = this.context.createBufferSource();
        source.buffer = audio_buffer
        source.connect(this.context.destination);
        source.start(0)
    }
    StartPlayBuffer( audio_type, buffer ) {
        console.log("StartPlayBuffer", buffer)
        var audio_buffer = new AudioBuffer({
            length: buffer.length*8192,
            sampleRate: 8000
        });
        for(var i = 0; i < buffer.length; i++){
            audio_buffer.copyToChannel(buffer[i], 0, i*8192)
        }
        
        console.log("===============================================================")
        var source = this.context.createBufferSource();
        source.buffer = audio_buffer
        source.connect(this.context.destination);
        source.start(0)
    }
    Draw(id) {
        var canvas = document.getElementById(id)
        if ( canvas.getContext ) {
            // console.log("开始绘图")
            var ctx = canvas.getContext("2d") 
            
            var height = 32;
            var width = 240;
            var scale_w = 2/3;
            var scale_h = 0.9
            var len = 60;
            // 清空屏幕
            ctx.fillStyle="#ffffff"
            ctx.fillRect(0, 0, width, height)
            // console.log(this.buffer.length)
            var value_list = []
            for (var i = 0; i < this.buffer.length; i++ ) {
                var sum = 0;
                for (var j = 0; j < this.buffer[i].length; j++ ) {
                    sum += Math.abs(this.buffer[i][j])
                }
                var value = Math.min( sum/this.buffer[i].length, 0.2 )
                value = Math.max( sum/this.buffer[i].length, 0.02 )
                value_list.push(value)
            }
            
            var max_value = 0
            value_list.forEach((v)=>{
                if ( v > max_value ) {
                    max_value = v
                }
            })
            var scale = 0.2/max_value
            // console.log(value_list, scale, max_value)
            for ( var i = 0; i <= len; i++) {
                var value = 0.02
                if ( i < value_list.length) {
                    value = value_list[i]
                    value *= scale
                }
                ctx.fillStyle="#1ecc33"
                ctx.fillRect(i*(width/len), ((0.2-value)/0.2)*height, width*scale_w/len, height)
                // console.log(i*(width/len), ((0.2-value)/0.2)*height, width*scale_w/len, height)
                
            }
            
            // ctx.fillRect(i*(width/len), 2, width/len*scale_w, 80)
            // console.log(i*(width/len), 2, width/len*scale_w, 80)
        } else {
            console.log("无法绘图")
        }
    }

}
export {
    ScAudio
}