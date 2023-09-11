import { getAll, set as settingsSet } from 'tauri-settings'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'

export type Preference = {
    theme?: 'dark' | 'light'
    port?: number
    isEnableAutoLaunch?: boolean
}

export default async function usePreference() {
    let preference = ref<Preference>({
        port: 9527,
        isEnableAutoLaunch: true
    })
    const { settings } = await getAll<Preference>()
    preference.value = {
        ...preference.value,
        ...(settings as Preference)
    }

    // give default value
    if (!settings.port) {
        await settingsSet('port', preference.value.port)
    }
    if (settings.isEnableAutoLaunch === undefined) {
        await setEnableAutoLaunch(preference.value.isEnableAutoLaunch!)
    }

    async function checkStatus() {
        const isEnableAutoLaunch = await invoke('is_enable_auto_launch')
        if (isEnableAutoLaunch !== preference.value.isEnableAutoLaunch) {
            preference.value.isEnableAutoLaunch
                ? await invoke('enable_auto_launch')
                : await invoke('disable_auto_launch')
        }
    }

    async function set<K extends keyof Preference>(
        key: K,
        value: Preference[K]
    ) {
        await settingsSet(key, value)
        preference.value[key] = value
    }

    async function setEnableAutoLaunch(enable: boolean) {
        await settingsSet('isEnableAutoLaunch', enable)
        preference.value.isEnableAutoLaunch = enable
        await checkStatus()
    }

    await checkStatus()

    return [preference, set, setEnableAutoLaunch] as const
}
