import { getAll, set as settingsSet } from 'tauri-settings'
import { ref } from 'vue'

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
	if (!settings.isEnableAutoLaunch) {
		await settingsSet(
			'isEnableAutoLaunch',
			preference.value.isEnableAutoLaunch
		)
	}

	async function set<K extends keyof Preference>(
		key: K,
		value: Preference[K]
	) {
		await settingsSet(key, value)
		preference.value[key] = value
	}

	return [preference, set] as const
}
