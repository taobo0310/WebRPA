@echo off
chcp 65001 >nul
title 重新生成启动器图标

echo ========================================
echo      重新生成启动器图标
echo ========================================
echo.

set "ROOT_DIR=%~dp0.."
set "PYTHON_EXE=%ROOT_DIR%\Python313\python.exe"

if not exist "%PYTHON_EXE%" (
    echo [错误] 未找到 Python，请确保 Python313 目录存在
    pause
    exit /b 1
)

echo [信息] 正在生成图标...
echo.

"%PYTHON_EXE%" -c "from PIL import Image, ImageDraw; import os; import struct; icons_dir = 'src-tauri/icons'; os.makedirs(icons_dir, exist_ok=True); img = Image.new('RGBA', (512, 512), (0,0,0,0)); draw = ImageDraw.Draw(img); draw.ellipse([0,0,512,512], fill=(245,87,108,255)); box_size = 204; center = 256; line_width = 25; points = [(center, center-box_size//2), (center+box_size//2, center), (center, center+box_size//2), (center-box_size//2, center)]; draw.polygon(points, fill=(255,255,255,50), outline=(255,255,255,255), width=line_width); draw.line([points[0], points[2]], fill=(255,255,255,255), width=line_width); draw.line([points[1], points[3]], fill=(255,255,255,255), width=line_width); [img.resize((s,s), Image.Resampling.LANCZOS).save(f'{icons_dir}/{n}') for s,n in [(32,'32x32.png'),(128,'128x128.png'),(256,'128x128@2x.png'),(512,'icon.png')]]; ico_img = Image.open(f'{icons_dir}/icon.png'); icons = [ico_img.resize(s, Image.Resampling.LANCZOS) for s in [(16,16),(32,32),(48,48),(64,64),(128,128),(256,256)]]; icons[0].save(f'{icons_dir}/icon.ico', format='ICO', sizes=[(i.width,i.height) for i in icons], append_images=icons[1:]); sizes = [(16,16,b'icp4'),(32,32,b'icp5'),(64,64,b'icp6'),(128,128,b'ic07'),(256,256,b'ic08'),(512,512,b'ic09'),(1024,1024,b'ic10')]; temp_files = [(f'{icons_dir}/temp_{w}x{h}.png', t, ico_img.resize((w,h), Image.Resampling.LANCZOS)) for w,h,t in sizes]; [i.save(p, 'PNG') for p,_,i in temp_files]; f = open(f'{icons_dir}/icon.icns', 'wb'); f.write(b'icns'); icon_data = [(t, open(p,'rb').read()) for p,t,_ in temp_files]; total_size = 8 + sum(8+len(d) for _,d in icon_data); f.write(struct.pack('>I', total_size)); [f.write(t + struct.pack('>I', 8+len(d)) + d) for t,d in icon_data]; f.close(); [os.remove(p) for p,_,_ in temp_files if os.path.exists(p)]; print('图标生成完成！')"

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================
    echo      图标生成成功！
    echo ========================================
    echo.
    echo 图标位置: launcher\src-tauri\icons\
    echo.
) else (
    echo.
    echo [错误] 图标生成失败
    echo.
)

pause
