import { useState } from 'react'
import { enableAutoStartInvoke } from '@/command.ts'

import { useSettings } from '@/hooks/useSettings.ts'
import { Label } from '@/components/ui/label.tsx'
import { Switch } from '@/components/ui/switch.tsx'

export default function AutoStartWidget() {
    const { settings, setIsAutoStart } = useSettings()
    const [checked, setChecked] = useState(settings.isAutoStart)

    const enableAutoStart = enableAutoStartInvoke
    const disableAutoStart = enableAutoStartInvoke

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
                    setIsAutoStart(checked)
                }}
            />
        </div>
    )
}
