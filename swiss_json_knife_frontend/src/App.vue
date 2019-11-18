<template>
  <div id="app">
    <SwissJsonKnife msg="Welcome to Your Vue.js App"/>
  </div>
</template>

<script>
import SwissJsonKnife from './components/SwissJsonKnife.vue'

export default {
  name: 'app',
  components: {
    SwissJsonKnife
  },
    created() {
        document.title = "swiss json knife";
    }
}
</script>
