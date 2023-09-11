import {execa} from 'execa'

import {renameSync} from "fs"
import {resolve} from 'path'


/// Download from GitHub and then unzip

// import {createWriteStream, readFile, renameSync, unlink} from "fs"
// import {join, resolve} from 'path'
// import {Octokit} from "@octokit/rest"
// import {SingleBar} from "cli-progress"
// import colors from "ansi-colors"
// import fetch from 'node-fetch'
// import JSZip from "jszip"

// const getLatestReleaseAddress = async () => {
//     const octokit = new Octokit();
//     const fileName = await downloadFileFullName()
//     let data
//     try {
//         ({data} = await octokit.rest.repos.getLatestRelease({
//             owner: 'ZingerLittleBee',
//             repo: 'server_bee-backend',
//         }))
//     } catch (e) {
//         console.error(`获取 Github 仓库状态失败: ${e.toString()}`)
//     }
//     const dataAssets = data.assets
//     return dataAssets.find((item) => item.name === fileName).browser_download_url
// }
//
// let targetTriple
//
// const downloadFileFullName = async () => {
//     let extension = '.zip'
//     return `serverbee-web-${await getTargetTriple()}${extension}`
// }
//
// const targetFileName = async () => {
//     return `serverbee-web-${await getTargetTriple()}`
// }
//
// const binDir = () => {
//     return resolve('src-tauri', 'binaries')
// }
//
// const targetFilePath = async () =>
//     join(binDir(), await targetFileName())
//
//
// const getTargetTriple = async () => {
//     if (!targetTriple) {
//         const rustInfo = (await execa('rustc', ['-vV'])).stdout
//         targetTriple = /host: (\S+)/g.exec(rustInfo)[1]
//         if (!targetTriple) {
//             console.error('Failed to determine platform target triple')
//         }
//     }
//     return targetTriple
// }
//
// const download = async (url, filePath) => {
//     const progressBar = new SingleBar({
//         format: 'Downloading |' + colors.cyan('{bar}') + '| {percentage}% || {value}/{total} Chunks',
//         barCompleteChar: '\u2588',
//         barIncompleteChar: '\u2591',
//         hideCursor: true
//     });
//
//     const fileZipName = `${filePath}.zip`
//
//     let file = createWriteStream(fileZipName);
//     fetch(url, {
//         redirect: 'follow'
//     }).then(response => {
//         let receivedBytes = 0
//         const totalBytes = response.headers.get('content-length');
//         progressBar.start(totalBytes, receivedBytes);
//
//         response.body.on('data', (chunk) => {
//             receivedBytes += chunk.length;
//             progressBar.update(receivedBytes);
//         });
//         response.body.pipe(file);
//
//         file.on('finish', function () {
//             file.close()
//             progressBar.stop()
//             console.log(`下载完成: ${fileZipName}`)
//         });
//     }).catch((err) => {
//         console.error('文件下载出错')
//         console.error(`请手动下载: ${url}`)
//         console.error(`解压到: ${filePath}`)
//         console.error(`并重命名为: ${targetFileName}`)
//         progressBar.stop()
//         unlink(filePath, () => {
//         })
//         throw err
//     })
// }

// const unzip = async (url) => {
//     console.log(`解压文件: ${url}`)
//     const zip = new JSZip();
//     readFile(url, function (err, data) {
//         if (err) throw err;
//         zip.loadAsync(data).then(async (zip) => {
//             let file = createWriteStream(await targetFilePath(), {mode: 0o755});
//             Object.keys(zip.files).forEach((fileName) => {
//                 zip.files[fileName].nodeStream().pipe(file)
//             })
//             file.on('finish', function () {
//                 file.close()
//                 console.log(`解压完成`)
//             });
//         });
//     });
// }

// getLatestReleaseAddress().then(async (res) => {
//     const filePath = await targetFilePath()
//     await download(res, filePath)
//     const fileZipName = `${filePath}.zip`
//     await unzip(fileZipName)
// })

const getTargetTriple = async () => {
    let targetTriple
    const rustInfo = (await execa('rustc', ['-vV'])).stdout
    targetTriple = /host: (\S+)/g.exec(rustInfo)[1]
    if (!targetTriple) {
        throw new Error('Failed to determine platform target triple')
    }
    return targetTriple
}

const binariesDir = resolve('src-tauri', 'binaries')
const workDir = resolve(binariesDir, 'server_bee-backend')
const viewWorkDir = resolve(workDir, 'view')
const releasePath = resolve(workDir, 'target', 'release', 'serverbee-web')


async function main() {
    let targetTriple = await getTargetTriple()
    console.log(`当前系统三元组: ${targetTriple}`)

    let extension = ''
    if (process.platform === 'win32') {
        extension = '.exe'
    }

    console.log('构建 submodule 仓库')
    console.log('首次构建可能需要较长时间, 请耐心等待')

    console.log('当前执行命令: pnpm install')
    await execa('pnpm', ['install'], {
        cwd: viewWorkDir
    })

    console.log('当前执行命令: pnpm build')
    await execa('pnpm', ['build'], {
        cwd: viewWorkDir
    })

    console.log('当前执行命令: cargo build --release')
    await execa('cargo', ['build', '--release'], {
        cwd: workDir
    })
    const newReleasePath = `${binariesDir}/serverbee-web-${targetTriple}${extension}`
    renameSync(
        `${releasePath}${extension}`,
        newReleasePath
    )
    console.log(`submodule: ${newReleasePath} 构建完成`)
}


main().catch((e) => {
    throw e
})
