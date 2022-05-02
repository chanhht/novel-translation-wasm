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
      <q-scroll-area 
        :thumb-style="thumbStyle"
        :content-style="contentStyle"
        :content-active-style="contentActiveStyle"
        style="height: 700px"
      >
        <div v-for="n in 100" :key="n" class="q-py-xs">
          Lorem ipsum dolor sit amet, consectetur adipisicing
          elit, sed do eiusmod tempor incididunt ut labore et
          dolore magna aliqua.
        </div>
      </q-scroll-area>
    </div>
  </div>
</template>

<script>
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
      inputText: ''
    }
  },
  methods: {
    translate() {
      console.log(this.inputText)
    }
  }
}
</script>
