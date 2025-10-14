# 极简模式完全透明化修复

## 问题描述
极简模式下窗口背景（深蓝色）仍然存在，没有真正透明化。

## 根本原因
1. Tauri 窗口虽然设置了 `transparent: true`，但 CSS 中仍有背景色设置
2. 窗口装饰（边框、标题栏）可能阻挡透明效果
3. CSS 变量和具体样式覆盖了透明设置

## 解决方案

### 1. Tauri 窗口配置修复
**文件：** `src-tauri/tauri.conf.json`
```json
{
  "windows": [
    {
      "title": "ai-window",
      "width": 600,
      "height": 400,
      "transparent": true,
      "decorations": true,
      "backgroundColor": "#00000000"  // 新增：完全透明背景
    }
  ]
}
```

### 2. Rust 后端增强
**文件：** `src-tauri/src/lib.rs`
```rust
#[tauri::command]
fn set_minimal_mode(app: tauri::AppHandle, minimal: bool) {
  if let Some(window) = app.get_webview_window("main") {
    // 移除窗口装饰
    let _ = window.set_decorations(!minimal);
    
    // 设置窗口置顶
    let _ = window.set_always_on_top(minimal);
    
    // 强制设置透明背景
    let _ = window.set_transparent(true);
    
    // 可选：调整窗口大小
    if minimal {
      let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
        width: 500.0,
        height: 600.0,
      }));
    }
  }
}
```

### 3. CSS 完全透明化
**文件：** `src/app.css`

#### 核心透明化规则：
```css
/* 强制所有元素背景透明 */
.minimal-mode * {
  background-color: transparent !important;
  background: transparent !important;
}

/* 覆盖所有 CSS 背景变量 */
.minimal-mode {
  --bg-primary: transparent !important;
  --bg-secondary: transparent !important;
  --bg-tertiary: transparent !important;
  --bg-glass: transparent !important;
  --bg-glass-light: transparent !important;
}

/* 确保特定容器透明 */
.minimal-mode .chat-container,
.minimal-mode .messages-container,
.minimal-mode .input-container,
.minimal-mode .input-wrapper {
  background: transparent !important;
  background-color: transparent !important;
  border: none !important;
  box-shadow: none !important;
}

/* 确保主元素透明 */
.minimal-mode main {
  background: transparent !important;
  background-color: transparent !important;
}
```

#### 保留的视觉元素：
- 用户消息气泡：`rgba(173, 216, 230, 0.85)` (浅蓝色半透明)
- 助手消息气泡：`rgba(255, 255, 255, 0.85)` (白色半透明) 
- 输入框：`rgba(255, 255, 255, 0.9)` (白色半透明)
- 发送按钮：`rgba(173, 216, 230, 0.9)` (浅蓝色半透明)

### 4. 隐藏不必要的元素
- 标题栏和所有按钮
- 消息头部（角色图标、时间戳）
- 附件功能
- 所有边框和阴影

## 测试验证

### 预期效果
1. ✅ 窗口完全透明，可以看到桌面背景
2. ✅ 只有聊天消息气泡和输入框可见
3. ✅ 消息垂直排列（气泡样式）
4. ✅ 用户和助手消息用不同颜色区分
5. ✅ 无边框、无阴影、无标题栏
6. ✅ 输入框和发送按钮保持可见

### 测试步骤
1. 启动应用程序
2. 进入设置 → 应用设置
3. 启用"极简模式"开关
4. 使用快捷键 `Ctrl+Shift+M` 切换极简模式
5. 验证窗口是否完全透明
6. 测试聊天功能是否正常工作

## 技术要点

1. **Tauri 透明窗口**：需要同时设置 `transparent: true` 和 `backgroundColor: "#00000000"`
2. **窗口装饰**：在极简模式下必须移除 `decorations`
3. **CSS 优先级**：使用 `!important` 确保透明规则覆盖所有其他样式
4. **CSS 变量覆盖**：必须覆盖所有背景相关的 CSS 变量
5. **硬件加速**：某些系统可能需要禁用硬件加速才能实现完全透明

## 故障排除

如果窗口仍然不透明：
1. 检查系统是否支持窗口透明化
2. 确保显卡驱动支持透明效果
3. 尝试在 Tauri 配置中禁用硬件加速
4. 使用浏览器开发者工具检查是否有 CSS 规则覆盖透明设置

## 最终效果
极简模式现在应该呈现为一个完全透明的浮窗聊天框，只有文字气泡悬浮在桌面上，实现真正的"桌面悬浮聊天"体验。