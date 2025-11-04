@echo off
chcp 65001 >nul
echo ======================================
echo   ScreenOCR Tauri - 安装依赖
echo ======================================
echo.

cd /d "%~dp0"

echo [1/2] 检查 Node.js...
where node >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ❌ 未检测到 Node.js，请先安装: https://nodejs.org/
    pause
    exit /b 1
)
node --version
npm --version
echo ✅ Node.js 已安装
echo.

echo [2/2] 安装 npm 依赖...
echo.
echo 正在安装，请稍候...
echo.

npm install

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ 依赖安装成功！
    echo.
    echo 下一步：运行 "启动开发环境.bat" 启动项目
) else (
    echo.
    echo ❌ 安装失败！
    echo.
    echo 如果遇到网络问题，可以使用淘宝镜像：
    echo npm install --registry=https://registry.npmmirror.com
)

echo.
pause

