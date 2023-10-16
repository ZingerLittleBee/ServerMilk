import { useEffect, useState } from 'react'
import { fetchTokenInvoke, setTokenInvoke } from '@/command.ts'
import { TokenForm } from '@/panel/token-form.tsx'
import { Check, Copy, Pencil } from 'lucide-react'

import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/ui/dialog.tsx'
import { Label } from '@/components/ui/label.tsx'
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from '@/components/ui/tooltip.tsx'

export default function TokenWidget({ signal }: { signal: number }) {
    const [isCopy, setIsCopy] = useState(false)
    const [token, setToken] = useState<string | undefined>(undefined)
    const [isOpen, setIsOpen] = useState(false)

    const getToken = async () => setToken(await fetchTokenInvoke())

    const onNewToken = async (token: string) => {
        await setTokenInvoke(token)
        await getToken()
        setIsOpen(false)
    }

    useEffect(() => {
        getToken()
    }, [signal])

    return (
        <div className="flex items-center justify-between space-x-2">
            <Label htmlFor="token" className="flex flex-col space-y-1">
                <div className="flex items-center space-x-2">
                    <span>Token</span>
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
                    <Dialog open={isOpen} onOpenChange={setIsOpen}>
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
                            <TokenForm token={token} onNewToken={onNewToken} />
                        </DialogContent>
                    </Dialog>
                </div>
                <span className="font-normal leading-snug text-muted-foreground">
                    Credentials for connection.
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
    )
}
