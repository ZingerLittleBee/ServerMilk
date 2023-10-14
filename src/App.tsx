import { useEffect, useState } from 'react'
import {
    enableAutoStartInvoke,
    fetchTokenInvoke,
    getPortInvoke,
    openLogInvoke,
    startWithNewPortInvoke,
} from '@/command.ts'
import { PortForm } from '@/port-form.tsx'
import { invoke } from '@tauri-apps/api'
import { Check, Copy, Pencil, RotateCw } from 'lucide-react'

import { cn } from '@/lib/utils.ts'
import { useSettings } from '@/hooks/useSettings.ts'
import { Button } from '@/components/ui/button.tsx'
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card'
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/ui/dialog.tsx'
import { Input } from '@/components/ui/input.tsx'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch.tsx'

import { Badge } from './components/ui/badge'
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from './components/ui/tooltip'

export default function App() {
    const { settings, setIsAutoStart } = useSettings()
    const [checked, setChecked] = useState(settings.isAutoStart)
    const [pid, setPid] = useState<number | null>(null)
    const [port, setPort] = useState<number | null>(null)
    const [isRunning, setIsRunning] = useState(false)
    const [token, setToken] = useState<string | null>(null)
    const [isCopy, setIsCopy] = useState(false)
    const [portDialogIsOpen, setPortDialogIsOpen] = useState(false)
    const [isSpin, setIsSpin] = useState(false)

    const checkRunningStatus = async () => {
        const res = await invoke<boolean>('check_running_status')
        setIsRunning(res)
    }

    const getPid = async () => setPid(await getPortInvoke())

    const getPort = async () => setPort(await getPortInvoke())
    const getToken = async () => setToken(await fetchTokenInvoke())

    const startWithNewPort = startWithNewPortInvoke

    const enableAutoStart = enableAutoStartInvoke
    const disableAutoStart = enableAutoStartInvoke
    const openLog = openLogInvoke

    const refreshStatus = async () => {
        checkRunningStatus()
        getPid()
        getPort()
        getToken()
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
        <Card className="border-0 shadow-none">
            <CardHeader>
                <CardTitle className="cursor-default">Control panel</CardTitle>
                <CardDescription className="cursor-default">
                    Manage your settings here.
                </CardDescription>
            </CardHeader>
            <CardContent className="grid gap-6">
                <div className="flex items-center justify-between space-x-2">
                    <Label
                        htmlFor="running status"
                        className="flex flex-col space-y-1"
                    >
                        <div className="flex space-x-2 items-center">
                            <span>Running Status</span>

                            <RotateCw
                                size="15"
                                className={cn(
                                    'stroke-muted-foreground cursor-pointer',
                                    isSpin && 'animate-spin'
                                )}
                                onClick={async () => {
                                    setIsSpin(true)
                                    await refreshStatus()
                                    setTimeout(() => setIsSpin(false), 1000)
                                }}
                            />
                        </div>
                        <span className="font-normal leading-snug text-muted-foreground">
                            The application status.
                        </span>
                    </Label>

                    <div className="space-x-2">
                        <Badge variant="outline">
                            PID: {pid ? pid : 'N/A'}
                        </Badge>
                        {isRunning ? (
                            <Badge variant="default">Running</Badge>
                        ) : (
                            <Badge variant="destructive">Stopped</Badge>
                        )}
                    </div>
                </div>
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
                <div className="flex items-center justify-between space-x-2">
                    <Label
                        htmlFor="functional"
                        className="flex flex-col space-y-1"
                    >
                        <div className="flex items-center space-x-2">
                            <span>Port</span>
                            <Dialog
                                open={portDialogIsOpen}
                                onOpenChange={setPortDialogIsOpen}
                            >
                                <DialogTrigger asChild>
                                    <Pencil
                                        size="15"
                                        className="stroke-muted-foreground cursor-pointer"
                                    />
                                </DialogTrigger>
                                <DialogContent className="w-[325px]">
                                    <DialogHeader className="text-left">
                                        <DialogTitle>New port</DialogTitle>
                                        <DialogDescription>
                                            Click save when you're done.
                                        </DialogDescription>
                                    </DialogHeader>
                                    <PortForm
                                        port={port ? port : undefined}
                                        onNewPort={(port) => {
                                            startWithNewPort(port)
                                            setPortDialogIsOpen(false)
                                        }}
                                    />
                                </DialogContent>
                            </Dialog>
                        </div>
                        <span className="font-normal leading-snug text-muted-foreground">
                            The port to use for the application.
                        </span>
                    </Label>
                    <Label className="p-2">{port}</Label>
                </div>
                <div className="flex items-center justify-between space-x-2">
                    <Label htmlFor="token" className="flex flex-col space-y-1">
                        <div className="flex items-center space-x-2">
                            <span>Token</span>
                            <Dialog>
                                <DialogTrigger asChild>
                                    <Pencil
                                        size="15"
                                        className="stroke-muted-foreground cursor-pointer"
                                    />
                                </DialogTrigger>
                                <DialogContent className="w-[325px]">
                                    <DialogHeader className="text-left">
                                        <DialogTitle>New token</DialogTitle>
                                        <DialogDescription>
                                            Click save when you're done.
                                        </DialogDescription>
                                    </DialogHeader>
                                    <div className="grid gap-4 py-4">
                                        <div className="grid grid-cols-4 items-center gap-4">
                                            <Label
                                                htmlFor="token"
                                                className="text-center"
                                            >
                                                Token
                                            </Label>
                                            <Input
                                                id="token"
                                                className="col-span-3"
                                            />
                                        </div>
                                    </div>
                                    <DialogFooter>
                                        <Button type="submit">
                                            Save changes
                                        </Button>
                                    </DialogFooter>
                                </DialogContent>
                            </Dialog>
                            {isCopy ? (
                                <Check size="15" className="icon-button" />
                            ) : (
                                <Copy
                                    size="15"
                                    className="icon-button"
                                    onClick={async () => {
                                        setIsCopy(true)
                                        await navigator.clipboard.writeText(
                                            token as string
                                        )
                                        setTimeout(() => setIsCopy(false), 2000)
                                    }}
                                />
                            )}
                        </div>
                        <span className="font-normal leading-snug text-muted-foreground">
                            The token to use for the ServerBee.
                        </span>
                    </Label>
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger asChild>
                                <Label className="max-w-[180px] truncate p-2 text-right">
                                    {token}
                                </Label>
                            </TooltipTrigger>
                            <TooltipContent>
                                <p>{token}</p>
                            </TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                </div>
                <div className="flex items-center justify-between space-x-2">
                    <Label className="flex flex-col space-y-1">
                        <span>Logs</span>
                        <span className="font-normal leading-snug text-muted-foreground">
                            View the application logs.
                        </span>
                    </Label>
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={() => openLog()}
                    >
                        Open
                    </Button>
                </div>
            </CardContent>
        </Card>
    )
}
