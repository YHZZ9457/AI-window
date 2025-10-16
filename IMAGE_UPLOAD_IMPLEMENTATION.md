# Image Upload Feature Implementation

## Overview
Add support for uploading images to AI models that support multimodal input (like GPT-4o).

## Backend Changes (Rust)

### 1. Add base64 dependency to Cargo.toml
```toml
base64 = "0.22"
```

### 2. New Tauri Commands

#### encode_image_to_base64
- Reads image file from path
- Encodes to base64 format
- Returns data URI format: `data:image/jpeg;base64,{data}`

### 3. Frontend Changes (Svelte)
- Update file type validation to include image formats
- Add image detection logic
- Format messages properly for multimodal AI models

## Implementation Steps

1. **Update file extension validation** to include image formats
2. **Create image encoding command** in Rust
3. **Update message formatting** to include base64 image data
4. **Test with GPT-4o** or other multimodal models

## Supported Image Formats
- JPEG (.jpg, .jpeg)
- PNG (.png)
- GIF (.gif)
- WebP (.webp)
- BMP (.bmp)

## Message Format for Multimodal Models
```json
{
  "role": "user",
  "content": [
    {
      "type": "text",
      "text": "请看下面这张图片（base64 编码）：
data:image/jpeg;base64,{base64_data}`
```

## Testing
- Upload a JPEG/PNG image
- Send message to GPT-4o
- Verify image content is processed correctly