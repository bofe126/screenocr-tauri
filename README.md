# ScreenOCR Tauri - 现代化 OCR 屏幕识别工具

🎉 使用 **Tauri + Rust + Vue 3 + Element Plus** 重构的高性能 OCR 应用

## ✨ 特性

- 🚀 **极速启动** - Rust 原生性能，启动速度极快
- 🎨 **现代 UI** - Vue 3 + Element Plus 精美界面
- 🔒 **安全可靠** - Tauri 提供的原生安全保障
- 📦 **体积小巧** - 比 Electron 应用小 10 倍以上
- 🌍 **跨平台** - Windows / macOS / Linux 全支持

## 🛠️ 技术栈

### 前端
- **Vue 3** - 渐进式 JavaScript 框架
- **TypeScript** - 类型安全
- **Element Plus** - Vue 3 组件库
- **Pinia** - Vue 3 状态管理
- **Vite** - 下一代前端构建工具

### 后端
- **Rust** - 系统级编程语言
- **Tauri** - 轻量级桌面应用框架
- **Tokio** - 异步运行时
- **Serde** - 序列化/反序列化

## 📥 安装

### 前置要求

1. **Node.js** (v18+)
   ```bash
   node --version
   ```

2. **Rust** (最新稳定版)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustc --version
   ```

3. **系统依赖**
   
   **Windows:**
   - Microsoft Visual Studio C++ Build Tools
   - WebView2 (Windows 10/11 自带)
   
   **macOS:**
   ```bash
   xcode-select --install
   ```
   
   **Linux:**
   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.0-dev \
       build-essential \
       curl \
       wget \
       libssl-dev \
       libgtk-3-dev \
       libayatana-appindicator3-dev \
       librsvg2-dev
   ```

### 开发环境设置

```bash
# 1. 克隆项目
git clone https://github.com/yourusername/screenocr-tauri.git
cd screenocr-tauri

# 2. 安装前端依赖
npm install

# 3. 启动开发服务器
npm run tauri:dev
```

## 🚀 使用

### 开发模式

```bash
npm run tauri:dev
```

### 构建生产版本

```bash
npm run tauri:build
```

构建完成后，可执行文件位于 `src-tauri/target/release/`

## 📝 使用说明

1. **启动应用** - 应用会在系统托盘运行
2. **左键点击托盘图标** - 打开设置页面
3. **配置 OCR 引擎** - 选择 Tesseract 或 WeChatOCR
4. **设置快捷键** - 自定义触发 OCR 的热键
5. **开始使用** - 按下快捷键开始识别

## 🗺️ 开发路线图

### 第一阶段：基础功能 (Week 1-2)
- [x] 项目结构搭建
- [x] 基础 UI 实现
- [x] 配置管理
- [ ] Rust 后端基础架构

### 第二阶段：核心功能 (Week 3-4)
- [ ] 屏幕截图功能
- [ ] Tesseract OCR 集成
- [ ] 全局热键监听
- [ ] 文本自动复制

### 第三阶段：高级功能 (Week 5-6)
- [ ] WeChatOCR 集成
- [ ] 文本智能选择
- [ ] OCR 结果历史
- [ ] 多显示器支持

### 第四阶段：扩展功能 (Week 7-8)
- [ ] 翻译功能集成
- [ ] AI 智能分析
- [ ] 自动更新
- [ ] 插件系统

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议！

1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的修改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开一个 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- [Tauri](https://tauri.app/) - 让桌面应用开发变得简单
- [Vue](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org/) - Vue 3 组件库
- [Rust](https://www.rust-lang.org/) - 系统级编程语言

## 📧 联系

如有问题或建议，请通过以下方式联系：

- 提交 [Issue](https://github.com/yourusername/screenocr-tauri/issues)
- 发送邮件至：your.email@example.com

---

⭐ 如果这个项目对你有帮助，请给个 Star！

