# AI Companion / AI 助手

A modern, minimalist desktop AI assistant application built with SvelteKit and Tauri.

一个使用 SvelteKit 和 Tauri 构建的现代化、极简主义桌面 AI 助手应用。

## Features / 功能特性

- 🚀 **Modern UI** - Clean, minimalist black and white design with dark mode support
- 🌙 **Dark Mode** - Seamless theme switching with system preference detection
- 💬 **AI Chat** - Interactive chat interface with OpenAI integration
- ⚡ **Fast & Lightweight** - Built with SvelteKit for optimal performance
- 🖥️ **Desktop App** - Native desktop experience with Tauri
- ⚙️ **Customizable** - Configurable API settings and system prompts
- 💾 **Export Chat** - Save conversations as JSON files
- ⌨️ **Global Shortcut** - Quick access with customizable keyboard shortcuts

- 🚀 **现代化界面** - 简洁的黑白极简设计，支持夜间模式
- 🌙 **夜间模式** - 无缝主题切换，支持系统偏好检测
- 💬 **AI 对话** - 集成 OpenAI 的交互式聊天界面
- ⚡ **快速轻量** - 使用 SvelteKit 构建，性能优异
- 🖥️ **桌面应用** - 通过 Tauri 提供原生桌面体验
- ⚙️ **可定制** - 可配置的 API 设置和系统提示
- 💾 **导出对话** - 将对话保存为 JSON 文件
- ⌨️ **全局快捷键** - 可自定义的键盘快捷键快速访问

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

3. **Configure environment** / **配置环境**
   - Copy `.env.example` to `.env`
   - Add your OpenAI API key
   
   - 复制 `.env.example` 为 `.env`
   - 添加你的 OpenAI API 密钥

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
   
   - 点击右上角的设置图标
   - 输入你的 OpenAI API 密钥和首选模型
   - 根据需要设置全局快捷键
   - 选择你偏好的主题（亮色、暗色或自动）

3. **Start chatting** / **开始对话**
   - Type your message in the input field
   - Press Enter or click the send button
   - Use the clear button to start a new conversation
   - Toggle theme with the theme button in the header
   
   - 在输入框中输入你的消息
   - 按 Enter 键或点击发送按钮
   - 使用清除按钮开始新的对话
   - 通过标题栏的主题按钮切换主题

### Keyboard Shortcuts / 键盘快捷键

- `Enter` - Send message / 发送消息
- `Escape` - Hide window / 隐藏窗口
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

### Environment Variables / 环境变量

Create a `.env` file in the root directory:

在根目录创建 `.env` 文件：

```env
# OpenAI API Configuration / OpenAI API 配置
OPENAI_API_KEY=your_api_key_here
OPENAI_API_URL=https://api.openai.com/v1/chat/completions
OPENAI_MODEL=gpt-4o-mini

# Application Settings / 应用设置
GLOBAL_SHORTCUT=Alt+Space
SYSTEM_PROMPT=You are a helpful AI assistant.
```

### Settings Page Options / 设置页面选项

- **System Prompt** - Define AI behavior and personality
- **API Key** - Your OpenAI API key (stored locally)
- **API URL** - OpenAI API endpoint
- **Model Name** - Preferred AI model (e.g., gpt-4o-mini)
- **Global Shortcut** - Keyboard shortcut to show/hide the app
- **Theme** - Choose between Light, Dark, or Auto (follow system)

- **系统提示** - 定义 AI 的行为和个性
- **API 密钥** - 你的 OpenAI API 密钥（本地存储）
- **API 地址** - OpenAI API 端点
- **模型名称** - 首选的 AI 模型（例如 gpt-4o-mini）
- **全局快捷键** - 显示/隐藏应用的键盘快捷键
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