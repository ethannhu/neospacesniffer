# 📦 neoSpaceSniffer

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/Vue-3-brightgreen?logo=vue.js)](https://vuejs.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri)](https://tauri.app/)

一个基于 **Tauri + Rust + Vue3** 的跨平台桌面应用，提供类似 [SpaceSniffer](http://www.uderzo.it/main_products/space_sniffer/) 的磁盘空间可视化功能。
它能够帮助你直观地分析磁盘使用情况，找到大文件和占用空间最多的目录。

---

## ✨ 特性

* 🚀 **跨平台**：支持 Windows / Linux / macOS
* 📊 **可视化**：提供类似 SpaceSniffer 的矩形树图展示
* 🔍 **实时扫描**：扫描线程与 UI 分离，边扫描边展示结果
* 📁 **文件夹选择**：可选择任意目录进行分析
* 🖥 **轻量级**：基于 Tauri，体积小、内存占用低

---

## 📷 截图



---

## 🛠 技术栈

* **前端**：Vue 3 + TypeScript + ECharts
* **后端**：Rust (多线程扫描、文件大小计算)
* **框架**：Tauri 2.0

---

## 📦 安装 & 构建

### 环境依赖

* [Rust](https://www.rust-lang.org/) >= 1.70
* [Node.js](https://nodejs.org/) >= 18
* [pnpm](https://pnpm.io/) 或 npm/yarn
* Tauri CLI

  ```bash
  cargo install tauri-cli
  ```

### 克隆仓库

```bash
git clone https://github.com/yourname/neoSpaceSniffer.git
cd neoSpaceSniffer
```

### 安装依赖

```bash
pnpm install   # 或 npm install / yarn install
```

### 启动开发环境

```bash
pnpm tauri dev
```

### 构建应用

```bash
pnpm tauri build
```

---

## 📂 项目结构

```
neoSpaceSniffer/
├── src-tauri/        # Rust 后端（扫描、文件系统逻辑）
│   ├── src/
│   │   └── main.rs   # Tauri 入口
│   └── Cargo.toml
├── src/              # 前端 Vue3 应用
│   ├── components/
│   └── App.vue
└── package.json
```

---

## 🧩 路线图

* [x] 基础目录扫描
* [x] 大文件过滤 (≥10MB)
* [x] 多线程扫描与读取
* [ ] 磁盘树形可视化 (Treemap)
* [ ] 文件搜索 / 筛选
* [ ] 导出 JSON 报告
* [ ] Dark Mode

---

## 🤝 贡献

欢迎提交 Issue 和 PR！
你可以这样参与：

1. Fork 本仓库
2. 创建新分支：`git checkout -b feature/xxx`
3. 提交更改：`git commit -m 'Add some xxx'`
4. 推送分支：`git push origin feature/xxx`
5. 提交 Pull Request

---

## 📜 License

[MIT License](LICENSE)

---
