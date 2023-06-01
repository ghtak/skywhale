<script>
export default {
  data(){
    return {
      count : 0
    }
  },

  methods: {
    increment(){
      this.count++
    }
  },

  mounted() {
    console.log(`${this.count}`)
  }
}
</script>
<template>
    <button @click="increment">
      {{ count }}
    </button>
</template>