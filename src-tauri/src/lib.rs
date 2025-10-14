use tauri::{Manager, AppHandle};
use std::str::FromStr;
use std::fs;
use std::time::{Duration, Instant};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent};
use tauri_plugin_opener::OpenerExt;
use url::Url;
use regex::Regex;
use std::io::Write;
use std::env;
use tauri_plugin_store::StoreBuilder;



#[tauri::command]
fn set_decorations(window: tauri::Window, decorations: bool) {
  let _ = window.set_decorations(decorations);
}

#[tauri::command]
fn set_minimal_mode(app: tauri::AppHandle, minimal: bool) {
  if let Some(window) = app.get_webview_window("main") {
    // 移除窗口装饰
    let _ = window.set_decorations(!minimal);
    
    // 设置窗口置顶
    let _ = window.set_always_on_top(minimal);
  }
}

#[tauri::command]
fn register_shortcut(app: AppHandle, shortcut: String) -> Result<(), String> {
    let shortcut_manager = app.global_shortcut();
    let _ = shortcut_manager.unregister_all();
    let shortcut = Shortcut::from_str(&shortcut).map_err(|e| e.to_string())?;
    shortcut_manager.register(shortcut).map_err(|e| e.to_string())?;
    Ok(())
}

// --- Security Utilities ---

// --- Security Utilities ---

/// 输入验证和清理
struct InputValidator;

impl InputValidator {
    /// 验证URL是否安全
    fn validate_url(url: &str) -> Result<(), String> {
        let parsed = Url::parse(url).map_err(|e| format!("Invalid URL: {}", e))?;
        
        // 检查协议
        if parsed.scheme() != "https" && parsed.scheme() != "http" {
            return Err("URL must use http or https protocol".to_string());
        }
        
        // 检查是否为本地地址
        if let Some(host) = parsed.host_str() {
            if host == "localhost" || host == "127.0.0.1" || host == "::1" {
                return Err("Localhost URLs are not allowed for security reasons".to_string());
            }
            
            // 检查是否为私有网络地址
            if host.starts_with("192.168.") || 
               host.starts_with("10.") || 
               host.starts_with("172.") ||
               host.ends_with(".local") {
                return Err("Private network URLs are not allowed".to_string());
            }
        }
        
        Ok(())
    }
    
    /// 清理和验证系统提示
    fn sanitize_system_prompt(prompt: &str) -> String {
        let mut sanitized = prompt.to_string();
        
        // 移除潜在的恶意内容
        let dangerous_patterns = [
            (r"<script[^>]*>.*?</script>", ""),
            (r"javascript:", ""),
            (r"on\w+=", ""),
            (r"eval\(.*\)", ""),
            (r"exec\(.*\)", ""),
        ];
        
        for (pattern, replacement) in dangerous_patterns.iter() {
            if let Ok(re) = Regex::new(pattern) {
                sanitized = re.replace_all(&sanitized, *replacement).to_string();
            }
        }
        
        // 限制长度
        if sanitized.len() > 10000 {
            sanitized.truncate(10000);
            sanitized.push_str("... [truncated]");
        }
        
        sanitized
    }
    
    /// 验证模型名称
    fn validate_model_name(model: &str) -> Result<(), String> {
        if model.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }
        
        // 只允许字母、数字、连字符、下划线和点
        let valid_pattern = Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap();
        if !valid_pattern.is_match(model) {
            return Err("Model name contains invalid characters".to_string());
        }
        
        // 限制长度
        if model.len() > 100 {
            return Err("Model name is too long".to_string());
        }
        
        Ok(())
    }
    
    /// 验证文件扩展名
    fn validate_file_extension(filename: &str) -> Result<(), String> {
        let extension = std::path::Path::new(filename)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("")
            .to_lowercase();
        
        // 允许的文件类型
        let allowed_extensions = [
            "pdf", "docx", "txt", "md", "json", "csv", 
            "html", "css", "js", "ts", "py", "rs", 
            "toml", "yaml", "yml"
        ];
        
        if !allowed_extensions.contains(&extension.as_str()) {
            return Err(format!("File type '{}' is not supported", extension));
        }
        
        Ok(())
    }
}

/// 安全日志记录
struct SecurityLogger;

impl SecurityLogger {
    fn log_security_event_with_file(app: &AppHandle, event: &str, level: &str) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] [{}] {}\n", timestamp, level, event);
        
        // 写入安全日志文件（放在应用程序配置目录中）
        if let Ok(config_dir) = app.path().app_config_dir() {
            let log_path = config_dir.join("security.log");
            if let Ok(mut file) = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path) {
                let _ = file.write_all(log_entry.as_bytes());
            }
        }
        
        // 同时在控制台输出（仅调试模式）
        #[cfg(debug_assertions)]
        println!("Security Event: {}", log_entry.trim());
    }
    
    fn log_api_request(app: &AppHandle, url: &str, model: &str) {
        Self::log_security_event_with_file(
            app,
            &format!("API Request - URL: {}, Model: {}", url, model),
            "INFO"
        );
    }
    
    fn log_security_violation(app: &AppHandle, event: &str) {
        Self::log_security_event_with_file(app, event, "WARNING");
    }
    
    fn log_error(app: &AppHandle, event: &str) {
        Self::log_security_event_with_file(app, event, "ERROR");
    }
}

// --- Tauri Commands ---

#[tauri::command]
fn open_config_directory(app: AppHandle) -> Result<(), String> {
    let path = app.path().app_config_dir().map_err(|e| e.to_string())?;
    app.opener().open_url(path.to_string_lossy(), None::<String>).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Result<serde_json::Value, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let path = config_dir.join("settings.dat");
    let store = match StoreBuilder::new(app.app_handle(), path).build() {
        Ok(store) => store,
        Err(e) => return Err(e.to_string()),
    };
    let _ = store.reload();

    let mut settings = serde_json::Map::new();
    
    // 读取所有设置值
    if let Some(api_key) = store.get("api_key") {
        settings.insert("api_key".to_string(), api_key.clone());
    }
    if let Some(api_url) = store.get("api_url") {
        settings.insert("api_url".to_string(), api_url.clone());
    }
    if let Some(model_name) = store.get("model_name") {
        settings.insert("model_name".to_string(), model_name.clone());
    }
    if let Some(shortcut) = store.get("shortcut") {
        settings.insert("shortcut".to_string(), shortcut.clone());
    }
    if let Some(system_prompt) = store.get("system_prompt") {
        settings.insert("system_prompt".to_string(), system_prompt.clone());
    }
    if let Some(api_type) = store.get("api_type") {
        settings.insert("api_type".to_string(), api_type.clone());
    }
    if let Some(clear_chat_shortcut) = store.get("clear_chat_shortcut") {
        settings.insert("clear_chat_shortcut".to_string(), clear_chat_shortcut.clone());
    }

    Ok(serde_json::Value::Object(settings))
}

#[tauri::command]
fn set_settings(app: AppHandle, settings: serde_json::Value) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let path = config_dir.join("settings.dat");
    let store = match StoreBuilder::new(app.app_handle(), path).build() {
        Ok(store) => store,
        Err(e) => return Err(e.to_string()),
    };
    let _ = store.reload();

    // 保存所有设置值
    if let Some(api_key) = settings.get("api_key") {
        store.set("api_key", api_key.clone());
    }
    if let Some(api_url) = settings.get("api_url") {
        store.set("api_url", api_url.clone());
    }
    if let Some(model_name) = settings.get("model_name") {
        store.set("model_name", model_name.clone());
    }
    if let Some(shortcut) = settings.get("shortcut") {
        store.set("shortcut", shortcut.clone());
        let _ = register_shortcut(app.clone(), shortcut.as_str().unwrap().to_string());
    }
    if let Some(system_prompt) = settings.get("system_prompt") {
        store.set("system_prompt", system_prompt.clone());
    }
    if let Some(api_type) = settings.get("api_type") {
        store.set("api_type", api_type.clone());
    }
    if let Some(clear_chat_shortcut) = settings.get("clear_chat_shortcut") {
        store.set("clear_chat_shortcut", clear_chat_shortcut.clone());
    }

    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn extract_text(app: AppHandle, bytes: Vec<u8>, file_name: String) -> Result<String, String> {
    // 文件扩展名验证
    if let Err(e) = InputValidator::validate_file_extension(&file_name) {
        SecurityLogger::log_security_violation(&app, &format!("Unsupported file type: {}", e));
        return Err(e);
    }
    
    // 文件大小限制（10MB）
    if bytes.len() > 10 * 1024 * 1024 {
        SecurityLogger::log_security_violation(&app, "File too large");
        return Err("File is too large (maximum 10MB)".to_string());
    }
    
    let extension = std::path::Path::new(&file_name)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("")
        .to_lowercase();

    SecurityLogger::log_security_event_with_file(&app, &format!("File extraction attempted: {} ({} bytes)", file_name, bytes.len()), "INFO");

    match extension.as_str() {
        "pdf" => {
            pdf_extract::extract_text_from_mem(&bytes)
                .map_err(|e| {
                    SecurityLogger::log_error(&app, &format!("PDF extraction failed: {}", e));
                    e.to_string()
                })
        },
        "docx" => {
            use docx_rs::{read_docx, DocumentChild, ParagraphChild, RunChild};

            match read_docx(&bytes) {
                Ok(docx) => {
                    let mut text = String::new();
                    for child in &docx.document.children {
                        if let DocumentChild::Paragraph(p) = child {
                            for p_child in &p.children {
                                if let ParagraphChild::Run(run) = p_child {
                                    for r_child in &run.children {
                                        if let RunChild::Text(t) = r_child {
                                            text.push_str(&t.text);
                                        }
                                    }
                                }
                            }
                            text.push('\n');
                        }
                    }
                    Ok(text)
                },
                Err(e) => {
                    SecurityLogger::log_error(&app, &format!("DOCX extraction failed: {}", e));
                    Err(e.to_string())
                },
            }
        },
        // Plain text extensions
        "txt" | "md" | "json" | "csv" | "html" | "css" | "js" | "ts" | "py" | "rs" | "toml" | "yaml" | "yml" => {
            String::from_utf8(bytes).map_err(|e| {
                SecurityLogger::log_error(&app, &format!("Text file reading failed: {}", e));
                e.to_string()
            })
        },
        _ => {
            // 不应该到达这里，因为已经验证了文件扩展名
            SecurityLogger::log_security_violation(&app, &format!("Unexpected file extension: {}", extension));
            Err("Unsupported file type".to_string())
        }
    }
}

// --- AI API Logic ---

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConversationMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct AIRequest {
    model: String,
    messages: Vec<ConversationMessage>,
    stream: bool,
}

// OpenAI 格式响应
#[derive(Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<ChatChoice>,
}

// 通用格式响应（支持 OpenAI 兼容 API）
#[derive(Deserialize)]
struct GenericChatResponse {
    choices: Option<Vec<ChatChoice>>,
    message: Option<ResponseMessage>,
    content: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ChatChoice {
    message: ResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ResponseMessage {
    content: String,
}

#[tauri::command]
async fn ask_ai(app: AppHandle, messages: Vec<ConversationMessage>) -> Result<String, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let path = config_dir.join("settings.dat");
    let store = match StoreBuilder::new(app.app_handle(), path).build() {
        Ok(store) => store,
        Err(e) => return Err(e.to_string()),
    };
    let _ = store.reload();

    let api_key = store.get("api_key").map(|v| v.to_string()).unwrap_or_default();
    
    // 清理 API 密钥 - 移除可能的引号和多余空格
    let api_key = api_key.trim().to_string();
    let api_key = api_key.trim_matches('"').trim_matches('\'').to_string();
    let api_url = store.get("api_url").map(|v| v.to_string()).unwrap_or("https://api.openai.com/v1/chat/completions".to_string());
    
    // 验证 API URL
    if api_url.trim().is_empty() {
        SecurityLogger::log_security_violation(&app, "API URL is empty");
        return Err("API URL cannot be empty. Please set it in the settings.".to_string());
    }
    
    // 确保 URL 是有效的 - 移除可能的引号和多余空格
    let api_url = api_url.trim().to_string();
    let api_url = api_url.trim_matches('"').to_string();
    
    // 确保 URL 有协议前缀
    let api_url = if !api_url.starts_with("http://") && !api_url.starts_with("https://") {
        format!("https://{}", api_url)
    } else {
        api_url
    };
    
    if let Err(e) = InputValidator::validate_url(&api_url) {
        SecurityLogger::log_security_violation(&app, &format!("Invalid API URL: {}", e));
        return Err(e);
    }
    
    // 清理模型名称 - 移除可能的引号和多余空格
    let model_name = store.get("model_name").map(|v| v.to_string()).unwrap_or("gpt-4o-mini".to_string());
    let model_name = model_name.trim().to_string();
    let model_name = model_name.trim_matches('"').trim_matches('\'').to_string();

    if let Err(e) = InputValidator::validate_model_name(&model_name) {
        SecurityLogger::log_security_violation(&app, &format!("Invalid model name: {}", e));
        return Err(e);
    }
    
    // 清理系统提示 - 移除可能的引号和多余空格
    let system_prompt = store.get("system_prompt").map(|v| v.to_string()).unwrap_or("You are a helpful assistant.".to_string());
    let system_prompt = system_prompt.trim().to_string();
    let system_prompt = system_prompt.trim_matches('"').trim_matches('\'').to_string();
    
    // 清理 API 类型
    let api_type = store.get("api_type").map(|v| v.to_string()).unwrap_or("openai".to_string());
    let api_type = api_type.trim().to_string();
    let api_type = api_type.trim_matches('"').trim_matches('\'').to_string();

    // API密钥验证
    if api_key.is_empty() || api_key == "your_api_key_here" {
        SecurityLogger::log_security_violation(&app, "API key not set");
        return Err("Please set your API key in the settings.".to_string());
    }

    // 消息内容验证和清理
    let mut safe_messages = Vec::new();
    for message in messages {
        let sanitized_content = InputValidator::sanitize_system_prompt(&message.content);
        safe_messages.push(ConversationMessage {
            role: message.role,
            content: sanitized_content,
        });
    }

    let mut messages_to_send = Vec::new();
    if !system_prompt.is_empty() {
        messages_to_send.push(ConversationMessage {
            role: "system".to_string(),
            content: system_prompt.clone(),
        });
    }
    messages_to_send.extend(safe_messages);

    // 记录API请求
    SecurityLogger::log_api_request(&app, &api_url, &model_name);

    let client = Client::builder()
        .timeout(Duration::from_secs(30)) // 设置超时
        .build()
        .map_err(|e| {
            SecurityLogger::log_error(&app, &format!("Failed to create HTTP client: {}", e));
            format!("Failed to create HTTP client: {}", e)
        })?;

    let request_body = AIRequest {
        model: model_name.clone(),
        messages: messages_to_send,
        stream: false,
    };

    // 调试：记录请求详情
    println!("=== DEBUG API REQUEST ===");
    println!("API URL: {}", api_url);
    println!("Model Name: {}", model_name);
    println!("API Type: {}", api_type);
    println!("Request Body: {}", serde_json::to_string_pretty(&request_body).unwrap_or("Failed to serialize".to_string()));
    println!("=== END DEBUG ===");

    let mut request = client.post(&api_url);
    
    // 设置 Content-Type 头
    request = request.header("Content-Type", "application/json");
    
    // 根据 API 类型设置认证方式
    if api_type == "openai" {
        request = request.bearer_auth(&api_key);
    } else {
        // 对于兼容 API，直接设置 Authorization 头
        request = request.header("Authorization", format!("Bearer {}", api_key));
    }
    
    let res = request
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            SecurityLogger::log_error(&app, &format!("API request failed: {}", e));
            format!("API request failed: {}", e)
        })?;

    if res.status().is_success() {
        let response_text = res.text().await.map_err(|e| {
            SecurityLogger::log_error(&app, &format!("Failed to read response: {}", e));
            format!("Failed to read response: {}", e)
        })?;
        
        // 首先尝试解析为标准的 OpenAI 格式
        if let Ok(openai_response) = serde_json::from_str::<OpenAIChatResponse>(&response_text) {
            if let Some(choice) = openai_response.choices.get(0) {
                let content = InputValidator::sanitize_system_prompt(&choice.message.content);
                return Ok(content.trim().to_string());
            }
        }
        
        // 尝试解析为通用格式
        if let Ok(generic_response) = serde_json::from_str::<GenericChatResponse>(&response_text) {
            if let Some(choice) = generic_response.choices.and_then(|mut choices| choices.pop()) {
                let content = InputValidator::sanitize_system_prompt(&choice.message.content);
                return Ok(content.trim().to_string());
            }
            if let Some(message) = generic_response.message {
                let content = InputValidator::sanitize_system_prompt(&message.content);
                return Ok(content.trim().to_string());
            }
            if let Some(content) = generic_response.content {
                let sanitized_content = InputValidator::sanitize_system_prompt(&content);
                return Ok(sanitized_content.trim().to_string());
            }
        }
        
        // 如果无法解析为标准格式，尝试手动解析 JSON 查找常见字段
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response_text) {
            // 尝试从 choices[0].message.content 获取
            if let Some(content) = json_value["choices"][0]["message"]["content"].as_str() {
                let sanitized_content = InputValidator::sanitize_system_prompt(content);
                return Ok(sanitized_content.trim().to_string());
            }
            // 尝试从 message.content 获取
            if let Some(content) = json_value["message"]["content"].as_str() {
                let sanitized_content = InputValidator::sanitize_system_prompt(content);
                return Ok(sanitized_content.trim().to_string());
            }
            // 尝试从 content 直接获取
            if let Some(content) = json_value["content"].as_str() {
                let sanitized_content = InputValidator::sanitize_system_prompt(content);
                return Ok(sanitized_content.trim().to_string());
            }
            // 尝试从 result 获取
            if let Some(content) = json_value["result"].as_str() {
                let sanitized_content = InputValidator::sanitize_system_prompt(content);
                return Ok(sanitized_content.trim().to_string());
            }
        }
        
        // 如果无法解析为标准格式，返回原始响应用于调试
        SecurityLogger::log_error(&app, &format!("Unable to parse API response. URL: {}, Model: {}", api_url, model_name));
        Err(format!("Unable to parse API response. Please check if the API endpoint and model name are correct. Raw response: {}", response_text))
    } else {
        let status = res.status();
        let error_body = res.text().await.map_err(|e| {
            SecurityLogger::log_error(&app, &format!("Failed to read error response: {}", e));
            format!("Failed to read error response: {}", e)
        })?;
        
        // 提供更详细的错误信息
        let error_message = if status.as_u16() == 401 {
            SecurityLogger::log_security_violation(&app, "API authentication failed");
            "Authentication failed. Please check your API key."
        } else if status.as_u16() == 404 {
            SecurityLogger::log_error(&app, "API endpoint not found");
            "API endpoint not found. Please check the URL."
        } else if status.as_u16() == 429 {
            SecurityLogger::log_security_event_with_file(&app, "Rate limit exceeded", "WARNING");
            "Rate limit exceeded. Please try again later."
        } else {
            SecurityLogger::log_error(&app, &format!("API request failed with status: {}", status));
            "API request failed"
        };
        
        Err(format!("{} ({}): {}\nCheck your API endpoint URL and model name in settings.", error_message, status, error_body))
    }
}
// --- Window and App Setup ---

fn toggle_window_visibility(handle: &AppHandle) {
    if let Some(window) = handle.get_webview_window("main") {
        if let Ok(is_visible) = window.is_visible() {
            if is_visible {
                if let Err(e) = window.hide() {
                    println!("Failed to hide window: {}", e);
                }
            } else {
                if let Err(e) = window.show() {
                    println!("Failed to show window: {}", e);
                }
                if let Err(e) = window.set_focus() {
                    println!("Failed to set window focus: {}", e);
                }
            }
        }
    }
}

// --- Security Configuration ---

/// 安全检查
fn perform_security_checks(app: &AppHandle) {
    SecurityLogger::log_security_event_with_file(app, "Starting security checks", "INFO");
    
    // 检查环境变量
    if let Ok(debug) = env::var("RUST_BACKTRACE") {
        if debug == "1" || debug == "full" {
            SecurityLogger::log_security_event_with_file(app, "Debug backtrace enabled", "WARNING");
        }
    }
    
    // 检查当前工作目录
    if let Ok(current_dir) = env::current_dir() {
        SecurityLogger::log_security_event_with_file(
            app,
            &format!("Application started in: {}", current_dir.display()),
            "INFO"
        );
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    
    static LAST_TRIGGER: std::sync::Mutex<Option<Instant>> = std::sync::Mutex::new(None);
    
    let handler = |app: &AppHandle, _shortcut: &Shortcut, _event: ShortcutEvent| {
        let now = Instant::now();
        let mut last_trigger_guard = LAST_TRIGGER.lock().unwrap();
        
        // 添加防抖机制，防止快速重复触发
        if let Some(last_trigger) = *last_trigger_guard {
            if now.duration_since(last_trigger) < Duration::from_millis(200) {
                return;
            }
        }
        
        *last_trigger_guard = Some(now);
        
        // 切换窗口显示/隐藏状态
        toggle_window_visibility(app);
    };

    tauri::Builder::default()
        .setup(|app| {
            perform_security_checks(app.handle());
            let app_handle = app.handle().clone();
            let settings = get_settings(app_handle.clone()).unwrap_or_default();
            if let Some(shortcut) = settings.get("shortcut") {
                let _ = register_shortcut(app_handle, shortcut.as_str().unwrap().to_string());
            }
            Ok(())
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().with_handler(handler).build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            ask_ai,
            open_config_directory,
            extract_text,
            get_settings,
            set_settings,
            register_shortcut,
            set_decorations,
            set_minimal_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}