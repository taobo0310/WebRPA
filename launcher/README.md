# WebRPA 启动器

现代化的 WebRPA 桌面启动器，使用 Tauri + Vue 3 开发。

## 功能特性

- 🚀 一键启动前后端服务
- 📊 实时查看服务日志
- 🔄 自动检查版本更新
- 🎨 现代化UI设计
- ⚡ 轻量级桌面应用

## 开发环境要求

- Node.js 16+
- Rust 1.70+
- 项目内置的 Node.js 和 Python 环境

## 安装依赖

```bash
cd launcher
npm install
```

## 开发模式

```bash
npm run tauri:dev
```

## 构建应用

```bash
npm run tauri:build
```

构建完成后，可执行文件位于 `src-tauri/target/release/` 目录。

## 项目结构

```
launcher/
├── src/                    # Vue 前端源码
│   ├── App.vue            # 主组件
│   ├── main.js            # 入口文件
│   └── style.css          # 全局样式
├── src-tauri/             # Tauri 后端
│   ├── src/
│   │   └── main.rs        # Rust 主程序
│   ├── icons/             # 应用图标
│   ├── Cargo.toml         # Rust 依赖配置
│   └── tauri.conf.json    # Tauri 配置
├── package.json           # Node.js 依赖
└── vite.config.js         # Vite 配置
```

## 使用说明

1. 双击运行 `WebRPA启动器.exe`
2. 点击"启动服务"按钮启动前后端
3. 等待服务启动完成后自动打开浏览器
4. 可在日志区查看实时运行日志
5. 点击"检查更新"可检查新版本

## 注意事项

- 启动器需要放在 WebRPA 项目根目录
- 确保 Python313 和 nodejs 文件夹存在
- 首次启动可能需要较长时间
