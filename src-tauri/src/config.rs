use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub ocr_engine: String,
    pub trigger_delay_ms: u32,
    pub hotkey: String,
    pub auto_copy: bool,
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

impl AppConfig {
    /// 获取配置文件路径
    pub fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("无法获取配置目录")?
            .join("screenocr-tauri");
        
        // 确保配置目录存在
        fs::create_dir_all(&config_dir)
            .context("创建配置目录失败")?;
        
        Ok(config_dir.join("config.json"))
    }
    
    /// 从文件加载配置
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            println!("📝 配置文件不存在，使用默认配置");
            return Ok(Self::default());
        }
        
        let content = fs::read_to_string(&config_path)
            .context("读取配置文件失败")?;
        
        let config: Self = serde_json::from_str(&content)
            .context("解析配置文件失败")?;
        
        println!("✅ 配置已从文件加载: {:?}", config_path);
        Ok(config)
    }
    
    /// 保存配置到文件
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        
        let content = serde_json::to_string_pretty(self)
            .context("序列化配置失败")?;
        
        fs::write(&config_path, content)
            .context("写入配置文件失败")?;
        
        println!("💾 配置已保存到文件: {:?}", config_path);
        Ok(())
    }
    
    /// 重置为默认配置
    pub fn reset() -> Result<Self> {
        let config = Self::default();
        config.save()?;
        println!("🔄 配置已重置为默认值");
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.ocr_engine, "Tesseract");
        assert_eq!(config.trigger_delay_ms, 300);
        assert_eq!(config.auto_copy, true);
    }
    
    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.ocr_engine, deserialized.ocr_engine);
    }
}

