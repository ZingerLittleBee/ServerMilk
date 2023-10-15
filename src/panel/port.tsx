import { useEffect, useState } from 'react'
import { getPortInvoke, startWithNewPortInvoke } from '@/command.ts'
import { PortForm } from '@/panel/port-form.tsx'
import { Pencil } from 'lucide-react'

import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/ui/dialog.tsx'
import { Label } from '@/components/ui/label.tsx'

export default function PortWidget({ signal }: { signal: number }) {
    const [port, setPort] = useState<number | null>(null)
    const [isOpen, setIsOpen] = useState(false)
    const getPort = async () => setPort(await getPortInvoke())

    const startWithNewPort = startWithNewPortInvoke

    useEffect(() => {
        getPort()
    }, [signal])

    return (
        <div className="flex items-center justify-between space-x-2">
            <Label htmlFor="port" className="flex flex-col space-y-1">
                <div className="flex items-center space-x-2">
                    <span>Port</span>
                    <Dialog open={isOpen} onOpenChange={setIsOpen}>
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
                                    setIsOpen(false)
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
    )
}
