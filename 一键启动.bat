@echo off
chcp 65001 >nul
echo ============================================
echo   ScreenOCR-Tauri 一键启动脚本
echo ============================================
echo.

cd /d "%~dp0"

echo [1/3] 检查 Node.js 环境...
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ 未检测到 Node.js，请先安装 Node.js
    echo    下载地址: https://nodejs.org/
    pause
    exit /b 1
)
echo ✅ Node.js 环境正常
echo.

echo [2/3] 检查依赖...
if not exist "node_modules\" (
    echo ⏳ 首次运行，正在安装依赖...
    echo    这可能需要几分钟，请耐心等待...
    echo.
    call npm install
    if %errorlevel% neq 0 (
        echo ❌ 依赖安装失败
        pause
        exit /b 1
    )
    echo ✅ 依赖安装完成
) else (
    echo ✅ 依赖已存在
)
echo.

echo [3/3] 启动开发环境...
echo ⏳ 正在编译 Rust 代码并启动 Vite...
echo    首次编译可能需要 3-5 分钟
echo.
call npm run tauri:dev

pause

