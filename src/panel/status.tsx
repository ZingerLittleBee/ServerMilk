import { useEffect, useState } from 'react'
import { getPidInvoke } from '@/command.ts'
import { invoke } from '@tauri-apps/api'

import { Badge } from '@/components/ui/badge.tsx'
import { Label } from '@/components/ui/label.tsx'

export default function StatusWidget() {
    const [pid, setPid] = useState<number | null>(null)

    const [isRunning, setIsRunning] = useState(false)

    const getPid = async () => setPid(await getPidInvoke())

    const checkRunningStatus = async () => {
        const res = await invoke<boolean>('check_running_status')
        setIsRunning(res)
    }

    const refreshStatus = async () => {
        checkRunningStatus()
        getPid()
    }

    useEffect(() => {
        const timer = setInterval(() => {
            refreshStatus()
        }, 2000)

        return () => {
            clearTimeout(timer)
        }
    }, [])

    return (
        <div className="flex items-center justify-between space-x-2">
            <Label htmlFor="running status" className="flex flex-col space-y-1">
                <span>Running Status</span>
                <span className="font-normal leading-snug text-muted-foreground">
                    The application status.
                </span>
            </Label>

            <div className="space-x-2">
                <Badge variant="outline">PID: {pid ? pid : 'N/A'}</Badge>
                {isRunning ? (
                    <Badge variant="default">Running</Badge>
                ) : (
                    <Badge variant="destructive">Stopped</Badge>
                )}
            </div>
        </div>
    )
}
