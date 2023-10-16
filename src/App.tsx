import { useEffect } from 'react'
import ControlPanel from '@/panel/page.tsx'

export default function App() {
    useEffect(() => {
        const handleSchemeChange = (event: { matches: boolean }) => {
            if (event.matches) {
                document.documentElement.classList.add('dark')
            } else {
                document.documentElement.classList.remove('dark')
            }
        }

        const matcher = window.matchMedia('(prefers-color-scheme: dark)')
        matcher.addEventListener('change', handleSchemeChange)

        return () => matcher.removeEventListener('change', handleSchemeChange)
    }, [])

    return <ControlPanel />
}
