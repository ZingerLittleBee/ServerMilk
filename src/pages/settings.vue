<script setup lang="ts">
import {getVersion} from '@tauri-apps/api/app'
import {exit} from '@tauri-apps/api/process'
import {invoke} from '@tauri-apps/api/tauri'
import {CheckCircleIcon, ErrorCircleIcon, JumpIcon, RefreshIcon} from 'tdesign-icons-vue-next'
import {onMounted, reactive, Ref, ref} from 'vue'
import usePreference, {Preference} from '../hooks/usePreference'
import useUpdater from '../hooks/useUpdater'
import useCommander from "../hooks/useCommander";
import {Body, fetch, ResponseType} from '@tauri-apps/api/http';
import {PhEyeSlash, PhEye} from "@phosphor-icons/vue";
import {MessagePlugin} from "tdesign-vue-next";
import {randomString} from "../utils/util";

const isEditing = ref(false)

let pre: Ref<Preference> = ref({})
let setPre: Function
let setEnableAutoLaunch: Function

const port = ref()

const isRunning = ref(true)

const appVersion = ref()

let start, restart: (port: number) => Promise<void>, stop

onMounted(async () => {
  ({start, restart, stop} = await useCommander())
  const preference = await usePreference()
  pre.value = preference[0].value
  setPre = preference[1]
  setEnableAutoLaunch = preference[2]
  port.value = pre.value.port
  appVersion.value = await getVersion()

  await start(port.value)
})

const handleLaunchChange = (e: boolean) => {
  setEnableAutoLaunch(e)
}

const portError = reactive({
  isErr: false,
  msg: ''
})

const checkPort = async (port: number): Promise<boolean> => invoke('is_free_port', {port: port})

const changePort = async () => {
  if (port.value >= 0 && port.value <= 65535) {
    if (await checkPort(port.value)) {
      isEditing.value = false
      await restart(port.value)
      await setPre('port', port.value)
    } else {
      portError.isErr = true
      portError.msg = '端口已被占用, 请更换其他端口'
    }
  } else {
    portError.isErr = true
    portError.msg = '端口范围为 0 - 65535'
  }
}

const changePortCancel = () => {
  isEditing.value = false
  port.value = pre.value.port
  portError.isErr = false
}

const openLog = async () => {
  invoke('open_web_log')
}

const openDocs = async () => {
  invoke('open_web_log')
}
const exitProcess = () => {
  exit(0)
}

const isRefresh = ref(false)

const refreshStatus = async () => {
  isRefresh.value = true
  isRunning.value = await invoke<boolean>('check_web_status', {
    port: pre.value.port
  })
  setTimeout(() => {
    isRefresh.value = false
  }, 1000)
}

const checkUpdate = () => useUpdater(true)

const apiKey = ref('')
const apiKeyVisible = ref(false)
const apiKeyIsEditing = ref(false)
const newApiKey = ref<string>('')

const handleViewKey = async () => {
  if (apiKeyVisible.value) {
    apiKeyVisible.value = false
    return;
  }
  if (pre.value.port === undefined) {
    await MessagePlugin.warning('请先设置端口号')
    return;
  }
  const response = await fetch<string>(`http://localhost:${pre.value.port}/local/token/view`, {
    method: 'GET',
    timeout: 30,
    responseType: ResponseType.Text,
  });
  if (response.data === '') {
    apiKey.value = '暂无密钥'
  } else {
    apiKey.value = response.data
  }
  apiKeyVisible.value = true
}

const changeApiKey = async () => {
  if (newApiKey.value === '') {
    await MessagePlugin.warning('密钥不能为空')
    return;
  }
  const response = await fetch<{success: boolean}>(`http://localhost:${pre.value.port}/local/token/rest`, {
    method: 'POST',
    timeout: 30,
    headers: {
      'Content-Type': 'application/json'
    },
    body: Body.json({
      token: newApiKey.value
        }),
  });
  if (response.data.success) {
    await MessagePlugin.success('密钥修改成功')
    apiKey.value = newApiKey.value
    apiKeyIsEditing.value = false
  } else {
    await MessagePlugin.error(`密钥修改失败, 错误码: ${response.status}`)
  }
}

const genKey = () => {
  newApiKey.value = randomString(16)
}
</script>

<template>
  <div class="settings-container">
    <div class="content-wrapper">
      <div class="content__left"><p>运行状态:</p></div>
      <div class="content__right">
        <div class="content__right--status">
          <t-tag v-if="isRunning" theme="success">
            <CheckCircleIcon/>
            正在运行
          </t-tag>
          <t-tag v-else theme="warning">
            <ErrorCircleIcon/>
            <span>停止运行</span>
          </t-tag>
          <t-button
              shape="circle"
              variant="text"
              @click="refreshStatus"
              :class="isRefresh ? 'refresh-btn' : ''"
          >
            <RefreshIcon
            />
          </t-button>
        </div>
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
          >编辑
          </t-button
          >
        </div>
        <div v-else class="content__right--port">
          <t-input-number
              v-model="port"
              autofocus
              min="0"
              max="65535"
              theme="normal"
              size="small"
              :style="{ width: '60px' }"
          />
          <t-button
              size="small"
              theme="primary"
              variant="text"
              @click="changePort"
          >确定
          </t-button
          >
          <t-button
              size="small"
              theme="primary"
              variant="text"
              @click="changePortCancel"
          >取消
          </t-button
          >
        </div>
      </div>
      <div v-if="portError.isErr" class="content__left"></div>
      <div v-if="portError.isErr" class="content__right">
        <span class="content__right--error">{{ portError.msg }}</span>
      </div>
      <div class="content__left"><p>访问密钥:</p></div>
      <div class="content__right">
        <div v-if="!apiKeyIsEditing" class="content__right--port">
          <div v-if="apiKeyVisible">{{apiKey}}</div>
          <t-button
              size="small"
              theme="primary"
              variant="text"
              class="p-0"
              @click="handleViewKey"
          >
            <PhEyeSlash v-if="!apiKeyVisible" :size="20" />
            <PhEye v-if="apiKeyVisible" :size="20" />
          </t-button
          >
        </div>
        <div v-else class="content__right--newApiKey">
          <t-input
              v-model="newApiKey"
              autofocus
              theme="normal"
              size="small"
          />
        </div>
      </div>
      <div v-if="apiKeyVisible" class="content__left"></div>
      <div v-if="apiKeyVisible" class="content__right">
        <t-button
            v-if="!apiKeyIsEditing"
            size="small"
            theme="primary"
            variant="text"
            @click="apiKeyIsEditing = !apiKeyIsEditing"
        >编辑
        </t-button
        >
        <t-button
            v-if="apiKeyIsEditing"
            size="small"
            theme="primary"
            variant="text"
            @click="changeApiKey"
        >确定
        </t-button
        >
        <t-button
            v-if="apiKeyIsEditing"
            size="small"
            theme="primary"
            variant="text"
            @click="() => apiKeyIsEditing = false"
        >取消
        </t-button
        >
        <t-button
            v-if="apiKeyIsEditing"
            size="small"
            theme="primary"
            variant="text"
            @click="genKey"
        >随机生成
        </t-button
        >
      </div>
      <div class="content__left"><p>如何使用:</p></div>
      <div class="content__right">
        <t-link theme="primary" hover="color"
                href="https://docs.serverbee.app" target="_blank"
        >
          使用教程
          <jump-icon slot="suffixIcon"/>
        </t-link>
      </div>
      <div class="content__left"><p>运行日志:</p></div>
      <div class="content__right">
        <t-button
            theme="default"
            variant="outline"
            size="small"
            @click="openLog"
        >打开日志
        </t-button
        >
      </div>
      <div class="content__left"><p>更新:</p></div>
      <div class="content__right">
        <div class="content__right--update">
          <span>{{ appVersion }}</span>
          <t-button
              theme="default"
              variant="outline"
              size="small"
              @click="checkUpdate"
          >检查更新
          </t-button
          >
        </div>
      </div>
      <div class="content__left"><p>退出:</p></div>
      <div class="content__right">
        <t-button
            theme="default"
            variant="outline"
            size="small"
            @click="exitProcess"
        >退出 ServerMilk
        </t-button
        >
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.settings-container {
  @apply flex flex-col items-center justify-center h-full w-full;

  .content-wrapper {
    @apply w-full grid grid-cols-3 gap-4 gap-y-1 justify-center items-center;

    .content__left {
      @apply justify-self-end;
    }

    .content__right {
      @apply col-span-2;

      .content__right--status {
        @apply flex flex-row items-center justify-start;

        .refresh-btn {
          animation-duration: 1.5s;
          animation-name: rotatefresh;
          animation-iteration-count: infinite;
        }
      }

      .content__right--port {
        @apply flex items-center;
      }

      .content__right--newApiKey {
        @apply flex items-center;
      }

      .content__right--error {
        @apply text-red-500 text-sm;
      }

      .content__right--update {
        @apply flex items-center space-x-2;
      }
    }
  }
}

@keyframes rotatefresh {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
