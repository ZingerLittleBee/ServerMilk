import {execa} from 'execa'
import {Octokit} from "@octokit/rest"
import {SingleBar} from "cli-progress"
import colors from "ansi-colors"
import {createWriteStream, readFile, unlink} from "fs"
import {join, resolve} from 'path'
import fetch from 'node-fetch'
import JSZip from "jszip"

const getLatestReleaseAddress = async () => {
    const octokit = new Octokit();
    const fileName = await downloadFileFullName()
    let data
    try {
        ({data} = await octokit.rest.repos.getLatestRelease({
            owner: 'ZingerLittleBee',
            repo: 'server_bee-backend',
        }))
    } catch (e) {
        console.error(`获取 Github 仓库状态失败: ${e.toString()}`)
    }
    const dataAssets = data.assets
    return dataAssets.find((item) => item.name === fileName).browser_download_url
}

let targetTriple

const downloadFileFullName = async () => {
    let extension = '.zip'
    return `serverbee-web-${await getTargetTriple()}${extension}`
}

const targetFileName = async () => {
    return `serverbee-web-${await getTargetTriple()}`
}

const binDir = () => {
    return resolve('src-tauri', 'binaries')
}

const targetFilePath = async () =>
    join(binDir(), await targetFileName())


const getTargetTriple = async () => {
    if (!targetTriple) {
        const rustInfo = (await execa('rustc', ['-vV'])).stdout
        targetTriple = /host: (\S+)/g.exec(rustInfo)[1]
        if (!targetTriple) {
            console.error('Failed to determine platform target triple')
        }
    }
    return targetTriple
}

const download = async (url, filePath) => {
    const progressBar = new SingleBar({
        format: 'Downloading |' + colors.cyan('{bar}') + '| {percentage}% || {value}/{total} Chunks',
        barCompleteChar: '\u2588',
        barIncompleteChar: '\u2591',
        hideCursor: true
    });

    const fileZipName = `${filePath}.zip`

    let file = createWriteStream(fileZipName);
    fetch(url, {
        redirect: 'follow'
    }).then(response => {
        let receivedBytes = 0
        const totalBytes = response.headers.get('content-length');
        progressBar.start(totalBytes, receivedBytes);

        response.body.on('data', (chunk) => {
            receivedBytes += chunk.length;
            progressBar.update(receivedBytes);
        });
        response.body.pipe(file);

        file.on('finish', function () {
            file.close()
            progressBar.stop()
            console.log(`下载完成: ${fileZipName}`)
        });
    }).catch((err) => {
        console.error(`下载失败: ${err.toString()}`)
        console.error(`请手动下载: ${url}`)
        console.error(`解压到: ${filePath}`)
        console.error(`并重命名为: ${targetFileName}`)
        progressBar.stop()
        unlink(filePath, () => {
        })
    })
}

const unzip = async (url) => {
    console.log(`解压文件: ${url}`)
    const zip = new JSZip();
    readFile(url, function (err, data) {
        if (err) throw err;
        zip.loadAsync(data).then(async (zip) => {
            let file = createWriteStream(await targetFilePath(), {mode: 0o755});
            Object.keys(zip.files).forEach((fileName) => {
                zip.files[fileName].nodeStream().pipe(file)
            })
            file.on('finish', function () {
                file.close()
                console.log(`解压完成`)
            });
        });
    });
}

getLatestReleaseAddress().then(async (res) => {
    const filePath = await targetFilePath()
    await download(res, filePath)
    const fileZipName = `${filePath}.zip`
    await unzip(fileZipName)
})
