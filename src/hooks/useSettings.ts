export type SettingsSchema = {
    port?: number
    isAutoStart?: boolean
}

const defaultSettings: SettingsSchema = {
    port: 9527,
    isAutoStart: true,
}

export const useSettings = (): {
    settings: SettingsSchema
    setPort: (port: number) => void
    setIsAutoStart: (isAutoStart: boolean) => void
} => {
    const settings = JSON.parse(localStorage.getItem('settings') ?? '{}')

    return {
        settings:
            Object.keys(settings).length === 0 ? defaultSettings : settings,
        setPort: (port: number) => {
            localStorage.setItem(
                'settings',
                JSON.stringify({ ...settings, port })
            )
        },
        setIsAutoStart: (isAutoStart: boolean) => {
            localStorage.setItem(
                'settings',
                JSON.stringify({ ...settings, isAutoStart })
            )
        },
    }
}
