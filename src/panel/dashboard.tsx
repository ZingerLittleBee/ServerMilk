import { openDashboardInvoke } from '@/command.ts'

import { Button } from '@/components/ui/button.tsx'
import { Label } from '@/components/ui/label.tsx'

export default function DashboardWidget() {
    const openDashboard = openDashboardInvoke
    return (
        <div className="flex items-center justify-between space-x-2">
            <Label className="flex flex-col space-y-1">
                <span>Dashboard</span>
                <span className="font-normal leading-snug text-muted-foreground">
                    Open the dashboard.
                </span>
            </Label>
            <Button size="sm" onClick={() => openDashboard()}>
                Open
            </Button>
        </div>
    )
}
