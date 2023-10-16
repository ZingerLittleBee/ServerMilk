import { useState } from 'react'
import AutoStartWidget from '@/panel/auto-start.tsx'
import LogsWidget from '@/panel/logs.tsx'
import PortWidget from '@/panel/port.tsx'
import StatusWidget from '@/panel/status.tsx'
import TokenWidget from '@/panel/token.tsx'
import { RotateCw } from 'lucide-react'

import { cn } from '@/lib/utils.ts'
import { Button } from '@/components/ui/button'
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card'
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from '@/components/ui/tooltip.tsx'

export default function ControlPanel() {
    const [signal, setSignal] = useState(0)
    const [isSpin, setIsSpin] = useState(false)

    return (
        <Card className="border-0 shadow-none">
            <CardHeader>
                <CardTitle className="cursor-default flex items-center justify-between">
                    <p> Control panel</p>
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger asChild>
                                <Button size="icon" className="h-8 w-8">
                                    <RotateCw
                                        size="20"
                                        className={cn(isSpin && 'animate-spin')}
                                        onClick={async () => {
                                            setIsSpin(true)
                                            setSignal(signal + 1)
                                            setTimeout(
                                                () => setIsSpin(false),
                                                1000
                                            )
                                        }}
                                    />
                                </Button>
                            </TooltipTrigger>
                            <TooltipContent>
                                <p>Refresh</p>
                            </TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                </CardTitle>
                <CardDescription className="cursor-default">
                    Manage ServerBee Service.
                </CardDescription>
            </CardHeader>
            <CardContent className="grid gap-6">
                <StatusWidget />
                <AutoStartWidget signal={signal} />
                <PortWidget signal={signal} />
                <TokenWidget signal={signal} />
                <LogsWidget />
            </CardContent>
        </Card>
    )
}
