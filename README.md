# FileScan - 文件扫描工具

一个用Rust编写的并行文件扫描工具，可以快速扫描指定目录中特定后缀的文件。

## 功能特性

- 🚀 并行处理，高性能扫描
- 📁 递归扫描目录
- 🎯 支持多种文件后缀过滤
- 💻 跨平台支持（Windows、Linux、macOS）
- 📊 显示文件大小和类型信息

## 安装

### 从源码编译

```bash
git clone https://github.com/yourusername/filescan.git
cd filescan
cargo build --release
```

### 下载预编译版本

从 [Releases](https://github.com/yourusername/filescan/releases) 页面下载对应平台的预编译版本。

## 使用方法

```bash
filescan <目录路径> [后缀1,后缀2,...]
```

### 参数说明

- `目录路径`: 要扫描的根目录
- `后缀列表`: 可选，用逗号分隔的文件后缀（不区分大小写）

### 示例

```bash
# 扫描当前目录下的 .property 和 .ini 文件
./filescan .

# 扫描指定目录下的 .txt 和 .log 文件
./filescan /path/to/directory txt,log

# 扫描指定目录下的 .json 文件
./filescan /Users/abc/data json
```

## 输出格式

程序会输出每个匹配文件的以下信息：
- 文件路径
- 文件大小（字节）
- 文件类型（后缀）

示例输出：
```
文件: "/Users/abc/config.properties", 大小: 1024 字节, 类型: property
文件: "/Users/abc/settings.ini", 大小: 512 字节, 类型: ini
```

## 构建

### 本地构建

```bash
cargo build --release
```

### 交叉编译

```bash
# Windows x86
cargo build --release --target i686-pc-windows-msvc

# Windows x64
cargo build --release --target x86_64-pc-windows-msvc

# Linux x64
cargo build --release --target x86_64-unknown-linux-gnu

# macOS x64
cargo build --release --target x86_64-apple-darwin
```

## 依赖

- [rayon](https://crates.io/crates/rayon) - 并行处理库

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
