

class ScAudio {
    constructor () {
        this.Reset()
    }
    Reset() {
        this.context = new AudioContext({ sampleRate: 8000, sampleBits: 16 })
        this.buffer = []
        this.encoded = []
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
    DrawByBuf(id, buffer) {
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
            for (var i = 0; i < buffer.length; i++ ) {
                var sum = 0;
                for (var j = 0; j < buffer[i].length; j++ ) {
                    sum += Math.abs(buffer[i][j])
                }
                var value = Math.min( sum/buffer[i].length, 0.2 )
                value = Math.max( sum/buffer[i].length, 0.02 )
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
    Encode() {
        console.log("encode!")
        var i32ptr = libspeex.allocate(1, 'i32', libspeex.ALLOC_STACK)
        var bits_addr = libspeex.allocate(1, 'i8', libspeex.ALLOC_STACK)
        libspeex._speex_bits_init(bits_addr)
        var state = libspeex._speex_encoder_init(libspeex._speex_lib_get_mode(0));
        libspeex.setValue(i32ptr, 8, 'i32');
        libspeex._speex_encoder_ctl(state, SPEEX_SET_QUALITY, i32ptr);

        libspeex._speex_encoder_ctl(state, SPEEX_GET_FRAME_SIZE, i32ptr);
        var frame_size = libspeex.getValue(i32ptr, 'i32');
        console.log("frome_Size:", frame_size)
        console.log("音频buffer数为:",this.buffer.length)
        var one_buf = []
        for (var i = 0; i < this.buffer.length; i++) {
            for (var j = 0; j < this.buffer[i].length; j++) {
                one_buf.push(this.buffer[i][j]*32767)
            }
        }
        // 计算frame的总数
        var frames_count = one_buf.length/frame_size+1;
        // 多填充一个frame的空数据
        for (var i = 0; i < frame_size; i++) {
            one_buf.push(0)
        }
        console.log("one_buf", one_buf)
        console.log("frames_count", frames_count)
        libspeex._speex_bits_reset(bits_addr);
        for(var i = 0; i < frames_count; i++) {
            console.log("i=====>", i)
            console.log(1)
            
            console.log(2)
            // 初始化编码frame
            var audio_in_buf = libspeex.allocate(one_buf.slice(i*frame_size, (i+1)*frame_size), 'i16', libspeex.ALLOC_STACK)
            console.log(3)
            // 初始化编码器
            libspeex._speex_encode(state, audio_in_buf, bits_addr);
            console.log(4)
            var count = libspeex._speex_bits_nbytes(bits_addr);
            console.log(5)
            var frame_buf = libspeex.allocate(count, 'i8', libspeex.ALLOC_STACK);
            console.log("加密前", frame_buf)
            libspeex._speex_bits_write(bits_addr, frame_buf, count+100);
            console.log("加密后", frame_buf)
        }
    }

}
export {
    ScAudio
}

/*

var Codec = {
    speex: new Speex({quality: 6}),

    // TODO(Bieber): See if you need to make a copy before returning the buffer
    encode: function(buffer) {
        // To preserve length, encode a multiple of 320 samples.
        var datalen = buffer.length;
        var shorts = new Int16Array(datalen);
        for(var i = 0; i < datalen; i++) {
            shorts[i] = Math.floor(Math.min(1.0, Math.max(-1.0, buffer[i])) * 32767);
        }
        var encoded = Codec.speex.encode(shorts, true);
        return encoded[0];
    },

    decode: function(buffer) {
        return Codec.speex.decode(buffer);
    }
};
*/