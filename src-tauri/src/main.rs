// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppConfig {
    ocr_engine: String,
    trigger_delay_ms: u32,
    hotkey: String,
    auto_copy: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ocr_engine: "Tesseract".to_string(),
            trigger_delay_ms: 300,
            hotkey: "Alt".to_string(),
            auto_copy: true,
        }
    }
}

#[tauri::command]
fn get_config() -> AppConfig {
    // TODO: 从文件加载配置
    AppConfig::default()
}

#[tauri::command]
fn update_config(config: AppConfig) -> Result<(), String> {
    // TODO: 保存配置到文件
    println!("配置已更新: {:?}", config);
    Ok(())
}

#[tauri::command]
fn trigger_screenshot() -> Result<Vec<u8>, String> {
    // TODO: 实现截图功能
    Err("功能开发中".to_string())
}

#[tauri::command]
fn perform_ocr(image_data: Vec<u8>) -> Result<String, String> {
    // TODO: 实现OCR识别
    Err("功能开发中".to_string())
}

fn main() {
    // 创建系统托盘菜单
    let settings = CustomMenuItem::new("settings".to_string(), "设置");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);
    
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                // 左键点击显示主窗口
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "settings" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            update_config,
            trigger_screenshot,
            perform_ocr
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

