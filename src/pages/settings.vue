<script setup lang="ts">
import usePreference from "../hooks/usePreference";
import {ref} from "vue";

const [pre, setPre] = usePreference()

const handleThemeChange = () => {
  // document.documentElement.setAttribute('theme-mode', 'dark');
  setPre('theme', pre.value.theme === 'dark' ? 'light' : 'dark')
}
const port = ref(8080)

const isEditing = ref(false)


const handleChangePort = () => {
  if (port.value >= 0 && port.value <= 65535) {
    isEditing.value = false
  }
}
</script>

<template>
  <div class="settings-container">
    <div class="content-wrapper">
      <div class="content__left"><p>启动:</p></div>
      <div class="content__right">
        <t-checkbox><p>登录时启动</p></t-checkbox>
      </div>
      <div class="content__left"><p>端口号:</p></div>
      <div class="content__right ">
        <div v-if="!isEditing" class="content__right--port">
          <p>{{ port }}</p>
          <t-button size="small" theme="primary" variant="text" @click="isEditing = !isEditing">编辑</t-button>
        </div>
        <div v-else class="content__right--port">
          <t-input-number v-model="port" autoWidth autofocus clearable min="0" max="65535" theme="normal" size="small"/>
          <t-button size="small" theme="primary" variant="text" @click="handleChangePort">确定</t-button>
          <t-button size="small" theme="primary" variant="text" @click="isEditing = false">取消</t-button>
        </div>
      </div>
      <div class="content__left"><p>更新:</p></div>
      <div class="content__right">
        <t-button size="small">检查更新</t-button>
      </div>
      <div class="content__left"><p>退出:</p></div>
      <div class="content__right">
        <t-button size="small">退出 ServerBee</t-button>
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
