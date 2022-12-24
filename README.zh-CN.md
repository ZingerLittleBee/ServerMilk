Language : [🇺🇸 English](./README.md) | 🇨🇳 简体中文

<div align="center">
    <img src="./app-icon.png" alt="ServerMilk" width="128"/>
    <h1>ServerMilk</h1>
</div>

<div align="center">

[server_bee-backend](https://github.com/ZingerLittleBee/server_bee-backend) 的桌面客户端

iOS 应用 [ServerBee](https://apps.apple.com/us/app/serverbee/id6443553714) 的后端

![GitHub release (latest by date)](https://img.shields.io/github/v/release/ZingerLittleBee/ServerMilk?style=for-the-badge)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ZingerLittleBee/ServerMilk/release.yml?style=for-the-badge)
![GitHub last commit](https://img.shields.io/github/last-commit/ZingerLittleBee/ServerMilk?style=for-the-badge)

</div>

## 截图

<img src="./snapshot/ServerMilk.png" alt="ServerMilk" width="400"/>

## 如何编译

### 编译环境

[请参考 tauri 指南](https://tauri.app/zh-cn/v1/guides/getting-started/prerequisites)

### 开始编译

```bash
$ git clone https://github.com/ZingerLittleBee/ServerMilk.git
$ cd ServerMilk
$ pnpm i
$ pnpm tauri build
```

在 `src-tauri/target/release/bundle` 可以找到对应的安装包

## 发现问题或有建议

[Create an issue](https://github.com/ZingerLittleBee/ServerMilk/issues)

## 发行说明

SEE [CHANGELOG](./CHANGELOG.md)
