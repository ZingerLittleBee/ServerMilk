import AutoStartWidget from '@/panel/auto-start.tsx'
import LogsWidget from '@/panel/logs.tsx'
import PortWidget from '@/panel/port.tsx'
import StatusWidget from '@/panel/status.tsx'
import TokenWidget from '@/panel/token.tsx'

import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card'

export default function ControlPanel() {
    return (
        <Card className="border-0 shadow-none">
            <CardHeader>
                <CardTitle className="cursor-default">Control panel</CardTitle>
                <CardDescription className="cursor-default">
                    Manage your settings here.
                </CardDescription>
            </CardHeader>
            <CardContent className="grid gap-6">
                <StatusWidget />
                <AutoStartWidget />
                <PortWidget />
                <TokenWidget />
                <LogsWidget />
            </CardContent>
        </Card>
    )
}
