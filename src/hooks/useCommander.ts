import {Child, Command} from "@tauri-apps/api/shell";
import {invoke} from "@tauri-apps/api";

export default async function useCommander() {

    let process: Child | null

    const logPath = await invoke<string>('get_log_path')

    const stop = () => {
        process?.kill()
    }

    const start = async (port: number) => {
        const command = Command.sidecar('binaries/serverbee-web', ['-p', port.toString(), '-l', logPath])
        process = await command.spawn()
    }

    const restart = async (port: number) => {
        await stop()
        await start(port)
    }

    return {
        start,
        restart,
        stop
    }
}
