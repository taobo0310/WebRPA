# 安装 Rust 环境指南

WebRPA 启动器使用 Tauri 框架开发,需要 Rust 编译环境。

## Windows 安装步骤

### 1. 安装 Rust

**方法一: 使用 rustup-init.exe (推荐)**

1. 访问 https://www.rust-lang.org/tools/install
2. 点击下载 `rustup-init.exe`
3. 运行安装程序
4. 选择 `1) Proceed with installation (default)`
5. 等待安装完成(约5-10分钟)
6. 重启命令提示符或PowerShell

**方法二: 使用 winget**

```powershell
winget install Rustlang.Rustup
```

### 2. 验证安装

打开新的命令提示符或PowerShell,运行:

```bash
rustc --version
cargo --version
```

如果显示版本号,说明安装成功。

### 3. 安装 Visual Studio C++ 构建工具

Rust 在 Windows 上需要 MSVC 工具链。

**方法一: 安装 Visual Studio Build Tools**

1. 访问 https://visualstudio.microsoft.com/zh-hans/downloads/
2. 下载 "Visual Studio 2022 生成工具"
3. 运行安装程序
4. 选择 "使用 C++ 的桌面开发"
5. 点击安装(约2-3GB)

**方法二: 安装完整的 Visual Studio Community**

1. 访问 https://visualstudio.microsoft.com/zh-hans/downloads/
2. 下载 "Visual Studio 2022 Community"
3. 安装时选择 "使用 C++ 的桌面开发"

### 4. 配置 Rust 镜像源 (可选,加速下载)

创建或编辑文件 `C:\Users\你的用户名\.cargo\config.toml`:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

## 构建启动器

安装完成后,运行:

```bash
cd launcher
双击 "构建启动器.bat"
```

或手动运行:

```bash
cd launcher
..\nodejs\npm.cmd run tauri:build
```

## 首次构建说明

首次构建会:
1. 下载 Rust 依赖包(约500MB)
2. 编译所有依赖(约10-20分钟)
3. 编译启动器本身(约2-3分钟)

**总耗时: 15-30分钟**

后续构建会快很多(约1-2分钟)。

## 常见问题

### Q: 提示 "rustc" 不是内部或外部命令

A: Rust 未正确安装或环境变量未生效,请:
1. 重新安装 Rust
2. 重启命令提示符/PowerShell
3. 重启电脑

### Q: 提示找不到 MSVC

A: 需要安装 Visual Studio C++ 构建工具,见上方步骤3。

### Q: 下载依赖很慢

A: 配置 Rust 镜像源,见上方步骤4。

### Q: 编译失败

A: 检查:
1. Rust 版本是否 >= 1.70
2. MSVC 是否正确安装
3. 查看错误信息,搜索解决方案

### Q: 磁盘空间不足

A: Rust 工具链和依赖需要约5-10GB空间,请确保有足够空间。

## 验证环境

运行以下命令验证环境是否正确:

```bash
# 检查 Rust
rustc --version
cargo --version

# 检查 MSVC (应该不报错)
cargo build --help

# 检查 Node.js
node --version
npm --version
```

全部正常后即可构建启动器。

## 开发模式

如果只是想测试修改,可以使用开发模式:

```bash
cd launcher
双击 "开发模式.bat"
```

开发模式支持热重载,修改代码后自动刷新。

## 获取帮助

- Rust 官方文档: https://www.rust-lang.org/zh-CN/
- Tauri 官方文档: https://tauri.app/zh-cn/
- Rust 中文社区: https://rust.cc/

## 替代方案

如果不想安装 Rust,可以:

1. 继续使用原有的 `双击启动WebRPA本地服务.bat`
2. 等待预编译的启动器发布版本
3. 请其他人帮忙构建

---

**提示**: 安装 Rust 是一次性的工作,安装后可以用于所有 Rust 项目的开发。
