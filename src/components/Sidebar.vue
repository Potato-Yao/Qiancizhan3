<script setup lang="ts" name="Sidebar">
import {useI18n} from "vue-i18n";

let {t} = useI18n()
</script>

<template>
  <v-navigation-drawer app>
    <v-list>
      <v-list-item link to="/select-word-book">
        {{t("choose_wordbook_button")}}
      </v-list-item>
      <v-list-item link to="/study">
        {{t("study_button")}}
      </v-list-item>
      <v-list-item link to="/edit-word-book">
        {{t("edit_word_book_button")}}
      </v-list-item>
      <v-list-item link to="/settings">
        {{t("settings_button")}}
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<style scoped>

</style>
