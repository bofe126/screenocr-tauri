use global_hotkey::{
    hotkey::{HotKey, Modifiers, Code},
    GlobalHotKeyManager, GlobalHotKeyEvent,
};
use std::sync::Arc;
use tauri::AppHandle;

pub struct HotKeyHandler {
    manager: Arc<GlobalHotKeyManager>,
    current_hotkey: Option<HotKey>,
}

impl HotKeyHandler {
    pub fn new() -> Result<Self, String> {
        let manager = GlobalHotKeyManager::new()
            .map_err(|e| format!("初始化热键管理器失败: {}", e))?;
        
        Ok(Self {
            manager: Arc::new(manager),
            current_hotkey: None,
        })
    }
    
    /// 注册热键
    pub fn register(&mut self, hotkey_str: &str) -> Result<(), String> {
        // 先注销旧热键
        if let Some(old_hotkey) = self.current_hotkey.take() {
            let _ = self.manager.unregister(old_hotkey);
        }
        
        // 解析并注册新热键
        let hotkey = Self::parse_hotkey(hotkey_str)?;
        
        self.manager.register(hotkey)
            .map_err(|e| format!("注册热键失败: {}", e))?;
        
        self.current_hotkey = Some(hotkey);
        println!("✅ 热键已注册: {}", hotkey_str);
        
        Ok(())
    }
    
    /// 注销热键
    pub fn unregister(&mut self) -> Result<(), String> {
        if let Some(hotkey) = self.current_hotkey.take() {
            self.manager.unregister(hotkey)
                .map_err(|e| format!("注销热键失败: {}", e))?;
            println!("🔓 热键已注销");
        }
        Ok(())
    }
    
    /// 解析热键字符串 (例如: "Alt", "Ctrl+Shift+A", "Alt+F1")
    fn parse_hotkey(hotkey_str: &str) -> Result<HotKey, String> {
        let parts: Vec<&str> = hotkey_str.split('+').map(|s| s.trim()).collect();
        
        if parts.is_empty() {
            return Err("热键不能为空".to_string());
        }
        
        let mut modifiers = Modifiers::empty();
        let mut key_code: Option<Code> = None;
        
        for part in parts {
            match part.to_uppercase().as_str() {
                "CTRL" | "CONTROL" => modifiers |= Modifiers::CONTROL,
                "ALT" => modifiers |= Modifiers::ALT,
                "SHIFT" => modifiers |= Modifiers::SHIFT,
                "SUPER" | "WIN" | "CMD" => modifiers |= Modifiers::SUPER,
                // 字母键
                "A" => key_code = Some(Code::KeyA),
                "B" => key_code = Some(Code::KeyB),
                "C" => key_code = Some(Code::KeyC),
                "D" => key_code = Some(Code::KeyD),
                "E" => key_code = Some(Code::KeyE),
                "F" => key_code = Some(Code::KeyF),
                "G" => key_code = Some(Code::KeyG),
                "H" => key_code = Some(Code::KeyH),
                "I" => key_code = Some(Code::KeyI),
                "J" => key_code = Some(Code::KeyJ),
                "K" => key_code = Some(Code::KeyK),
                "L" => key_code = Some(Code::KeyL),
                "M" => key_code = Some(Code::KeyM),
                "N" => key_code = Some(Code::KeyN),
                "O" => key_code = Some(Code::KeyO),
                "P" => key_code = Some(Code::KeyP),
                "Q" => key_code = Some(Code::KeyQ),
                "R" => key_code = Some(Code::KeyR),
                "S" => key_code = Some(Code::KeyS),
                "T" => key_code = Some(Code::KeyT),
                "U" => key_code = Some(Code::KeyU),
                "V" => key_code = Some(Code::KeyV),
                "W" => key_code = Some(Code::KeyW),
                "X" => key_code = Some(Code::KeyX),
                "Y" => key_code = Some(Code::KeyY),
                "Z" => key_code = Some(Code::KeyZ),
                // 功能键
                "F1" => key_code = Some(Code::F1),
                "F2" => key_code = Some(Code::F2),
                "F3" => key_code = Some(Code::F3),
                "F4" => key_code = Some(Code::F4),
                "F5" => key_code = Some(Code::F5),
                "F6" => key_code = Some(Code::F6),
                "F7" => key_code = Some(Code::F7),
                "F8" => key_code = Some(Code::F8),
                "F9" => key_code = Some(Code::F9),
                "F10" => key_code = Some(Code::F10),
                "F11" => key_code = Some(Code::F11),
                "F12" => key_code = Some(Code::F12),
                // 数字键
                "0" => key_code = Some(Code::Digit0),
                "1" => key_code = Some(Code::Digit1),
                "2" => key_code = Some(Code::Digit2),
                "3" => key_code = Some(Code::Digit3),
                "4" => key_code = Some(Code::Digit4),
                "5" => key_code = Some(Code::Digit5),
                "6" => key_code = Some(Code::Digit6),
                "7" => key_code = Some(Code::Digit7),
                "8" => key_code = Some(Code::Digit8),
                "9" => key_code = Some(Code::Digit9),
                // 特殊键
                "SPACE" => key_code = Some(Code::Space),
                "ENTER" => key_code = Some(Code::Enter),
                "TAB" => key_code = Some(Code::Tab),
                "BACKSPACE" => key_code = Some(Code::Backspace),
                "ESC" | "ESCAPE" => key_code = Some(Code::Escape),
                _ => return Err(format!("不支持的按键: {}", part)),
            }
        }
        
        let code = key_code.ok_or_else(|| "未指定按键".to_string())?;
        
        Ok(HotKey::new(Some(modifiers), code))
    }
    
    /// 启动热键事件监听
    pub fn start_listener(app: AppHandle) {
        std::thread::spawn(move || {
            let receiver = GlobalHotKeyEvent::receiver();
            loop {
                if let Ok(event) = receiver.recv() {
                    println!("🔥 热键触发: {:?}", event);
                    
                    // 触发 OCR
                    let app_clone = app.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Err(e) = app_clone.emit_all("hotkey-triggered", ()) {
                            eprintln!("❌ 发送热键事件失败: {}", e);
                        }
                    });
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_key() {
        let hotkey = HotKeyHandler::parse_hotkey("Alt").unwrap();
        assert!(hotkey.mods.contains(Modifiers::ALT));
    }
    
    #[test]
    fn test_parse_combination() {
        let hotkey = HotKeyHandler::parse_hotkey("Ctrl+Shift+A").unwrap();
        assert!(hotkey.mods.contains(Modifiers::CONTROL));
        assert!(hotkey.mods.contains(Modifiers::SHIFT));
    }
    
    #[test]
    fn test_parse_invalid() {
        let result = HotKeyHandler::parse_hotkey("InvalidKey");
        assert!(result.is_err());
    }
}

