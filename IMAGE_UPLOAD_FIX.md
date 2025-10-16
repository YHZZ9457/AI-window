# Image Upload Feature Implementation

## Issues Identified
1. File type validation in Rust backend doesn't include image formats
2. Frontend file processing needs to handle images differently
3. Need to add image encoding command to invoke handler

## Solution

### 1. Update Rust Backend

Add the following command to `src-tauri/src/lib.rs`:

```rust
// --- Image Processing ---

#[tauri::command]
fn encode_image_to_base64(app: AppHandle, path: String) -> Result<String, String> {
    use std::fs;
    
    // 验证文件扩展名
    if let Err(e) = InputValidator::validate_file_extension(&path) {
        return Err(e);
    }
    
    // 读取文件并编码为base64
    let data = fs::read(&path).map_err(|e| e.to_string())?;
    
    // 文件大小限制（5MB）
    if data.len() > 5 * 1024 * 1024 {
        return Err("Image file is too large (maximum 5MB)".to_string());
    }
    
    // 获取MIME类型
    let mime_type = match std::path::Path::new(&path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("")
        .to_lowercase()
        .as_str() 
    {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        _ => "application/octet-stream"
    };
    
    // 编码为base64
    let base64_data = base64::engine::general_purpose::STANDARD.encode(&data);
    
    // 记录图像处理事件
    SecurityLogger::log_security_event_with_file(&app, &format!("Image encoded: {} ({} bytes)", path, data.len()), "INFO");
    
    Ok(format!("data:{};base64,{}", mime_type, base64_data))
}
```

### 2. Update Frontend

Add image format support to the allowed extensions in `src/routes/+page.svelte`:

```javascript
const allowedExtensions = [
  '.txt', '.md', '.json', '.csv',
  '.html', '.css', '.js', '.ts', '.py', '.rs',
  '.pdf', '.docx',
  '.jpg', '.jpeg', '.png', '.gif', '.webp', '.bmp'
];
```

## Testing
- Upload a JPEG/PNG image
- The image should be encoded to base64
- The AI should be able to process the image content