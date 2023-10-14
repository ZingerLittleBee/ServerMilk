import { isFreePortInvoke } from '@/command.ts'
import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import * as z from 'zod'

import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@/components/ui/form.tsx'
import { Input } from '@/components/ui/input.tsx'

import { Button } from './components/ui/button'

const formSchema = z.object({
    port: z.coerce.number().int().min(0).max(65535),
})

export function PortForm({
    port,
    onNewPort,
}: {
    port?: number
    onNewPort: (port: number) => void
}) {
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            port: port,
        },
    })

    async function onSubmit(values: z.infer<typeof formSchema>) {
        const res = await isFreePortInvoke(values.port)
        if (res) {
            onNewPort(values.port)
        } else {
            form.setError('port', {
                type: 'manual',
                message: 'Port is not free',
            })
        }
    }

    return (
        <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
                <FormField
                    control={form.control}
                    name="port"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Port</FormLabel>
                            <FormControl>
                                <Input {...field} className="h-8" />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button type="submit" className="w-full">
                    Submit
                </Button>
            </form>
        </Form>
    )
}
