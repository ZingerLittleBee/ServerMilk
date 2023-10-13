import { invoke } from '@tauri-apps/api'

export const openLogInvoke = async () => invoke('open_log')

export const enableAutoStartInvoke = async () => invoke('enable_auto_start')
export const disableAutoStartInvoke = async () => invoke('disable_auto_start')

export const getPortInvoke = async () => invoke<number>('get_port')

export const getPidInvoke = async () => invoke<number>('get_pid')

export const isFreePortInvoke = async (port: number) =>
    invoke<boolean>('is_free_port', { port })
