# 构建说明

## 前置要求

### 1. 安装 Rust

```bash
# 使用 rustup 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 或访问 https://rustup.rs/
```

### 2. 安装 OpenCV

#### Windows (使用 vcpkg)

```powershell
# 克隆 vcpkg
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat

# 安装 OpenCV
.\vcpkg install opencv4:x64-windows

# 设置环境变量
$env:OPENCV_DIR = "C:\path\to\vcpkg\installed\x64-windows"
$env:OPENCV_STATIC = "1"
```

#### Windows (使用预编译版本)

1. 从 [OpenCV 官网](https://opencv.org/releases/) 下载预编译版本
2. 解压到 `C:\opencv`
3. 设置环境变量：
   ```powershell
   $env:OPENCV_DIR = "C:\opencv\build"
   ```

### 3. 设置环境变量

确保以下环境变量已设置：

```powershell
# OpenCV 路径
$env:OPENCV_DIR = "C:\path\to\opencv"

# 如果使用 vcpkg
$env:VCPKG_ROOT = "C:\path\to\vcpkg"
```

## 构建

### 开发模式

```bash
cargo build
```

### 发布模式

```bash
cargo build --release
```

### 运行

```bash
# 开发模式
cargo run

# 发布模式
cargo run --release
```

## 常见问题

### OpenCV 找不到

如果遇到 OpenCV 找不到的错误：

1. 检查 `OPENCV_DIR` 环境变量是否正确设置
2. 确保 OpenCV 库文件在系统路径中
3. 尝试重新安装 OpenCV

### 编译错误

如果遇到编译错误：

1. 确保 Rust 版本 >= 1.70
2. 运行 `cargo clean` 清理构建缓存
3. 检查所有依赖是否正确安装

## 交叉编译

目前仅支持 Windows x64 平台。如需支持其他平台，需要：

1. 安装对应平台的 OpenCV
2. 配置交叉编译工具链
3. 更新 `Cargo.toml` 中的平台特定依赖

