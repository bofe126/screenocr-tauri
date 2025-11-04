@echo off
chcp 65001 >nul
echo ======================================
echo   ScreenOCR Tauri - 启动开发环境
echo ======================================
echo.

cd /d "%~dp0"

echo [1/3] 检查 Node.js...
where node >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ❌ 未检测到 Node.js，请先安装: https://nodejs.org/
    pause
    exit /b 1
)
node --version
echo ✅ Node.js 已安装
echo.

echo [2/3] 检查 Rust...
where rustc >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ❌ 未检测到 Rust，请先安装: https://rustup.rs/
    pause
    exit /b 1
)
rustc --version
echo ✅ Rust 已安装
echo.

echo [3/3] 启动开发服务器...
echo.
echo ⚠️  首次启动需要 3-5 分钟编译 Rust 依赖
echo ⚠️  后续启动会快很多（10-20秒）
echo.
echo 正在启动...
echo.

npm run tauri:dev

pause

