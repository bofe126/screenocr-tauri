// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod screenshot;
mod ocr;
mod config;
mod hotkey;

use serde::Serialize;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, Manager, ClipboardManager};
use std::sync::Mutex;
use config::AppConfig;
use hotkey::HotKeyHandler;

// 全局配置状态
struct AppState {
    config: Mutex<AppConfig>,
    hotkey_handler: Mutex<HotKeyHandler>,
}

#[tauri::command]
fn get_config(state: tauri::State<AppState>) -> AppConfig {
    let config = state.config.lock().unwrap();
    config.clone()
}

#[tauri::command]
fn update_config(config: AppConfig, state: tauri::State<AppState>) -> Result<(), String> {
    // 保存到文件
    config.save().map_err(|e| format!("保存配置失败: {}", e))?;
    
    // 更新热键
    let mut hotkey_handler = state.hotkey_handler.lock().unwrap();
    hotkey_handler.register(&config.hotkey)?;
    
    // 更新内存中的配置
    let mut app_config = state.config.lock().unwrap();
    *app_config = config.clone();
    
    println!("✅ 配置已更新并保存: {:?}", config);
    Ok(())
}

#[tauri::command]
fn reset_config(state: tauri::State<AppState>) -> Result<AppConfig, String> {
    let config = AppConfig::reset().map_err(|e| format!("重置配置失败: {}", e))?;
    
    let mut app_config = state.config.lock().unwrap();
    *app_config = config.clone();
    
    println!("🔄 配置已重置");
    Ok(config)
}

#[tauri::command]
async fn capture_screen() -> Result<CaptureResponse, String> {
    println!("📸 开始屏幕截图...");
    
    let capture = screenshot::capture_primary_screen()
        .map_err(|e| format!("截图失败: {}", e))?;
    
    let base64_image = screenshot::to_base64_png(&capture)
        .map_err(|e| format!("图像转换失败: {}", e))?;
    
    println!("✅ 截图完成: {}x{}", capture.width, capture.height);
    
    Ok(CaptureResponse {
        width: capture.width,
        height: capture.height,
        image_base64: base64_image,
    })
}

#[tauri::command]
async fn perform_ocr_on_screen(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<OcrResponse, String> {
    println!("🔍 开始 OCR 识别...");
    
    // 获取配置
    let config = {
        let cfg = state.config.lock().unwrap();
        cfg.clone()
    };
    
    // 添加延时
    if config.trigger_delay_ms > 0 {
        tokio::time::sleep(tokio::time::Duration::from_millis(config.trigger_delay_ms as u64)).await;
    }
    
    // 截图
    let capture = screenshot::capture_primary_screen()
        .map_err(|e| format!("截图失败: {}", e))?;
    
    println!("📸 截图完成: {}x{}", capture.width, capture.height);
    
    // OCR 识别
    let engine = match config.ocr_engine.as_str() {
        "WeChatOCR" => ocr::OcrEngine::WeChatOCR,
        _ => ocr::OcrEngine::Tesseract,
    };
    
    let ocr_result = ocr::perform_ocr(&capture, engine).await
        .map_err(|e| format!("OCR 识别失败: {}", e))?;
    
    println!("✅ OCR 完成，识别了 {} 个字符", ocr_result.text.len());
    
    // 自动复制到剪贴板
    if config.auto_copy && !ocr_result.text.is_empty() {
        if let Err(e) = app.clipboard_manager().write_text(ocr_result.text.clone()) {
            eprintln!("⚠️  复制到剪贴板失败: {}", e);
        } else {
            println!("📋 已复制到剪贴板");
        }
    }
    
    Ok(OcrResponse {
        text: ocr_result.text,
        confidence: ocr_result.confidence,
        language: ocr_result.language,
    })
}

#[tauri::command]
async fn perform_ocr_on_region(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<OcrResponse, String> {
    println!("🔍 开始区域 OCR 识别: {}x{} at ({}, {})", width, height, x, y);
    
    // 获取配置
    let config = {
        let cfg = state.config.lock().unwrap();
        cfg.clone()
    };
    
    // 截图全屏
    let capture = screenshot::capture_primary_screen()
        .map_err(|e| format!("截图失败: {}", e))?;
    
    println!("📸 截图完成: {}x{}", capture.width, capture.height);
    
    // 裁剪区域
    let region = screenshot::CaptureRegion {
        x,
        y,
        width,
        height,
    };
    
    let cropped = screenshot::crop_region(&capture, &region)
        .map_err(|e| format!("裁剪区域失败: {}", e))?;
    
    println!("✂️  区域裁剪完成: {}x{}", cropped.width, cropped.height);
    
    // OCR 识别
    let engine = match config.ocr_engine.as_str() {
        "WeChatOCR" => ocr::OcrEngine::WeChatOCR,
        _ => ocr::OcrEngine::Tesseract,
    };
    
    let ocr_result = ocr::perform_ocr(&cropped, engine).await
        .map_err(|e| format!("OCR 识别失败: {}", e))?;
    
    println!("✅ OCR 完成，识别了 {} 个字符", ocr_result.text.len());
    
    // 自动复制到剪贴板
    if config.auto_copy && !ocr_result.text.is_empty() {
        if let Err(e) = app.clipboard_manager().write_text(ocr_result.text.clone()) {
            eprintln!("⚠️  复制到剪贴板失败: {}", e);
        } else {
            println!("📋 已复制到剪贴板");
        }
    }
    
    Ok(OcrResponse {
        text: ocr_result.text,
        confidence: ocr_result.confidence,
        language: ocr_result.language,
    })
}

#[derive(Debug, Serialize)]
struct CaptureResponse {
    width: u32,
    height: u32,
    image_base64: String,
}

#[derive(Debug, Serialize)]
struct OcrResponse {
    text: String,
    confidence: f32,
    language: String,
}

fn main() {
    // 创建系统托盘菜单
    let settings = CustomMenuItem::new("settings".to_string(), "设置");
    let ocr_now = CustomMenuItem::new("ocr_now".to_string(), "立即识别");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    
    let tray_menu = SystemTrayMenu::new()
        .add_item(ocr_now)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);
    
    let system_tray = SystemTray::new().with_menu(tray_menu);

    // 加载配置
    let initial_config = AppConfig::load().unwrap_or_else(|e| {
        eprintln!("⚠️  加载配置失败，使用默认配置: {}", e);
        AppConfig::default()
    });
    
    println!("📋 当前配置: {:?}", initial_config);
    
    // 初始化热键处理器
    let mut hotkey_handler = HotKeyHandler::new()
        .expect("初始化热键管理器失败");
    
    // 注册初始热键
    if let Err(e) = hotkey_handler.register(&initial_config.hotkey) {
        eprintln!("⚠️  注册热键失败: {}", e);
    }
    
    tauri::Builder::default()
        .manage(AppState {
            config: Mutex::new(initial_config),
            hotkey_handler: Mutex::new(hotkey_handler),
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                // 左键点击显示主窗口
                if let Some(window) = app.get_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "settings" => {
                    if let Some(window) = app.get_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "ocr_now" => {
                    // 触发 OCR 识别
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        match perform_ocr_on_screen(
                            app_handle.clone(),
                            app_handle.state::<AppState>(),
                        ).await {
                            Ok(result) => {
                                println!("✅ OCR 成功: {} 个字符", result.text.len());
                                // 可以发送通知或显示结果窗口
                            }
                            Err(e) => {
                                eprintln!("❌ OCR 失败: {}", e);
                            }
                        }
                    });
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            // 启动热键监听
            let app_handle = app.handle();
            HotKeyHandler::start_listener(app_handle.clone());
            
            // 监听热键触发事件
            let app_handle_clone = app_handle.clone();
            app.listen_global("hotkey-triggered", move |_event| {
                let app_clone = app_handle_clone.clone();
                tauri::async_runtime::spawn(async move {
                    println!("🎯 热键触发，开始 OCR...");
                    match perform_ocr_on_screen(
                        app_clone.clone(),
                        app_clone.state::<AppState>(),
                    ).await {
                        Ok(result) => {
                            println!("✅ OCR 成功: {} 个字符", result.text.len());
                            // 发送结果到前端
                            let _ = app_clone.emit_all("ocr-result", result);
                        }
                        Err(e) => {
                            eprintln!("❌ OCR 失败: {}", e);
                            let _ = app_clone.emit_all("ocr-error", e);
                        }
                    }
                });
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            update_config,
            reset_config,
            capture_screen,
            perform_ocr_on_screen,
            perform_ocr_on_region
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

