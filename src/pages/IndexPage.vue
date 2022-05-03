<template>
  <div class="row items-start justify-evenly q-ma-xl">
    <div class="col-5">
      <q-input
        v-model="inputText"
        outlined
        rows="20"
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
        rows="20"
        type="textarea"
      />
    </div>
  </div>
</template>

<script>
import init, {convert} from '../wasm/mylib.js';

init()

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
      inputText: '',
      outputText: ''
    }
  },
  methods: {
    translate() {
      convert(this.inputText).then(outputText => {
        this.outputText = outputText
        console.log(outputText)
      })
    }
  }
}
</script>
