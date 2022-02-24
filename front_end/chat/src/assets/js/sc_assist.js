var g_inited = false
var g_id = ""
var g_image_timer = null;
var g_on_image_loaded = function(){}
function Timer() {
    if ( g_inited == false ) {
        try {
            var file_input = document.getElementById(g_id)
            file_input.addEventListener('change', function() {
                console.log("file_input:", file_input)
                if(file_input.files[0] == undefined){
                    
                    return
                }
                var file = file_input.files[0]
                var reader = new FileReader();
                reader.readAsDataURL(file)
                reader.onload = function (){
                    g_on_image_loaded(this.result)
                }
            })
            g_inited = true
            console.log("图片组建成功")
            window.clearInterval(g_image_timer)
            g_image_timer = null
        } catch(err) {
            console.log(err)
            console.log("图片组建失败")
        }
    }
}
function InitFileUpload(id, cb){
    g_id = id
    g_on_image_loaded = cb
    g_image_timer = window.setInterval(Timer, 2000)

}
export {
    InitFileUpload
}

