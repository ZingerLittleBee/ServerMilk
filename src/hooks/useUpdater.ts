import { ask } from '@tauri-apps/api/dialog'
import { relaunch } from '@tauri-apps/api/process'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { MessagePlugin } from 'tdesign-vue-next'

export default async function useUpdater(needMsg = false) {
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
        } else {
            if (needMsg) {
                await MessagePlugin.info('已是最新版本', 1000)
            }
        }
    } catch (error) {
        await MessagePlugin.error('检查更新失败', 1000)
    }
}
