# 项目结构文档

本文档描述 Blinko / 明眸 项目的完整目录结构和文件组织。

---

## 📁 目录结构

```
Blinko/
├── .github/                    # GitHub 配置文件
│   └── workflows/              # CI/CD 工作流
├── docs/                       # 项目文档
│   ├── requirements/           # 需求文档
│   │   ├── README.md          # 需求文档索引
│   │   ├── 01-core-requirements.md
│   │   ├── 02-functional-requirements.md
│   │   ├── 03-technical-requirements.md
│   │   ├── 04-ux-requirements.md
│   │   ├── 05-non-functional-requirements.md
│   │   └── SUMMARY.md         # 需求总结
│   ├── design/                 # 设计文档
│   │   ├── README.md
│   │   ├── 01-architecture.md
│   │   ├── 02-ui-design.md
│   │   ├── 03-interaction-design.md
│   │   └── 04-data-model.md
│   ├── development/            # 开发文档
│   │   ├── README.md
│   │   ├── 01-setup.md
│   │   ├── 02-code-style.md
│   │   ├── 03-git-workflow.md
│   │   ├── 04-testing.md
│   │   └── 05-build-release.md
│   └── PROJECT_STRUCTURE.md    # 本文件
├── src/                        # 前端源代码（React + TypeScript）
│   ├── components/             # UI 组件
│   │   ├── common/            # 通用组件
│   │   ├── reminder/          # 提醒相关组件
│   │   ├── statistics/        # 统计相关组件
│   │   └── settings/          # 设置相关组件
│   ├── pages/                  # 页面组件
│   │   ├── Dashboard.tsx      # 主面板
│   │   ├── Statistics.tsx     # 统计页面
│   │   └── Settings.tsx       # 设置页面
│   ├── store/                  # 状态管理
│   │   ├── config.ts          # 配置状态
│   │   ├── reminder.ts        # 提醒状态
│   │   └── statistics.ts      # 统计状态
│   ├── hooks/                  # 自定义 Hooks
│   │   ├── useReminder.ts
│   │   ├── useBlinkDetection.ts
│   │   └── usePostureDetection.ts
│   ├── utils/                  # 工具函数
│   │   ├── format.ts          # 格式化函数
│   │   ├── validation.ts      # 验证函数
│   │   └── constants.ts       # 常量定义
│   ├── locales/                # 国际化文件
│   │   ├── zh-CN.json
│   │   └── en-US.json
│   ├── assets/                 # 静态资源
│   │   ├── images/
│   │   ├── icons/
│   │   └── fonts/
│   ├── styles/                 # 样式文件
│   │   ├── global.css
│   │   └── themes.css
│   ├── App.tsx                 # 根组件
│   ├── main.tsx                # 入口文件
│   └── vite-env.d.ts           # Vite 类型定义
├── src-tauri/                  # Tauri 后端（Rust）
│   ├── src/
│   │   ├── main.rs             # 主入口
│   │   ├── commands/           # Tauri 命令（IPC 接口）
│   │   │   ├── mod.rs
│   │   │   ├── reminder.rs
│   │   │   ├── config.rs
│   │   │   ├── statistics.rs
│   │   │   └── camera.rs
│   │   ├── timer/              # 定时器管理
│   │   │   ├── mod.rs
│   │   │   └── manager.rs
│   │   ├── tray/               # 系统托盘管理
│   │   │   ├── mod.rs
│   │   │   └── manager.rs
│   │   ├── database/           # 数据库操作
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs
│   │   │   └── models.rs
│   │   ├── config/             # 配置管理
│   │   │   ├── mod.rs
│   │   │   └── manager.rs
│   │   └── utils/              # 工具函数
│   │       ├── mod.rs
│   │       └── error.rs
│   ├── Cargo.toml              # Rust 依赖配置
│   └── tauri.conf.json         # Tauri 配置文件
├── scripts/                    # 辅助脚本
│   ├── blink_detection.py     # 眨眼检测脚本
│   ├── posture_detection.py    # 坐姿检测脚本
│   └── utils.py                # Python 工具函数
├── tests/                      # 测试文件
│   ├── unit/                   # 单元测试
│   ├── integration/            # 集成测试
│   └── e2e/                    # 端到端测试
├── .gitignore                  # Git 忽略文件
├── .editorconfig              # 编辑器配置
├── .eslintrc.json             # ESLint 配置
├── .prettierrc                # Prettier 配置
├── package.json               # Node.js 依赖配置
├── tsconfig.json              # TypeScript 配置
├── vite.config.ts             # Vite 配置
└── README.md                  # 项目说明（根目录）
```

---

## 📝 文件说明

### 根目录文件

- **README.md**：项目主说明文档，包含项目介绍、快速开始等
- **.gitignore**：Git 忽略文件配置
- **package.json**：Node.js 项目配置和依赖
- **tsconfig.json**：TypeScript 编译配置
- **vite.config.ts**：Vite 构建工具配置

### 前端目录（src/）

- **components/**：可复用的 UI 组件
- **pages/**：页面级组件
- **store/**：状态管理（Zustand 或 Context）
- **hooks/**：自定义 React Hooks
- **utils/**：工具函数和常量
- **locales/**：国际化翻译文件
- **assets/**：静态资源（图片、图标、字体）
- **styles/**：全局样式和主题

### 后端目录（src-tauri/）

- **src/main.rs**：Tauri 应用主入口
- **src/commands/**：Tauri IPC 命令，前端与后端通信接口
- **src/timer/**：定时器管理模块
- **src/tray/**：系统托盘管理模块
- **src/database/**：数据库操作模块
- **src/config/**：配置管理模块
- **Cargo.toml**：Rust 依赖和项目配置
- **tauri.conf.json**：Tauri 应用配置

### 脚本目录（scripts/）

- **blink_detection.py**：眨眼检测 Python 脚本
- **posture_detection.py**：坐姿检测 Python 脚本
- **utils.py**：Python 工具函数

### 文档目录（docs/）

- **requirements/**：需求文档
- **design/**：设计文档
- **development/**：开发文档

---

## 🔄 数据流

### 前端 → 后端
```
React Component
    ↓
Tauri Command (IPC)
    ↓
Rust Handler
    ↓
Database / System API
```

### 后端 → 前端
```
Timer / Detection
    ↓
Tauri Event
    ↓
React Hook (useEffect)
    ↓
UI Update
```

---

## 📦 依赖管理

### 前端依赖
- 通过 `package.json` 管理
- 使用 `npm` 或 `pnpm` 安装

### 后端依赖
- 通过 `Cargo.toml` 管理
- 使用 `cargo` 安装

### Python 依赖（可选）
- 通过 `requirements.txt` 管理
- 使用 `pip` 安装

---

## 🛠️ 开发工具配置

### 编辑器配置
- **.editorconfig**：统一编辑器设置
- **.vscode/**：VS Code 工作区配置（可选）

### 代码质量
- **.eslintrc.json**：ESLint 代码检查配置
- **.prettierrc**：Prettier 代码格式化配置
- **rustfmt.toml**：Rust 代码格式化配置

---

## 📊 数据存储位置

### 开发环境
- **配置文件**：`src-tauri/target/debug/`
- **数据库**：`src-tauri/target/debug/data/blinko.db`
- **日志**：`src-tauri/target/debug/logs/`

### 生产环境
- **Windows**：`%APPDATA%\Blinko\`
  - 配置：`config.toml`
  - 数据库：`data/blinko.db`
  - 日志：`logs/blinko.log`

---

## 🔐 安全考虑

- **敏感数据**：不提交到版本控制
- **API 密钥**：使用环境变量
- **用户数据**：存储在用户数据目录，不共享

---

**最后更新**：2024

