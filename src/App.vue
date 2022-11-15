<script lang="ts" setup>
import { appWindow } from '@tauri-apps/api/window'
import { onMounted } from 'vue'
import Settings from './pages/settings.vue'

onMounted(async () => {
	let theme = await appWindow.theme()

	theme === 'dark'
		? document.documentElement.setAttribute('theme-mode', 'dark')
		: document.documentElement.removeAttribute('theme-mode')
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
	@apply w-full h-full p-4 pt-0 dark:bg-zinc-800 dark:text-white;
}
</style>
