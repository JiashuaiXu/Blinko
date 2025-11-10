# 👁️ Blinko / 明眸

> **明眸善睐，坐如钟** | *Blinko watches, so you can rest.*

一个轻量、智能的 Windows 桌面健康提醒工具，专注于保护你的眼睛和坐姿健康。

**设计哲学：Keep it Simple, Keep it Stupid**

---

## 🌐 语言

[English](README.md) | [中文](README.zh-CN.md)

---

## ✨ 核心功能

- 🕐 **定时提醒**：20-20-20 护眼法则自动提醒
- 👁️ **眨眼激励**：通过摄像头检测，提醒你多眨眼
- 🪑 **坐姿检测**：实时监测坐姿，提醒保持正确姿势
- 👀 **桌面图标**：Windows 桌面顶层显示小眼睛图标，低占用
- 📊 **数据统计**：记录你的健康习惯
- 🔔 **系统托盘**：轻量常驻，不打扰但可交互
- 🔒 **隐私优先**：所有图像处理在本地完成，不上传任何数据

---

## 🛠️ 技术栈

**纯 Rust 实现** - 简单、高效、低占用

- **语言**：Rust 2021 Edition
- **窗口系统**：Windows API（原生）
- **图像处理**：OpenCV Rust（纯 Rust，无需 Python）
- **异步运行时**：Tokio
- **数据存储**：SQLite（rusqlite）
- **配置管理**：TOML + Serde

**设计原则**：
- ✅ 纯 Rust 实现，无外部依赖（除 OpenCV）
- ✅ 最小化依赖，保持简单
- ✅ 低资源占用（内存 < 50MB）
- ✅ 原生 Windows 体验

---

## 📦 项目结构

```
Blinko/
├── src/                    # Rust 源代码
│   ├── main.rs            # 主入口
│   ├── config.rs          # 配置管理
│   ├── database.rs        # 数据库操作
│   ├── detection.rs        # 眨眼/坐姿检测
│   ├── reminder.rs         # 定时提醒
│   ├── tray.rs            # 系统托盘
│   └── window.rs          # 桌面顶层窗口
├── docs/                   # 项目文档
├── Cargo.toml             # Rust 依赖配置
└── README.md              # 本文件
```

---

## 🚀 快速开始

### 环境要求

- **Rust** 1.70+（推荐使用 rustup 安装）
- **Windows** 10/11
- **OpenCV** 4.x（通过 vcpkg 或系统包管理器安装）

### 安装 OpenCV（Windows）

#### 方法 1：使用 vcpkg（推荐）

```powershell
# 安装 vcpkg
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat

# 安装 OpenCV
.\vcpkg install opencv4:x64-windows

# 设置环境变量
$env:OPENCV_DIR = "C:\path\to\vcpkg\installed\x64-windows"
```

#### 方法 2：使用预编译版本

下载 OpenCV 预编译版本，设置 `OPENCV_DIR` 环境变量指向安装目录。

### 构建和运行

```bash
# 克隆仓库
git clone <repository-url>
cd Blinko

# 开发模式运行
cargo run

# 构建发布版本
cargo build --release

# 运行发布版本
cargo run --release
```

### 首次运行

1. 程序会在系统托盘显示图标
2. 桌面顶层会显示一个小眼睛图标（64x64 像素）
3. 默认每 20 分钟提醒一次休息
4. 配置文件保存在：`%APPDATA%\blinko\config.toml`
5. 数据库保存在：`%LOCALAPPDATA%\blinko\blinko.db`

---

## ⚙️ 配置

编辑配置文件 `%APPDATA%\blinko\config.toml`：

```toml
[reminder]
enabled = true
interval_minutes = 20

[blink_detection]
enabled = false
check_interval_seconds = 5
threshold_seconds = 30

[posture_detection]
enabled = false
sensitivity = "medium"
```

---

## 🎯 功能说明

### 定时提醒

- 默认每 20 分钟提醒一次
- 遵循 20-20-20 护眼法则
- 可通过配置文件调整间隔

### 眨眼检测

- 使用摄像头实时检测眨眼频率
- 长时间未眨眼时提醒
- 所有处理在本地完成，保护隐私

### 坐姿检测

- 检测头部和肩部位置
- 发现异常坐姿时提醒
- 可调整检测灵敏度

### 桌面图标

- 在 Windows 桌面顶层显示小眼睛图标
- 64x64 像素，低占用
- 始终置顶，不遮挡重要内容

---

## 📊 性能指标

- **内存占用**：< 50MB（空闲时）
- **CPU 占用**：< 1%（空闲时），< 15%（检测时）
- **启动时间**：< 2 秒
- **检测延迟**：< 500ms

---

## 🔒 隐私保护

- ✅ 所有图像处理在本地完成
- ✅ 不保存任何图像数据
- ✅ 不上传任何数据到服务器
- ✅ 统计数据仅存储在本地
- ✅ 完全离线运行

---

## 🛠️ 开发

### 代码风格

遵循 Rust 官方代码风格：
```bash
cargo fmt
cargo clippy
```

### 测试

```bash
cargo test
```

---

## 📖 文档

详细文档请查看 [docs/](./docs/) 目录：

- [需求文档](./docs/requirements/) - 详细的功能需求和分析
- [设计文档](./docs/design/) - UI/UX 设计和架构设计
- [开发指南](./docs/development/) - 开发规范和贡献指南

---

## 🎯 品牌定位

- **中文名**：明眸 - 文雅、文化感强、聚焦眼睛健康
- **英文名**：Blinko - 源自 "blink"（眨眼），轻快友好
- **Logo 概念**：半闭眼的猫头鹰/熊猫/机器人，眼睛有"明眸"光泽

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 贡献指南

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

---

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

---

## 🙏 致谢

- OpenCV 社区
- Rust 社区
- 所有贡献者

---

**Made with ❤️ for your eye health**

*Keep it simple, keep it stupid.*

