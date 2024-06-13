<script setup lang="ts" name="StartMenu">
import {ref} from "vue"
import {invoke} from '@tauri-apps/api/tauri'
import {useI18n} from "vue-i18n"

const {t} = useI18n()
let bookList = ref(new Map<String, String>)

invoke("get_wordbooks").then((res: any) => {
  for (const [k, v] of Object.entries(res)) {
    bookList.value.set(k.toString(), v.toString())
    console.log(bookList.value)
  }
})
</script>

<template>
  <v-container class="StartMenu">
    <h2>{{ t('greet') }}</h2>
    <h3>{{ t('choose_wordbook') }}</h3>
    <v-list lines="two">
      <v-list-item v-for="(item, index) in bookList" :key="index" :title="item[0]" :subtitle="item[1]"></v-list-item>
    </v-list>
  </v-container>
</template>

<style scoped>

</style>
