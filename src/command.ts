import { invoke } from '@tauri-apps/api'

export const openDashboardInvoke = async () => invoke('open_dashboard')

export const openLogInvoke = async () => invoke('open_log')

export const isEnableAutoStartInvoke = async () =>
    invoke<boolean>('is_enable_auto_start')
export const enableAutoStartInvoke = async () => invoke('enable_auto_start')
export const disableAutoStartInvoke = async () => invoke('disable_auto_start')

export const getPortInvoke = async () => invoke<number>('get_port')

export const getPidInvoke = async () => invoke<number>('get_pid')

export const isFreePortInvoke = async (port: number) =>
    invoke<boolean>('is_free_port', { port })

export const fetchTokenInvoke = async () => invoke<string>('fetch_token')
export const setTokenInvoke = async (token: string) =>
    invoke<boolean>('set_token', { token })

export const startWithNewPortInvoke = async (port: number) =>
    invoke('start_with_new_port', {
        port,
    })
