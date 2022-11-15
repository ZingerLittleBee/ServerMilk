import { ask } from '@tauri-apps/api/dialog'
import { relaunch } from '@tauri-apps/api/process'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'

export default async function useUpdater() {
	try {
		const { shouldUpdate, manifest } = await checkUpdate()

		if (shouldUpdate) {
			const isUpdate = await ask('是否更新?', {
				title: `发现新版本: ${manifest?.version}`,
				type: 'info'
			})
			if (isUpdate) {
				// display dialog
				await installUpdate()
				// install complete, restart app
				await relaunch()
			}
		}
	} catch (error) {
		console.error(error)
	}
}
