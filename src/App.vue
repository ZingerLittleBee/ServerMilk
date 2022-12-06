<script lang="ts" setup>
import {appWindow, Theme} from '@tauri-apps/api/window'
import {onMounted, onUnmounted} from 'vue'
import Settings from './pages/settings.vue'
import {UnlistenFn} from "@tauri-apps/api/event";

const checkTheme = async (theme?: Theme | null) => {
  theme = theme ? theme : await appWindow.theme()

  theme === 'dark'
      ? document.documentElement.setAttribute('theme-mode', 'dark')
      : document.documentElement.removeAttribute('theme-mode')
}

let unlisten: UnlistenFn

onMounted(async () => {
  await checkTheme()
  unlisten = await appWindow.onThemeChanged(({payload: theme}) => {
    checkTheme(theme)
  });
})

onUnmounted(() => {
  unlisten()
})


</script>

<template>
  <div class="container">
    <settings></settings>
  </div>
</template>

<style>
html,
body {
  width: 100%;
  height: 100%;
}

#app {
  height: 100%;
}

.container {
  @apply w-full h-full p-4 dark:bg-zinc-800 dark:text-white;
}
</style>
