<template>
  <div class="row items-start justify-evenly q-ma-xl">
    <div class="col-5">
      <q-input
        v-model="inputText"
        outlined
        rows="35"
        type="textarea"
      />
      <div class="row items-center justify-center">
        <q-btn color="primary" label="Translate" class="q-ma-sm" @click="translate" />
      </div>
    </div>
    <div class="col-5">
      <q-input
        v-model="outputText"
        outlined
        rows="35"
        type="textarea"
      />
    </div>
  </div>
</template>

<script>
import init, {Converter} from '../wasm/mylib.js';

function loadDict(url, promise) {
  fetch(url, {
    method: 'GET',
    headers: {
      'Accept': 'application/text',
    },
  })
  .then(res => res.text())
  .then(promise)
}

let converter;

init().then(() => {
  converter = Converter.new();

  loadDict('dicts/vietphrase.txt', data => {
    converter.set_vietphrase_dict(data)
  })

  loadDict('dicts/names.txt', data => {
    converter.set_names_dict(data)
  })

  loadDict('dicts/hanviet.txt', data => {
    converter.set_hanviet_dict(data)
  })

  loadDict('dicts/luatnhan.txt', data => {
    converter.set_luatnhan_dict(data)
  })

  loadDict('dicts/pronouns.txt', data => {
    converter.set_pronouns_dict(data)
  })

})


export default {
  setup () {
    return {
      contentStyle: {
        backgroundColor: 'rgba(0,0,0,0.02)',
        color: '#555'
      },

      contentActiveStyle: {
        backgroundColor: '#eee',
        color: 'black'
      },

      thumbStyle: {
        right: '2px',
        borderRadius: '5px',
        backgroundColor: '#027be3',
        width: '5px',
        opacity: '0.75'
      }
    }
  },
  data() {
    return {
      inputText: 
`第一章 太阳消失
时间:2012年12月22日
...
"中央电视台!中央电视台!观众朋友们,您现在收看的是中央电视台!"
"观众朋友们!我现在的位置是纽约市的皇后区,这里刚刚发生了一起骚乱,纽约警方动用了大量人员强行驱散了骚乱人群,这次骚乱造成了一人死亡,三十多人受伤,有关情况还在继续了解..."
...
"现在我们再联系一下欧洲的孙菲,我们了解一下那边的情况."
"主持人您好!"
"孙菲您好,你现在能和我们说一下巴黎的情况吗?"`,
      outputText: ''
    }
  },
  methods: {
    translate() {
      var startTime = performance.now()
      this.outputText = converter.convert(this.inputText)
      var endTime = performance.now()
      console.log(`Convert takes ${endTime - startTime} milliseconds`)
    }
  }
}
</script>
