Language : 🇺🇸 English | [🇨🇳 简体中文](./README.zh-CN.md)

<div align="center">
    <img src="./app-icon.png" alt="ServerMilk" width="128"/>
    <h1>ServerMilk</h1>
</div>

<div align="center">

A desktop wrapper power by [tauri](https://github.com/tauri-apps/tauri) for [server_bee-backend](https://github.com/ZingerLittleBee/server_bee-backend)

Backend for iOS application named [ServerBee](https://apps.apple.com/us/app/serverbee/id6443553714)

![GitHub release (latest by date)](https://img.shields.io/github/v/release/ZingerLittleBee/ServerMilk?style=for-the-badge)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ZingerLittleBee/ServerMilk/release.yml?style=for-the-badge)
![GitHub last commit](https://img.shields.io/github/last-commit/ZingerLittleBee/ServerMilk?style=for-the-badge)

</div>

## Snapshot

![control-panel](./snapshot/desktop-control-panel.png)
![overview](./snapshot/desktop-overview.png)
![process](./snapshot/desktop-process.png)
![disk&network](./snapshot/desktop-disk&network.png)
![terminal](./snapshot/desktop-terminal.png)
![settings](./snapshot/desktop-settings.png)

## How to compile

### Prerequisites

[Please refer to the tauri guide](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Quick Start

```bash
$ git clone --recursive https://github.com/ZingerLittleBee/ServerMilk.git
$ cd ServerMilk
$ git submodule update --remote
$ pnpm i
$ pnpm tauri build
```

and then, find release in `src-tauri/target/release/bundle`

## Found an issue or have a proposal

[Create an issue](https://github.com/ZingerLittleBee/ServerMilk/issues)

## Release Notes

SEE [CHANGELOG](./CHANGELOG.md)
