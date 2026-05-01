# 图标文件说明

请将以下图标文件放置在此目录:

- `32x32.png` - 32x32 像素的 PNG 图标
- `128x128.png` - 128x128 像素的 PNG 图标
- `128x128@2x.png` - 256x256 像素的 PNG 图标 (高分辨率)
- `icon.icns` - macOS 图标文件
- `icon.ico` - Windows 图标文件

## 生成图标

可以使用在线工具或以下命令生成:

```bash
# 使用 Tauri CLI 生成图标
npm run tauri icon path/to/your/icon.png
```

## 临时方案

如果暂时没有图标,可以使用 Tauri 的默认图标,或者从以下网站下载:

- https://www.flaticon.com/
- https://icons8.com/
- https://www.iconfinder.com/

建议图标设计:
- 使用 WebRPA 的品牌色
- 简洁现代的设计风格
- 清晰可辨的图形元素
