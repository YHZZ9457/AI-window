use tauri::{Manager, AppHandle, State};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent};
use tauri_plugin_opener::OpenerExt;

// --- Settings Management ---

#[derive(Serialize, Deserialize, Clone)]
struct AppSettings {
    api_key: String,
    api_url: String,
    model_name: String,
    shortcut: String,
    system_prompt: String,
    api_type: String, // "openai" or "openai-compatible"
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            api_key: "your_api_key_here".to_string(),
            api_url: "https://api.openai.com/v1/chat/completions".to_string(),
            model_name: "gpt-4o-mini".to_string(),
            shortcut: "Alt+Space".to_string(),
            system_prompt: "You are a helpful assistant. Please answer questions as concisely and effectively as possible.".to_string(),
            api_type: "openai".to_string(),
        }
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
                if settings.api_url.is_empty() { settings.api_url = default.api_url; }
                if settings.model_name.is_empty() { settings.model_name = default.model_name; }
                if settings.shortcut.is_empty() { settings.shortcut = default.shortcut; }
                if settings.system_prompt.is_empty() { settings.system_prompt = default.system_prompt; }
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
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

// --- Tauri Commands ---

#[tauri::command]
fn get_settings(state: State<SettingsState>) -> Result<AppSettings, String> {
    Ok(state.0.lock().unwrap().clone())
}

#[tauri::command]
fn set_settings(app: AppHandle, settings: AppSettings, state: State<SettingsState>) -> Result<(), String> {
    let old_shortcut = state.0.lock().unwrap().shortcut.clone();
    app.global_shortcut().unregister(old_shortcut.as_str()).unwrap_or_else(|e| println!("Failed to unregister old shortcut: {}", e));

    let new_shortcut = settings.shortcut.clone();
    app.global_shortcut().register(new_shortcut.as_str()).map_err(|e| {
        app.global_shortcut().register(old_shortcut.as_str()).unwrap_or_else(|e2| println!("Failed to re-register old shortcut: {}", e2));
        format!("Failed to register new shortcut '{}': {}", new_shortcut, e)
    })?;

    save_settings(&app, &settings)?;
    *state.0.lock().unwrap() = settings;
    Ok(())
}

#[tauri::command]
fn open_config_directory(app: AppHandle) -> Result<(), String> {
    let path = app.path().app_config_dir().map_err(|e| e.to_string())?;
    app.opener().open_url(path.to_string_lossy(), None::<String>).map_err(|e| e.to_string())
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
async fn ask_ai(messages: Vec<ConversationMessage>, state: State<'_, SettingsState>) -> Result<String, String> {
    let settings = state.0.lock().unwrap().clone();

    if settings.api_key.is_empty() || settings.api_key == "your_api_key_here" {
        return Err("Please set your API key in the settings.".to_string());
    }

    let mut messages_to_send = Vec::new();
    if !settings.system_prompt.is_empty() {
        messages_to_send.push(ConversationMessage {
            role: "system".to_string(),
            content: settings.system_prompt,
        });
    }
    messages_to_send.extend(messages);

    let client = Client::new();
    let request_body = AIRequest {
        model: settings.model_name,
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
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let response_text = res.text().await.map_err(|e| e.to_string())?;
        
        // 首先尝试解析为标准的 OpenAI 格式
        if let Ok(openai_response) = serde_json::from_str::<OpenAIChatResponse>(&response_text) {
            if let Some(choice) = openai_response.choices.get(0) {
                return Ok(choice.message.content.trim().to_string());
            }
        }
        
        // 尝试解析为通用格式
        if let Ok(generic_response) = serde_json::from_str::<GenericChatResponse>(&response_text) {
            if let Some(choice) = generic_response.choices.and_then(|mut choices| choices.pop()) {
                return Ok(choice.message.content.trim().to_string());
            }
            if let Some(message) = generic_response.message {
                return Ok(message.content.trim().to_string());
            }
            if let Some(content) = generic_response.content {
                return Ok(content.trim().to_string());
            }
        }
        
        // 如果无法解析为标准格式，尝试手动解析 JSON 查找常见字段
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response_text) {
            // 尝试从 choices[0].message.content 获取
            if let Some(content) = json_value["choices"][0]["message"]["content"].as_str() {
                return Ok(content.trim().to_string());
            }
            // 尝试从 message.content 获取
            if let Some(content) = json_value["message"]["content"].as_str() {
                return Ok(content.trim().to_string());
            }
            // 尝试从 content 直接获取
            if let Some(content) = json_value["content"].as_str() {
                return Ok(content.trim().to_string());
            }
            // 尝试从 result 获取
            if let Some(content) = json_value["result"].as_str() {
                return Ok(content.trim().to_string());
            }
        }
        
        // 如果无法解析为标准格式，返回原始响应用于调试
        Err(format!("Unable to parse API response. Please check if the API endpoint and model name are correct. Raw response: {}", response_text))
    } else {
        let status = res.status();
        let error_body = res.text().await.map_err(|e| e.to_string())?;
        
        // 提供更详细的错误信息
        let error_message = if status.as_u16() == 401 {
            "Authentication failed. Please check your API key."
        } else if status.as_u16() == 404 {
            "API endpoint not found. Please check the URL."
        } else if status.as_u16() == 429 {
            "Rate limit exceeded. Please try again later."
        } else {
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
                window.hide().unwrap();
            } else {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let handler = |app: &AppHandle, _shortcut: &Shortcut, _event: ShortcutEvent| {
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
            
            let config_dir = app.path().app_config_dir().unwrap();
            if !config_dir.exists() {
                fs::create_dir_all(&config_dir).unwrap();
            }

            let settings = load_settings(&handle);
            let state: State<SettingsState> = app.state();
            *state.0.lock().unwrap() = settings.clone();

            let shortcut_to_register = settings.shortcut;
            handle.global_shortcut().register(shortcut_to_register.as_str()).unwrap_or_else(|err| {
                println!("Failed to register global shortcut '{}': {}", shortcut_to_register, err);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ask_ai,
            get_settings,
            set_settings,
            open_config_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}