import { useEffect, useState } from 'react'
import {
    disableAutoStartInvoke,
    enableAutoStartInvoke,
    isEnableAutoStartInvoke,
} from '@/command.ts'

import { Label } from '@/components/ui/label.tsx'
import { Switch } from '@/components/ui/switch.tsx'

export default function AutoStartWidget({ signal }: { signal: number }) {
    const [checked, setChecked] = useState(false)

    const isEnableAutoStart = isEnableAutoStartInvoke
    const enableAutoStart = enableAutoStartInvoke
    const disableAutoStart = disableAutoStartInvoke

    useEffect(() => {
        const checkAutoStart = async () => {
            setChecked(await isEnableAutoStart())
        }
        checkAutoStart()
    }, [signal])

    return (
        <div className="flex items-center justify-between space-x-2">
            <Label className="flex flex-col space-y-1">
                <span>Auto Start</span>
                <span className="font-normal leading-snug text-muted-foreground">
                    Automatically start when system startup.
                </span>
            </Label>
            <Switch
                checked={checked}
                onCheckedChange={(checked) => {
                    setChecked(checked)
                    checked ? enableAutoStart() : disableAutoStart()
                }}
            />
        </div>
    )
}
