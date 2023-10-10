import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch.tsx'

import { Badge } from './components/ui/badge'

export default function App() {
    return (
        <Card className="border-0 shadow-none">
            <CardHeader>
                <CardTitle className="cursor-default">Settings</CardTitle>
                <CardDescription className="cursor-default">
                    Manage your settings here.
                </CardDescription>
            </CardHeader>
            <CardContent className="grid gap-6">
                <div className="flex items-center justify-between space-x-2">
                    <Label
                        htmlFor="performance"
                        className="flex flex-col space-y-1"
                    >
                        <span>Running Status</span>
                        <span className="font-normal leading-snug text-muted-foreground">
                            The application status.
                        </span>
                    </Label>
                    <Badge>Badge</Badge>
                </div>
                <div className="flex items-center justify-between space-x-2">
                    <Label
                        htmlFor="necessary"
                        className="flex flex-col space-y-1"
                    >
                        <span>Auto Start</span>
                        <span className="font-normal leading-snug text-muted-foreground">
                            Automatically start when system startup.
                        </span>
                    </Label>
                    <Switch id="necessary" defaultChecked />
                </div>
                <div className="flex items-center justify-between space-x-2">
                    <Label
                        htmlFor="functional"
                        className="flex flex-col space-y-1"
                    >
                        <span>Port</span>
                        <span className="font-normal leading-snug text-muted-foreground">
                            The port to use for the application.
                        </span>
                    </Label>
                    <Label className="p-2">9527</Label>
                </div>
            </CardContent>
        </Card>
    )
}
