# 技术需求文档

本文档详细描述 Blinko / 明眸 的技术架构、技术栈选择和性能要求。

---

## 1. 技术栈选择

### 1.1 前端技术栈
- **框架**：React 18+
- **语言**：TypeScript 5+
- **构建工具**：Vite 5+
- **UI 库**：待定（可选：Ant Design / Material-UI / 自定义）
- **状态管理**：Zustand 或 React Context
- **国际化**：react-i18next
- **图表库**：Chart.js 或 Recharts（轻量级）

**选择理由**：
- 符合开发者技术背景（React + TypeScript + Vite）
- Vite 提供快速开发体验
- TypeScript 提供类型安全

### 1.2 桌面框架
- **框架**：Tauri 2.0+
- **后端语言**：Rust 1.90.0+
- **前端绑定**：Tauri API (TypeScript)

**选择理由**：
- 轻量级（相比 Electron）
- 使用 Rust 提供更好的性能和安全性
- 支持系统托盘、通知等系统级功能
- 符合开发者 Rust 技术背景

### 1.3 图像处理
- **方案 A（推荐）**：Python + OpenCV + MediaPipe
  - 通过子进程调用 Python 脚本
  - 使用标准输入输出通信
  - 优点：开发快速，模型成熟
  - 缺点：需要 Python 运行环境

- **方案 B（备选）**：纯 Rust 实现
  - 使用 `opencv-rust` 和 MediaPipe Rust 绑定
  - 优点：无需额外依赖，性能更好
  - 缺点：开发复杂度高，库生态较少

**推荐方案 A**：开发效率优先，后期可优化为方案 B

### 1.4 数据存储
- **数据库**：SQLite（通过 `sqlx` 或 `rusqlite`）
- **配置文件**：TOML 或 JSON（通过 Tauri 配置系统）

---

## 2. 系统架构

### 2.1 整体架构

```
┌─────────────────────────────────────┐
│         Frontend (React)           │
│  ┌──────────┐  ┌──────────┐       │
│  │   UI     │  │  State   │       │
│  │ Components│ │ Management│      │
│  └──────────┘  └──────────┘       │
└──────────────┬──────────────────────┘
               │ Tauri IPC
┌──────────────┴──────────────────────┐
│      Tauri Backend (Rust)           │
│  ┌──────────┐  ┌──────────┐       │
│  │  Timer   │  │  Config  │       │
│  │  Manager │  │  Manager │       │
│  └──────────┘  └──────────┘       │
│  ┌──────────┐  ┌──────────┐       │
│  │  Tray    │  │  Database│       │
│  │  Manager │  │  Manager │       │
│  └──────────┘  └──────────┘       │
└──────────────┬──────────────────────┘
               │
┌──────────────┴──────────────────────┐
│    Python Subprocess (Optional)      │
│  ┌──────────┐  ┌──────────┐       │
│  │  Blink   │  │  Posture │       │
│  │ Detection│  │ Detection│       │
│  └──────────┘  └──────────┘       │
└─────────────────────────────────────┘
```

### 2.2 模块划分

#### 前端模块
- `src/components/` - UI 组件
- `src/pages/` - 页面组件
- `src/store/` - 状态管理
- `src/hooks/` - 自定义 Hooks
- `src/utils/` - 工具函数
- `src/locales/` - 国际化文件

#### 后端模块（Rust）
- `src-tauri/src/main.rs` - 主入口
- `src-tauri/src/commands/` - Tauri 命令（IPC 接口）
- `src-tauri/src/timer/` - 定时器管理
- `src-tauri/src/tray/` - 系统托盘管理
- `src-tauri/src/database/` - 数据库操作
- `src-tauri/src/config/` - 配置管理
- `src-tauri/src/camera/` - 摄像头管理（如需要）

#### Python 模块（可选）
- `scripts/blink_detection.py` - 眨眼检测脚本
- `scripts/posture_detection.py` - 坐姿检测脚本
- `scripts/utils.py` - 工具函数

---

## 3. 核心功能技术实现

### 3.1 定时提醒实现

#### Rust 端
```rust
// 使用 tokio 异步运行时
use tokio::time::{interval, Duration};

// 定时器管理器
struct TimerManager {
    interval: Duration,
    callback: Box<dyn Fn() + Send + Sync>,
}

impl TimerManager {
    fn start(&self) {
        // 启动定时器
        // 通过 Tauri 事件系统通知前端
    }
}
```

#### 前端端
```typescript
// 监听 Tauri 事件
import { listen } from '@tauri-apps/api/event';

listen('timer:reminder', (event) => {
  // 显示提醒弹窗
});
```

### 3.2 眨眼检测实现

#### Python 脚本
```python
# 使用 MediaPipe Face Mesh
import cv2
import mediapipe as mp

def detect_blink(frame):
    # 检测关键点
    # 计算眼睛纵横比（EAR）
    # 返回是否眨眼
    pass
```

#### Rust 端调用
```rust
// 通过子进程调用 Python 脚本
use std::process::{Command, Stdio};

fn call_blink_detection(frame_data: Vec<u8>) -> Result<bool> {
    let output = Command::new("python")
        .arg("scripts/blink_detection.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    // 发送帧数据，接收检测结果
}
```

### 3.3 坐姿检测实现

#### Python 脚本
```python
# 使用 MediaPipe Pose
import mediapipe as mp

def detect_posture(frame):
    # 检测关键点（头部、肩部）
    # 计算相对位置
    # 判断是否异常
    pass
```

### 3.4 系统托盘实现

#### Rust 端
```rust
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem};

fn create_tray() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(SystemTrayMenuItem::new("打开", "open"))
        .add_item(SystemTrayMenuItem::new("设置", "settings"))
        .add_item(SystemTrayMenuItem::new("退出", "quit"));
    
    SystemTray::new().with_menu(menu)
}
```

---

## 4. 数据存储设计

### 4.1 数据库表结构

#### reminders 表
```sql
CREATE TABLE reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    type TEXT NOT NULL,  -- 'timer', 'blink', 'posture'
    action TEXT,  -- 'rest', 'skip', 'delay'
    duration INTEGER  -- 休息时长（秒）
);
```

#### statistics 表
```sql
CREATE TABLE statistics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,  -- YYYY-MM-DD
    reminder_count INTEGER DEFAULT 0,
    rest_count INTEGER DEFAULT 0,
    blink_alerts INTEGER DEFAULT 0,
    posture_alerts INTEGER DEFAULT 0,
    total_work_time INTEGER DEFAULT 0  -- 秒
);
```

### 4.2 配置文件结构

#### config.toml
```toml
[reminder]
interval = 20  # 分钟
enabled = true
sound = "gentle"

[blink_detection]
enabled = false
interval = 5  # 秒
threshold = 30  # 秒

[posture_detection]
enabled = false
sensitivity = "medium"

[ui]
language = "zh-CN"
theme = "light"
```

---

## 5. 性能要求

### 5.1 应用启动性能
- **冷启动时间**：< 3 秒
- **首次初始化**：< 10 秒（包含模型加载）
- **内存占用**：< 100MB（空闲时）

### 5.2 检测性能
- **眨眼检测延迟**：< 500ms
- **坐姿检测延迟**：< 2 秒
- **CPU 占用**：
  - 空闲时：< 1%
  - 检测时：< 15%

### 5.3 响应性能
- **UI 响应时间**：< 100ms
- **设置保存时间**：< 50ms
- **数据查询时间**：< 200ms（单次查询）

---

## 6. 安全要求

### 6.1 权限管理
- 摄像头权限：仅在用户启用时请求
- 文件系统权限：仅访问应用数据目录
- 网络权限：不需要（完全离线）

### 6.2 数据安全
- 所有数据存储在本地
- 配置文件加密存储（可选）
- 敏感信息不记录日志

### 6.3 隐私保护
- 图像数据不保存
- 统计数据匿名化
- 用户可随时删除所有数据

---

## 7. 可维护性要求

### 7.1 代码规范
- **Rust**：遵循 `rustfmt` 和 `clippy` 规范
- **TypeScript**：使用 ESLint + Prettier
- **Python**：遵循 PEP 8

### 7.2 文档要求
- 代码注释覆盖率 > 30%
- 公共 API 必须有文档注释
- 关键算法需要详细说明

### 7.3 测试要求
- 单元测试覆盖率 > 60%
- 集成测试覆盖核心功能
- 关键路径必须有测试

---

## 8. 部署要求

### 8.1 打包要求
- 使用 Tauri 打包工具
- 生成 Windows 安装包（.msi 或 .exe）
- 安装包大小 < 50MB（不含 Python 运行时）

### 8.2 依赖管理
- Rust 依赖：通过 `Cargo.toml`
- Node.js 依赖：通过 `package.json`
- Python 依赖：通过 `requirements.txt`（如需要）

### 8.3 版本管理
- 使用语义化版本（Semantic Versioning）
- 主版本号：重大功能更新
- 次版本号：新功能添加
- 修订版本号：Bug 修复

