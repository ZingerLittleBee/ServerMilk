<script setup lang="ts">
import { exit } from '@tauri-apps/api/process'
import { invoke } from '@tauri-apps/api/tauri'
import { CheckCircleIcon, ErrorCircleIcon } from 'tdesign-icons-vue-next'
import { onMounted, Ref, ref } from 'vue'
import usePreference, { Preference } from '../hooks/usePreference'

const isEditing = ref(false)

let pre: Ref<Preference> = ref({})
let setPre: Function

const port = ref()

onMounted(async () => {
	const preference = await usePreference()
	pre.value = preference[0].value
	setPre = preference[1]
	port.value = pre.value.port
})

const handleChangePort = async () => {
	if (port.value >= 0 && port.value <= 65535) {
		isEditing.value = false
		await setPre('port', port.value)
		invoke('web_server_restart')
	}
}

const handleLaunchChange = (e: boolean) => {
	setPre('isEnableAutoLaunch', e)
}

const isRunning = ref(false)

const openLog = async () => {
	invoke('open_web_log')
}
const exitProcess = () => {
	exit(0)
}
</script>

<template>
	<div class="settings-container">
		<div class="content-wrapper">
			<div class="content__left"><p>运行状态:</p></div>
			<div class="content__right">
				<t-tag v-if="isRunning" theme="success">
					<CheckCircleIcon />
					正在运行
				</t-tag>
				<t-tag v-else theme="warning">
					<ErrorCircleIcon />
					<span>停止运行</span>
				</t-tag>
			</div>
			<div class="content__left"><p>启动:</p></div>
			<div class="content__right">
				<t-checkbox
					v-model="pre.isEnableAutoLaunch"
					@change="handleLaunchChange"
					><p>登录时启动</p></t-checkbox
				>
			</div>
			<div class="content__left"><p>端口号:</p></div>
			<div class="content__right">
				<div v-if="!isEditing" class="content__right--port">
					<p>{{ pre.port }}</p>
					<t-button
						size="small"
						theme="primary"
						variant="text"
						@click="isEditing = !isEditing"
						>编辑</t-button
					>
				</div>
				<div v-else class="content__right--port">
					<t-input-number
						v-model="port"
						autoWidth
						autofocus
						clearable
						min="0"
						max="65535"
						theme="normal"
						size="small"
					/>
					<t-button
						size="small"
						theme="primary"
						variant="text"
						@click="handleChangePort"
						>确定</t-button
					>
					<t-button
						size="small"
						theme="primary"
						variant="text"
						@click="isEditing = false"
						>取消</t-button
					>
				</div>
			</div>
			<div class="content__left"><p>运行日志:</p></div>
			<div class="content__right">
				<t-button
					theme="default"
					variant="outline"
					size="small"
					@click="openLog"
					>打开日志</t-button
				>
			</div>
			<div class="content__left"><p>更新:</p></div>
			<div class="content__right">
				<t-button theme="default" variant="outline" size="small"
					>检查更新</t-button
				>
			</div>
			<div class="content__left"><p>退出:</p></div>
			<div class="content__right">
				<t-button
					theme="default"
					variant="outline"
					size="small"
					@click="exitProcess"
					>退出 ServerBee</t-button
				>
			</div>
		</div>
	</div>
</template>

<style lang="scss" scoped>
.settings-container {
	@apply flex flex-col items-center justify-center h-full w-full;

	.content-wrapper {
		@apply grid grid-cols-3 gap-4 gap-y-1 justify-center items-center;

		.content__left {
			@apply justify-self-end;
		}

		.content__right {
			@apply col-span-2 w-36;

			.content__right--port {
				@apply flex items-center;
			}
		}
	}
}
</style>
