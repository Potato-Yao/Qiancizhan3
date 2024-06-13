<template>
  <v-app>
    <Sidebar/>
    <v-main>
      <StartMenu/>
    </v-main>
  </v-app>
</template>

<script setup lang="ts" name="App">
  //
import StartMenu from "./components/StartMenu.vue";
import Sidebar from "./components/Sidebar.vue";
</script>
