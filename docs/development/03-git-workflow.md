# Git 工作流和提交规范

本文档定义 Blinko / 明眸 项目的 Git 工作流程和提交信息规范。

---

## 1. 分支策略

### 1.1 分支命名
- **主分支**：`main` - 生产环境代码
- **开发分支**：`develop` - 开发环境代码（可选）
- **功能分支**：`feature/功能名称` - 新功能开发
- **修复分支**：`fix/问题描述` - Bug 修复
- **文档分支**：`docs/文档内容` - 文档更新
- **重构分支**：`refactor/重构内容` - 代码重构

### 1.2 分支工作流
```
main (生产环境)
  ↑
develop (开发环境)
  ↑
feature/xxx (功能开发)
```

**简化流程**（小团队）：
- 直接从 `main` 创建功能分支
- 完成开发后合并到 `main`
- 使用 Pull Request 进行代码审查

---

## 2. 提交信息规范

### 2.1 提交信息格式

遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <subject>

<body>

<footer>
```

### 2.2 类型（Type）

必填，描述提交类型：

- **feat**: 新功能
- **fix**: Bug 修复
- **docs**: 文档更新
- **style**: 代码格式调整（不影响功能）
- **refactor**: 代码重构
- **perf**: 性能优化
- **test**: 测试相关
- **chore**: 构建/工具链相关
- **ci**: CI/CD 相关

### 2.3 范围（Scope）

可选，描述影响范围：

- `ui`: 用户界面
- `timer`: 定时器功能
- `detection`: 检测功能
- `config`: 配置管理
- `database`: 数据库
- `tray`: 系统托盘
- `docs`: 文档

### 2.4 主题（Subject）

必填，简洁描述提交内容：

- 使用中文或英文（保持一致）
- 不超过 50 个字符
- 首字母小写
- 不以句号结尾
- 使用祈使语气（"添加"而不是"添加了"）

### 2.5 正文（Body）

可选，详细描述：

- 说明"为什么"而不是"是什么"
- 可以包含多行
- 每行不超过 72 个字符

### 2.6 页脚（Footer）

可选，包含：

- 关闭的 Issue：`Closes #123`
- 破坏性变更：`BREAKING CHANGE: 描述`

---

## 3. 提交信息示例

### 3.1 好的提交信息

```
feat(timer): 添加 20-20-20 定时提醒功能

实现每 20 分钟提醒用户休息的功能，支持自定义提醒间隔。

Closes #10
```

```
fix(detection): 修复眨眼检测误报问题

调整检测阈值，减少误报率。

Fixes #25
```

```
docs: 添加开发环境搭建指南

完善开发文档，添加环境配置步骤。
```

```
refactor(config): 简化配置管理逻辑

移除不必要的配置验证，简化代码结构。
```

### 3.2 不好的提交信息

```
❌ update
❌ fix bug
❌ 更新代码
❌ 修复了一些问题并添加了新功能
❌ feat: 添加了很多功能，包括定时提醒、眨眼检测、坐姿检测等
```

---

## 4. 提交频率

### 4.1 提交原则
- **小而频繁**：完成一个小功能就提交
- **原子性**：每次提交只做一件事
- **可回滚**：每次提交后代码都可以正常编译运行

### 4.2 提交前检查
- [ ] 代码可以正常编译运行
- [ ] 通过代码检查（ESLint / Clippy）
- [ ] 提交信息符合规范
- [ ] 没有提交临时文件或调试代码

---

## 5. Pull Request 规范

### 5.1 PR 标题
使用与提交信息相同的格式：

```
feat(timer): 添加定时提醒功能
```

### 5.2 PR 描述
包含以下内容：

```markdown
## 变更内容
- 添加了定时提醒功能
- 支持自定义提醒间隔

## 相关 Issue
Closes #10

## 测试
- [ ] 已测试定时提醒功能
- [ ] 已测试设置修改
```

### 5.3 PR 审查
- 至少需要 1 人审查
- 审查通过后合并到主分支
- 合并后删除功能分支

---

## 6. 代码审查清单

审查代码时，检查以下内容：

- [ ] 代码是否符合规范？
- [ ] 是否有过度设计？
- [ ] 是否有未使用的代码？
- [ ] 命名是否清晰？
- [ ] 是否有必要的注释？
- [ ] 是否有测试覆盖？
- [ ] 是否影响现有功能？

---

## 7. 提交工具配置

### 7.1 Commitizen（可选）

安装 Commitizen 辅助生成规范提交：

```bash
npm install -g commitizen
npm install --save-dev cz-conventional-changelog
```

在 `package.json` 中添加：

```json
{
  "config": {
    "commitizen": {
      "path": "./node_modules/cz-conventional-changelog"
    }
  },
  "scripts": {
    "commit": "cz"
  }
}
```

### 7.2 Commitlint（推荐）

安装 Commitlint 验证提交信息：

```bash
npm install --save-dev @commitlint/config-conventional @commitlint/cli
```

创建 `commitlint.config.js`：

```javascript
module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'type-enum': [
      2,
      'always',
      [
        'feat',
        'fix',
        'docs',
        'style',
        'refactor',
        'perf',
        'test',
        'chore',
        'ci',
      ],
    ],
    'subject-case': [0],
  },
};
```

### 7.3 Husky（Git Hooks）

安装 Husky 自动运行检查：

```bash
npm install --save-dev husky
npx husky install
npx husky add .husky/commit-msg 'npx --no -- commitlint --edit $1'
npx husky add .husky/pre-commit 'npm run lint && npm run test'
```

---

## 8. 版本管理

### 8.1 版本号格式

遵循 [语义化版本](https://semver.org/)：

```
主版本号.次版本号.修订号
例如：1.0.0
```

- **主版本号**：不兼容的 API 修改
- **次版本号**：向下兼容的功能新增
- **修订号**：向下兼容的问题修复

### 8.2 版本标签

使用 Git 标签标记版本：

```bash
git tag -a v1.0.0 -m "版本 1.0.0 发布"
git push origin v1.0.0
```

---

## 9. 快速参考

### 9.1 常用命令

```bash
# 创建功能分支
git checkout -b feature/timer-reminder

# 提交代码
git add .
git commit -m "feat(timer): 添加定时提醒功能"

# 推送分支
git push origin feature/timer-reminder

# 合并到主分支
git checkout main
git merge feature/timer-reminder
git push origin main
```

### 9.2 提交信息模板

```
<type>(<scope>): <subject>

<body>

<footer>
```

---

**记住：好的提交信息是项目历史的重要组成部分。**

