# 代码规范

本文档定义 Blinko / 明眸 项目的代码风格和编写规范。

---

## 1. 核心原则

### 1.1 精简原则
- **避免过度设计**：只实现当前需要的功能，不预判未来需求
- **代码简洁**：优先使用简单直接的方式，避免不必要的抽象
- **删除冗余**：定期清理未使用的代码、注释和依赖

### 1.2 可读性优先
- 代码是给人读的，不是给机器读的
- 清晰的命名比复杂的注释更重要
- 简单的逻辑比优雅的语法更重要

---

## 2. TypeScript / JavaScript 规范

### 2.1 命名规范
```typescript
// ✅ 好的命名
const reminderInterval = 20;
function getUserSettings() {}
class TimerManager {}

// ❌ 不好的命名
const ri = 20;
function get() {}
class TM {}
```

- **变量/函数**：使用 camelCase
- **常量**：使用 UPPER_SNAKE_CASE
- **类/组件**：使用 PascalCase
- **类型/接口**：使用 PascalCase，接口名可以加 `I` 前缀（可选）

### 2.2 函数规范
```typescript
// ✅ 简洁、单一职责
function calculateReminderTime(interval: number): number {
  return interval * 60 * 1000;
}

// ❌ 过度复杂
function calculateReminderTimeWithValidationAndLoggingAndErrorHandling(
  interval: number,
  options?: { validate?: boolean; log?: boolean }
): number {
  // 太多职责
}
```

- **函数长度**：不超过 50 行，优先拆分为更小的函数
- **参数数量**：不超过 3 个，超过则使用对象参数
- **单一职责**：一个函数只做一件事

### 2.3 组件规范
```typescript
// ✅ 简洁的组件
function ReminderCard({ count }: { count: number }) {
  return (
    <div className="reminder-card">
      <span>今日提醒: {count}</span>
    </div>
  );
}

// ❌ 过度设计的组件
function ReminderCard({
  count,
  onUpdate,
  theme,
  animation,
  analytics,
  // ... 太多 props
}: ReminderCardProps) {
  // 太多功能
}
```

- **组件大小**：单个文件不超过 300 行
- **Props 数量**：不超过 5 个，超过则考虑拆分或使用 Context
- **避免过早抽象**：只有 2 处以上重复时才考虑提取组件

### 2.4 类型定义
```typescript
// ✅ 简洁的类型
interface ReminderConfig {
  interval: number;
  enabled: boolean;
}

// ❌ 过度复杂的类型
interface ReminderConfig {
  interval: number;
  enabled: boolean;
  metadata?: {
    created?: Date;
    updated?: Date;
    version?: string;
    // ... 太多字段
  };
}
```

- **类型定义**：只包含实际使用的字段
- **避免过度泛型**：只在真正需要时使用泛型
- **使用类型推断**：让 TypeScript 自动推断类型

---

## 3. Rust 规范

### 3.1 命名规范
```rust
// ✅ 好的命名
struct TimerManager {
    interval: Duration,
}

fn get_user_settings() -> Result<Config> {
    // ...
}

// ❌ 不好的命名
struct TM {
    iv: Duration,
}

fn get() -> Result<C> {
    // ...
}
```

- **函数/变量**：使用 snake_case
- **类型/结构体**：使用 PascalCase
- **常量**：使用 UPPER_SNAKE_CASE
- **模块**：使用 snake_case

### 3.2 错误处理
```rust
// ✅ 简洁的错误处理
fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")?;
    toml::from_str(&content).map_err(Into::into)
}

// ❌ 过度复杂的错误处理
fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    match std::fs::read_to_string("config.toml") {
        Ok(content) => {
            match toml::from_str(&content) {
                Ok(config) => Ok(config),
                Err(e) => Err(Box::new(e)),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}
```

- **使用 `?` 操作符**：简化错误传播
- **避免过度嵌套**：使用早期返回或 `?` 操作符
- **错误类型**：使用 `Result<T, E>` 而不是 `Option<T>`

### 3.3 代码组织
```rust
// ✅ 简洁的模块结构
mod timer {
    pub struct Manager;
    impl Manager {
        pub fn new() -> Self { Self }
    }
}

// ❌ 过度复杂的模块结构
mod timer {
    mod manager {
        mod core {
            mod implementation {
                // 太多层级
            }
        }
    }
}
```

- **模块层级**：不超过 3 层
- **文件大小**：单个文件不超过 500 行
- **公共 API**：只暴露必要的函数和类型

---

## 4. Python 规范（可选）

### 4.1 遵循 PEP 8
- 使用 4 空格缩进
- 行长度不超过 100 字符
- 函数名使用 snake_case
- 类名使用 PascalCase

### 4.2 简洁原则
```python
# ✅ 简洁的函数
def detect_blink(frame):
    """检测眨眼"""
    # 简单直接的实现
    return result

# ❌ 过度设计
def detect_blink_with_multiple_algorithms_and_validation(
    frame, 
    algorithm="default",
    validate=True,
    log=True
):
    # 太多参数和选项
    pass
```

---

## 5. 注释规范

### 5.1 注释原则
- **代码优先**：让代码自己说话，注释是补充
- **解释为什么**：注释应该解释"为什么"，而不是"是什么"
- **删除过时注释**：及时删除过时或错误的注释

```typescript
// ✅ 好的注释
// 使用 20-20-20 法则，20 分钟提醒一次
const INTERVAL = 20;

// ❌ 不好的注释
// 设置间隔为 20
const INTERVAL = 20; // 这个变量是间隔时间
```

### 5.2 文档注释
```rust
// ✅ 好的文档注释
/// 启动定时器，每 `interval` 分钟触发一次提醒
/// 
/// # 参数
/// * `interval` - 提醒间隔（分钟）
/// 
/// # 返回
/// 成功返回 `Ok(())`，失败返回错误
pub fn start_timer(interval: u64) -> Result<()> {
    // ...
}
```

---

## 6. 文件组织

### 6.1 文件大小
- **TypeScript/React**：单个文件不超过 300 行
- **Rust**：单个文件不超过 500 行
- **Python**：单个文件不超过 300 行

### 6.2 导入顺序
```typescript
// 1. 外部库
import React from 'react';
import { useState } from 'react';

// 2. 内部模块
import { useReminder } from '@/hooks/useReminder';
import { formatTime } from '@/utils/format';

// 3. 类型
import type { ReminderConfig } from '@/types';
```

---

## 7. 代码审查清单

在提交代码前，检查以下问题：

- [ ] 代码是否简洁易懂？
- [ ] 是否有未使用的代码、导入或变量？
- [ ] 函数是否单一职责？
- [ ] 命名是否清晰？
- [ ] 是否有过度设计的地方？
- [ ] 注释是否必要且准确？
- [ ] 是否遵循项目规范？

---

## 8. 工具配置

### 8.1 ESLint（TypeScript）
使用项目配置的 ESLint 规则

### 8.2 Rustfmt（Rust）
使用默认配置，运行 `cargo fmt`

### 8.3 Prettier（前端）
使用项目配置的 Prettier 规则

---

**记住：简洁是最好的设计，过度设计是最大的浪费。**

