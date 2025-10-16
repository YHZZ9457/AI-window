# AI Companion / AI 助手

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A modern, minimalist desktop AI assistant application built with SvelteKit and Tauri.

一个使用 SvelteKit 和 Tauri 构建的现代化、极简主义桌面 AI 助手应用。

## Features / 功能特性

- 🚀 **Modern UI** - Clean, minimalist black and white design with dark mode support
- 🌙 **Dark Mode** - Seamless theme switching with system preference detection
- 💬 **AI Chat** - Interactive chat with OpenAI and compatible APIs (e.g., DeepSeek)
- ⚡ **Fast & Lightweight** - Built with SvelteKit for optimal performance
- 🖥️ **Desktop App** - Native desktop experience with Tauri
  - ⚙️ **Customizable** - Configurable API settings and system prompts
  - 💾 **Export Chat** - Save conversations as JSON files
  - ⌨️ **Global Shortcut** - Quick access with customizable keyboard shortcuts
  - 📎 **File Attachment** - Attach text, PDF, and Word files via button, paste, or drag-and-drop
- 🖼️ **Image Support** - Upload and process images with visual preview
- 🎯 **Drag & Drop** - Intuitive file upload with visual feedback
- 🌍 **Multi-language** - Full internationalization support (English, Chinese, Japanese)

- 📎 **文件附件** - 通过按钮、粘贴或拖拽上传文本、PDF 和 Word 文件
- 🖼️ **图片支持** - 上传和处理图片，带有视觉预览
- 🎯 **拖放上传** - 直观的文件上传体验，带有视觉反馈
- 🌍 **多语言支持** - 完整的国际化支持（英文、中文、日文）

- 🚀 **现代化界面** - 简洁的黑白极简设计，支持夜间模式
- 🌙 **夜间模式** - 无缝主题切换，支持系统偏好检测
- 💬 **AI 对话** - 与 OpenAI 及兼容 API（如 DeepSeek）进行交互式聊天
- ⚡ **快速轻量** - 使用 SvelteKit 构建，性能优异
- 🖥️ **桌面应用** - 通过 Tauri 提供原生桌面体验
- ⚙️ **可定制** - 可配置的 API 设置和系统提示
- 💾 **导出对话** - 将对话保存为 JSON 文件
- ⌨️ **全局快捷键** - 可自定义的键盘快捷键快速访问
- 📎 **文件附件** - 通过按钮、粘贴或拖拽上传文本、PDF 和 Word 文件

## Downloads / 下载

You can download the latest version of AI Companion from the [GitHub Releases](https://github.com/YHZZ9457/AI-window/releases) page.

您可以从 [GitHub Releases](https://github.com/YHZZ9457/AI-window/releases) 页面下载 AI 助手的最新版本。

## Screenshots / 截图

### Main Chat Interface / 主聊天界面
```
简洁的黑白设计，左侧为AI消息，右侧为用户消息
Clean black and white design with AI messages on left, user messages on right
```

### Settings Page / 设置页面
```
配置API密钥、模型设置和快捷键
Configure API keys, model settings, and shortcuts
```

## Installation / 安装

### Prerequisites / 前置要求

- Node.js (v18 or higher)
- Rust (for Tauri development)
- OpenAI API key

### Steps / 步骤

1. **Clone the repository** / **克隆仓库**
   ```bash
   git clone <repository-url>
   cd ai-window
   ```

2. **Install dependencies** / **安装依赖**
   ```bash
   npm install
   ```

3. **Configure settings** / **配置设置**
   - Launch the application and go to Settings
   - Enter your API key and other settings in the UI
   
   - 启动应用并进入设置页面
   - 在界面中输入你的 API 密钥和其他设置

4. **Development mode** / **开发模式**
   ```bash
   npm run tauri dev
   ```

5. **Build for production** / **生产构建**
   ```bash
   npm run tauri build
   ```

## Usage / 使用说明

### Basic Usage / 基本使用

1. **Start the application** / **启动应用**
   - Run the built executable or use `npm run tauri dev`
   - 运行构建的可执行文件或使用 `npm run tauri dev`

2. **Configure settings** / **配置设置**
   - Click the settings icon in the top-right corner
   - Enter your OpenAI API key and preferred model
   - Set up global shortcut if desired
   - Choose your preferred theme (Light, Dark, or Auto)
   - Select your preferred language
   
   - 点击右上角的设置图标
   - 输入你的 OpenAI API 密钥和首选模型
   - 根据需要设置全局快捷键
   - 选择你偏好的主题（亮色、暗色或自动）
   - 选择你偏好的语言

3. **Start chatting** / **开始对话**
   - Type your message in the input field
   - Press Enter or click the send button
   - Use the clear button to start a new conversation
   - Toggle theme with the theme button in the header
   
   - 在输入框中输入你的消息
   - 按 Enter 键或点击发送按钮
   - 使用清除按钮开始新的对话
   - 通过标题栏的主题按钮切换主题

### File Upload Methods / 文件上传方式

1. **Button Upload** / **按钮上传**
   - Click the attachment button (+)
   - Select files from the file dialog
   - Supports images and text files
   
   - 点击附件按钮 (+)
   - 从文件对话框中选择文件
   - 支持图片和文本文件

2. **Drag & Drop** / **拖放上传**
   - Drag files directly onto the application window
   - Visual overlay provides feedback during drag
   - Automatic file type detection and processing
   
   - 直接将文件拖拽到应用窗口
   - 拖拽时显示视觉覆盖层提供反馈
   - 自动文件类型检测和处理

3. **Paste from Clipboard** / **剪贴板粘贴**
   - Copy images or files to clipboard
   - Paste directly into the application
   - Automatic processing of pasted content
   
   - 复制图片或文件到剪贴板
   - 直接粘贴到应用中
   - 自动处理粘贴的内容

### Keyboard Shortcuts / 键盘快捷键

- `Enter` - Send message / 发送消息
- `Escape` - Hide window / 隐藏窗口
- `Ctrl+/` - Focus message input / 聚焦消息输入框
- `Ctrl+Shift+E` - Export chat to JSON / 导出对话到 JSON
- `Ctrl+Shift+T` - Toggle theme / 切换主题
- Custom global shortcut (configurable) / 可自定义的全局快捷键

### Theme Management / 主题管理

- **Light Mode** - Clean white background with black text
- **Dark Mode** - Comfortable dark background with light text
- **Auto Mode** - Automatically follows your system theme preference
- **Quick Toggle** - Click the theme button in the header to switch between light and dark modes

- **亮色模式** - 干净的白色背景配黑色文字
- **暗色模式** - 舒适的深色背景配浅色文字
- **自动模式** - 自动跟随系统主题偏好
- **快速切换** - 点击标题栏的主题按钮在亮色和暗色模式之间切换

## Configuration / 配置

### Settings Configuration / 设置配置

Configure your settings through the application's Settings page:

通过应用的设置页面配置你的设置：

- **System Prompt** - Define AI behavior and personality
- **API Key** - Your API key from OpenAI or a compatible service (e.g., DeepSeek)
- **API URL** - OpenAI API endpoint or compatible service URL
- **Model Name** - Preferred AI model (e.g., gpt-4o-mini)
- **Global Shortcut** - Keyboard shortcut to show/hide the app
- **Clear Chat Shortcut** - Keyboard shortcut to clear chat history
- **Theme** - Choose between Light, Dark, or Auto (follow system)

- **API 地址** - OpenAI API 端点或兼容服务地址
- **模型名称** - 首选的 AI 模型（例如 gpt-4o-mini）
- **全局快捷键** - 显示/隐藏应用的键盘快捷键
- **清空对话快捷键** - 清空对话历史的键盘快捷键
- **主题** - 在亮色、暗色或自动（跟随系统）之间选择

### Supported File Formats / 支持的文件格式

- **Images** / **图片**: PNG, JPG, JPEG, GIF, WebP
- **Text Files** / **文本文件**: TXT, MD, JSON, CSV, HTML, CSS, JS, TS, PY, RS
- **Documents** / **文档**: PDF, DOCX

All uploaded files are processed automatically and can be referenced in your conversations.

所有上传的文件都会自动处理，可以在对话中引用。

- **系统提示** - 定义 AI 的行为和个性
- **API 类型** - 选择 OpenAI 或 OpenAI 兼容服务
- **API 密钥** - 你的 OpenAI 或兼容服务（如 DeepSeek）的 API 密钥
- **API 地址** - OpenAI API 端点或兼容服务地址
- **模型名称** - 首选的 AI 模型（例如 gpt-4o-mini）
- **全局快捷键** - 显示/隐藏应用的键盘快捷键
- **清空对话快捷键** - 清空对话历史的键盘快捷键
- **主题** - 在亮色、暗色或自动（跟随系统）之间选择

### Settings Page Options / 设置页面选项

- **System Prompt** - Define AI behavior and personality
- **API Type** - Choose between OpenAI or OpenAI-compatible services
- **API Key** - Your API key from OpenAI or a compatible service (e.g., DeepSeek)
- **API URL** - OpenAI API endpoint or compatible service URL
- **Model Name** - Preferred AI model (e.g., gpt-4o-mini)
- **Global Shortcut** - Keyboard shortcut to show/hide the app
- **Clear Chat Shortcut** - Keyboard shortcut to clear chat history
- **Theme** - Choose between Light, Dark, or Auto (follow system)

- **系统提示** - 定义 AI 的行为和个性
- **API 类型** - 选择 OpenAI 或 OpenAI 兼容服务
- **API 密钥** - 你的 OpenAI 或兼容服务（如 DeepSeek）的 API 密钥
- **API 地址** - OpenAI API 端点或兼容服务地址
- **模型名称** - 首选的 AI 模型（例如 gpt-4o-mini）
- **全局快捷键** - 显示/隐藏应用的键盘快捷键
- **清空对话快捷键** - 清空对话历史的键盘快捷键
- **主题** - 在亮色、暗色或自动（跟随系统）之间选择

## Development / 开发

### Project Structure / 项目结构

```
src/
├── routes/           # SvelteKit pages / SvelteKit 页面
│   ├── +page.svelte # Main chat interface / 主聊天界面
│   └── settings/     # Settings page / 设置页面
├── app.css          # Global styles / 全局样式
└── app.html         # HTML template / HTML 模板

src-tauri/
├── src/             # Rust backend / Rust 后端
└── tauri.conf.json  # Tauri configuration / Tauri 配置
```

### Available Scripts / 可用脚本

- `npm run dev` - Start development server / 启动开发服务器
- `npm run build` - Build for production / 生产构建
- `npm run preview` - Preview production build / 预览生产构建
- `npm run tauri dev` - Start Tauri development / 启动 Tauri 开发
- `npm run tauri build` - Build Tauri application / 构建 Tauri 应用

### Tech Stack / 技术栈

- **Frontend**: SvelteKit, TypeScript
- **Backend**: Tauri (Rust)
- **Styling**: CSS with modern design system
- **AI Integration**: OpenAI API

- **前端**: SvelteKit, TypeScript
- **后端**: Tauri (Rust)
- **样式**: CSS 现代化设计系统
- **AI 集成**: OpenAI API

## Contributing / 贡献

We welcome contributions! Please feel free to submit issues and pull requests.

欢迎贡献！请随时提交问题报告和拉取请求。

### Development Setup / 开发设置

1. Fork the repository / 复刻仓库
2. Create a feature branch / 创建功能分支
3. Make your changes / 进行修改
4. Test your changes / 测试修改
5. Submit a pull request / 提交拉取请求

## Changelog / 更新日志

### v0.4.4 (2025-10-16)

- **Drag & Drop:** Added intuitive drag and drop file upload with visual feedback
- **Image Support:** Enhanced image processing with preview and compression
- **UI Improvements:** Optimized file name display and attachment preview styling
- **Markdown Enhancement:** Improved Markdown rendering with better styling
- **Bug Fixes:** Fixed file type detection and import issues

- **拖放功能:** 添加了直观的拖放文件上传功能，带有视觉反馈
- **图片支持:** 增强了图片处理功能，支持预览和压缩
- **界面优化:** 优化了文件名显示和附件预览样式
- **Markdown 增强:** 改进了 Markdown 渲染效果，样式更美观
- **错误修复:** 修复了文件类型检测和导入问题

### v0.4.0 (2025-10-14)

- **Performance:** Optimized the application by removing unused code and fixing warnings.
- **Refactor:** Improved code readability by refactoring imports.
- **UI:** Hid the minimalist mode shortcut from the settings page.

- **性能:** 通过移除未使用的代码和修复警告来优化应用程序。
- **重构:** 通过重构导入来提高代码可读性。
- **界面:** 从设置页面隐藏了极简模式快捷键。

## License / 许可证

This project is licensed under the MIT License - see the LICENSE file for details.

本项目采用 MIT 许可证 - 详见 LICENSE 文件。

## Support / 支持

If you encounter any issues or have questions:

如果你遇到任何问题或有疑问：

- Check the existing issues on GitHub / 查看 GitHub 上的现有问题
- Create a new issue with detailed information / 创建包含详细信息的新问题
- Contact the development team / 联系开发团队

## Acknowledgments / 致谢

- [SvelteKit](https://kit.svelte.dev/) - The web framework used
- [Tauri](https://tauri.app/) - For building desktop applications
- [OpenAI](https://openai.com/) - AI capabilities
- [Inter Font](https://rsms.me/inter/) - Beautiful typography

- [SvelteKit](https://kit.svelte.dev/) - 使用的 Web 框架
- [Tauri](https://tauri.app/) - 用于构建桌面应用
- [OpenAI](https://openai.com/) - AI 能力
- [Inter Font](https://rsms.me/inter/) - 优美的字体排版

---

**Made with ❤️ for the developer community**

**为开发者社区用心打造 ❤️**