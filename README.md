# 👁️ Blinko / 明眸

> **明眸善睐，坐如钟** | *Blinko watches, so you can rest.*

一个轻量、智能的 Windows 桌面健康提醒工具，专注于保护你的眼睛和坐姿健康。

---

## ✨ 核心功能

- 🕐 **定时提醒**：20-20-20 护眼法则自动提醒
- 👁️ **眨眼激励**：通过摄像头检测，提醒你多眨眼
- 🪑 **坐姿检测**：实时监测坐姿，提醒保持正确姿势
- 📊 **数据统计**：记录你的健康习惯，可视化展示趋势
- 🔔 **系统托盘**：轻量常驻，不打扰但可交互
- 🌐 **中英双语**：默认中文（明眸场景），支持英文切换
- 🔒 **隐私优先**：所有图像处理在本地完成，不上传任何数据

---

## 🛠️ 技术栈

- **前端**：React + TypeScript + Vite
- **桌面框架**：Tauri (Rust + Web 前端)
- **系统交互**：Rust 1.90.0
- **图像处理**：Python + OpenCV + MediaPipe（可选，通过子进程调用）

---

## 📦 项目结构

```
Blinko/
├── src-tauri/          # Tauri 后端（Rust）
├── src/                # 前端（React + TypeScript）
├── docs/               # 项目文档
│   ├── requirements/   # 需求文档
│   ├── design/         # 设计文档
│   └── api/            # API 文档
├── scripts/            # 辅助脚本
└── README.md           # 本文件
```

详细文档请查看 [docs/](./docs/) 目录。

---

## 🚀 快速开始

> ⚠️ 项目正在开发中，敬请期待...

### 环境要求

- Node.js v18+
- Rust 1.90.0+
- Windows 10/11

### 安装步骤

```bash
# 克隆仓库
git clone <repository-url>
cd Blinko

# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建生产版本
npm run tauri build
```

---

## 📖 文档导航

- [需求文档](./docs/requirements/) - 详细的功能需求和分析
- [设计文档](./docs/design/) - UI/UX 设计和架构设计
- [开发指南](./docs/development/) - 开发规范和贡献指南

---

## 🎯 品牌定位

- **中文名**：明眸 - 文雅、文化感强、聚焦眼睛健康
- **英文名**：Blinko - 源自 "blink"（眨眼），轻快友好
- **Logo 概念**：半闭眼的猫头鹰/熊猫/机器人，眼睛有"明眸"光泽

---

## 📄 许可证

[待定]

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

**Made with ❤️ for your eye health**

