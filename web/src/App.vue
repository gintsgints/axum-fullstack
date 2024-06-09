<script setup lang="ts">
import { ref } from "vue"
import HelloWorld from './components/HelloWorld.vue'

const drawer = ref(false);
</script>

<template>
  <v-app id="inspire">
    <v-navigation-drawer v-model="drawer">
      <!--  -->
    </v-navigation-drawer>

    <v-app-bar>
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>

      <v-app-bar-title>Application</v-app-bar-title>
    </v-app-bar>

    <v-main>
      <HelloWorld msg="Vite + Vue" />
    </v-main>
  </v-app>
</template>
