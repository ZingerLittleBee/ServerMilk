import { openLogInvoke } from '@/command.ts'

import { Button } from '@/components/ui/button.tsx'
import { Label } from '@/components/ui/label.tsx'

export default function LogsWidget() {
    const openLog = openLogInvoke
    return (
        <div className="flex items-center justify-between space-x-2">
            <Label className="flex flex-col space-y-1">
                <span>Logs</span>
                <span className="font-normal leading-snug text-muted-foreground">
                    View the application logs.
                </span>
            </Label>
            <Button variant="outline" size="sm" onClick={() => openLog()}>
                Open
            </Button>
        </div>
    )
}
