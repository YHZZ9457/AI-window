const fs = require('fs');

try {
  const content = fs.readFileSync('./src-tauri/tauri.conf.json', 'utf8');
  JSON.parse(content);
  console.log('JSON is valid');
} catch (e) {
  console.log('JSON error:', e.message);
}