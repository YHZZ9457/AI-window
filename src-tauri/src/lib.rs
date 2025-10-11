use tauri::{Manager, AppHandle, State};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent};
use tauri_plugin_opener::OpenerExt;
use url::Url;
use regex::Regex;
use std::io::Write;
use std::env;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

// --- Settings Management ---

#[derive(Serialize, Deserialize, Clone)]
struct AppSettings {
    #[serde(skip_serializing)]
    api_key: String,
    encrypted_api_key: String,
    api_url: String,
    model_name: String,
    shortcut: String,
    system_prompt: String,
    api_type: String, // "openai" or "openai-compatible"
    system_prompt_preset: String, // "default", "minimal", or "custom"
}

impl Default for AppSettings {
    fn default() -> Self {
        let default_api_key = "your_api_key_here".to_string();
        let encrypted_key = BASE64.encode(&default_api_key);
        
        Self {
            api_key: default_api_key,
            encrypted_api_key: encrypted_key,
            api_url: "https://api.openai.com/v1/chat/completions".to_string(),
            model_name: "gpt-4o-mini".to_string(),
            shortcut: "Alt+Space".to_string(),
            system_prompt: "You are a helpful assistant.".to_string(),
            api_type: "openai".to_string(),
            system_prompt_preset: "default".to_string(),
        }
    }
}

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
    fn log_security_event(event: &str, level: &str) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] [{}] {}\n", timestamp, level, event);
        
        // 同时在控制台输出（仅调试模式）
        #[cfg(debug_assertions)]
        println!("Security Event: {}", log_entry.trim());
    }
    
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

struct SettingsState(Mutex<AppSettings>);

fn get_settings_path(app: &AppHandle) -> PathBuf {
    app.path().app_config_dir().unwrap().join("settings.json")
}

fn load_settings(app: &AppHandle) -> AppSettings {
    let path = get_settings_path(app);
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        let loaded: Result<AppSettings, _> = serde_json::from_str(&content);
        match loaded {
            Ok(mut settings) => {
                let default = AppSettings::default();
                
                // 解密API密钥
                if let Ok(bytes) = BASE64.decode(&settings.encrypted_api_key) {
                    if let Ok(decrypted_key) = String::from_utf8(bytes) {
                        settings.api_key = decrypted_key;
                    } else {
                        settings.api_key = default.api_key.clone();
                        settings.encrypted_api_key = default.encrypted_api_key.clone();
                    }
                } else {
                    settings.api_key = default.api_key.clone();
                    settings.encrypted_api_key = default.encrypted_api_key.clone();
                }
                
                if settings.api_url.is_empty() { settings.api_url = default.api_url; }
                if settings.model_name.is_empty() { settings.model_name = default.model_name; }
                if settings.shortcut.is_empty() { settings.shortcut = default.shortcut; }
                if settings.system_prompt.is_empty() { settings.system_prompt = default.system_prompt; }
                if settings.system_prompt_preset.is_empty() { settings.system_prompt_preset = default.system_prompt_preset; }
                
                settings
            },
            Err(_) => AppSettings::default(),
        }
    } else {
        AppSettings::default()
    }
}

fn save_settings(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let path = get_settings_path(app);
    
    // 创建要保存的设置副本，加密API密钥
    let mut settings_to_save = settings.clone();
    settings_to_save.encrypted_api_key = BASE64.encode(&settings.api_key);
    settings_to_save.api_key = String::new(); // 清除明文API密钥
    
    let json = serde_json::to_string_pretty(&settings_to_save).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

// --- Tauri Commands ---

#[tauri::command]
fn get_settings(state: State<SettingsState>) -> Result<AppSettings, String> {
    Ok(state.0.lock().unwrap().clone())
}

#[tauri::command]
fn set_settings(app: AppHandle, settings: AppSettings, state: State<SettingsState>) -> Result<(), String> {
    // 输入验证
    if let Err(e) = InputValidator::validate_url(&settings.api_url) {
        SecurityLogger::log_security_violation(&app, &format!("Invalid URL in settings: {}", e));
        return Err(e);
    }
    
    if let Err(e) = InputValidator::validate_model_name(&settings.model_name) {
        SecurityLogger::log_security_violation(&app, &format!("Invalid model name: {}", e));
        return Err(e);
    }
    
    // 清理系统提示
    let sanitized_prompt = InputValidator::sanitize_system_prompt(&settings.system_prompt);
    let mut safe_settings = settings;
    safe_settings.system_prompt = sanitized_prompt;
    
    let old_shortcut = state.0.lock().unwrap().shortcut.clone();
    app.global_shortcut().unregister(old_shortcut.as_str()).unwrap_or_else(|e| {
        SecurityLogger::log_error(&app, &format!("Failed to unregister old shortcut: {}", e));
    });

    let new_shortcut = safe_settings.shortcut.clone();
    app.global_shortcut().register(new_shortcut.as_str()).map_err(|e| {
        app.global_shortcut().register(old_shortcut.as_str()).unwrap_or_else(|e2| {
            SecurityLogger::log_error(&app, &format!("Failed to re-register old shortcut: {}", e2));
        });
        SecurityLogger::log_error(&app, &format!("Failed to register new shortcut '{}': {}", new_shortcut, e));
        format!("Failed to register new shortcut '{}': {}", new_shortcut, e)
    })?;

    save_settings(&app, &safe_settings)?;
    *state.0.lock().unwrap() = safe_settings;
    
    SecurityLogger::log_security_event_with_file(&app, "Settings updated successfully", "INFO");
    Ok(())
}

#[tauri::command]
fn open_config_directory(app: AppHandle) -> Result<(), String> {
    let path = app.path().app_config_dir().map_err(|e| e.to_string())?;
    app.opener().open_url(path.to_string_lossy(), None::<String>).map_err(|e| e.to_string())
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
async fn ask_ai(app: AppHandle, messages: Vec<ConversationMessage>, state: State<'_, SettingsState>) -> Result<String, String> {
    let settings = state.0.lock().unwrap().clone();

    // API密钥验证
    if settings.api_key.is_empty() || settings.api_key == "your_api_key_here" {
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
    if !settings.system_prompt.is_empty() {
        messages_to_send.push(ConversationMessage {
            role: "system".to_string(),
            content: settings.system_prompt.clone(),
        });
    }
    messages_to_send.extend(safe_messages);

    // 记录API请求
    SecurityLogger::log_api_request(&app, &settings.api_url, &settings.model_name);

    let client = Client::builder()
        .timeout(Duration::from_secs(30)) // 设置超时
        .build()
        .map_err(|e| {
            SecurityLogger::log_error(&app, &format!("Failed to create HTTP client: {}", e));
            format!("Failed to create HTTP client: {}", e)
        })?;

    let request_body = AIRequest {
        model: settings.model_name.clone(),
        messages: messages_to_send,
    };

    let mut request = client.post(&settings.api_url);
    
    // 根据 API 类型设置认证方式
    if settings.api_type == "openai" {
        request = request.bearer_auth(&settings.api_key);
    } else {
        // 对于兼容 API，通常使用 Bearer 认证或 API Key 头
        request = request.bearer_auth(&settings.api_key);
        // 添加一些常见的兼容 API 头
        request = request.header("Content-Type", "application/json");
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
        SecurityLogger::log_error(&app, &format!("Unable to parse API response. URL: {}, Model: {}", settings.api_url, settings.model_name));
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
        .plugin(tauri_plugin_global_shortcut::Builder::new().with_handler(handler).build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(SettingsState(Mutex::new(AppSettings::default())))
        .setup(|app| {
            let handle = app.handle().clone();
            
            // 执行安全检查
            perform_security_checks(&handle);
            
            let config_dir = app.path().app_config_dir().unwrap();
            if !config_dir.exists() {
                fs::create_dir_all(&config_dir).unwrap();
            }

            let settings = load_settings(&handle);
            let state: State<SettingsState> = app.state();
            *state.0.lock().unwrap() = settings.clone();

            let shortcut_to_register = settings.shortcut;
            handle.global_shortcut().register(shortcut_to_register.as_str()).unwrap_or_else(|err| {
                SecurityLogger::log_error(&handle, &format!("Failed to register global shortcut '{}': {}", shortcut_to_register, err));
            });

            SecurityLogger::log_security_event_with_file(&handle, "Application started successfully", "INFO");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ask_ai,
            get_settings,
            set_settings,
            open_config_directory,
            extract_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}