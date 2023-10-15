import { useEffect, useState } from 'react'
import { fetchTokenInvoke } from '@/command.ts'
import { Check, Copy, Pencil } from 'lucide-react'

import { Button } from '@/components/ui/button.tsx'
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
import { Label } from '@/components/ui/label.tsx'
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from '@/components/ui/tooltip.tsx'

export default function TokenWidget({ signal }: { signal: number }) {
    const [isCopy, setIsCopy] = useState(false)
    const [token, setToken] = useState<string | null>(null)

    const getToken = async () => setToken(await fetchTokenInvoke())

    useEffect(() => {
        getToken()
    }, [signal])

    return (
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
                                    <Input id="token" className="col-span-3" />
                                </div>
                            </div>
                            <DialogFooter>
                                <Button type="submit">Save changes</Button>
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
    )
}
