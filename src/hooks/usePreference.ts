import {getAll, set as settingsSet} from 'tauri-settings'
import {onMounted, ref} from 'vue'

type Preference = {
    theme?: 'dark' | 'light'
    port?: number
    isEnableAutoLaunch?: boolean
    startFullscreen?: boolean
}

export default function usePreference() {
    let preference = ref<Preference>({})

    onMounted(async () => {
        const {settings} = await getAll()
        preference.value = {
            ...preference.value,
            ...settings as Preference,
        }
    })

    async function set<K extends keyof Preference>(key: K, value: Preference[K]) {
        settingsSet(key, value)
        preference.value[key] = value
    }

    return [preference, set] as const
}
