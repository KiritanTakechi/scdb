<template>
  <div class="scdb-app">
    <div class="scdb-app__navbar">
      <NavBar></NavBar>
    </div>
    <div class="scdb-app__display">
      <router-view></router-view>
    </div>
  </div>
</template>

<script setup lang="ts">
import NavBar from "./components/NavBar.vue";

</script>

<style scoped>
.scdb-app {
  display: flex;
}

.scdb-app__navbar {
  height: 100dvh;

  /* padding: 5px; */

  flex-grow: 0;
  background-color:#004469;
}

.scdb-app__display {
  height: 100dvh;
  flex-grow: 1;
  background-color: brown;
}
</style>
