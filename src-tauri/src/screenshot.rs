use anyhow::Result;
use image::{ImageBuffer, Rgba};
use screenshots::Screen;
use std::io::Cursor;

/// 截取所有屏幕并返回截图数据
pub fn capture_all_screens() -> Result<Vec<ScreenCapture>> {
    let screens = Screen::all()?;
    let mut captures = Vec::new();

    for screen in screens {
        let image = screen.capture()?;
        let buffer = image.rgba().to_vec();
        
        captures.push(ScreenCapture {
            width: image.width(),
            height: image.height(),
            data: buffer,
            x: screen.display_info.x,
            y: screen.display_info.y,
        });
    }

    Ok(captures)
}

/// 截取主屏幕
pub fn capture_primary_screen() -> Result<ScreenCapture> {
    let screens = Screen::all()?;
    
    if screens.is_empty() {
        return Err(anyhow::anyhow!("未找到可用屏幕"));
    }

    let screen = &screens[0];
    let image = screen.capture()?;
    let buffer = image.rgba().to_vec();

    Ok(ScreenCapture {
        width: image.width(),
        height: image.height(),
        data: buffer,
        x: screen.display_info.x,
        y: screen.display_info.y,
    })
}

/// 将图像转换为 PNG 格式的 Base64 字符串
pub fn to_base64_png(capture: &ScreenCapture) -> Result<String> {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(capture.width, capture.height, capture.data.clone())
            .ok_or_else(|| anyhow::anyhow!("无法创建图像缓冲区"))?;

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    img.write_to(&mut cursor, image::ImageOutputFormat::Png)?;

    Ok(base64::encode(&buffer))
}

/// 屏幕截图数据
#[derive(Debug, Clone)]
pub struct ScreenCapture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub x: i32,
    pub y: i32,
}

/// 截图区域
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CaptureRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// 从完整截图中裁剪指定区域
pub fn crop_region(capture: &ScreenCapture, region: &CaptureRegion) -> Result<ScreenCapture> {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(capture.width, capture.height, capture.data.clone())
            .ok_or_else(|| anyhow::anyhow!("无法创建图像缓冲区"))?;

    let cropped = image::imageops::crop_imm(&img, region.x, region.y, region.width, region.height);
    let cropped_buffer = cropped.to_image();

    Ok(ScreenCapture {
        width: region.width,
        height: region.height,
        data: cropped_buffer.into_raw(),
        x: capture.x + region.x as i32,
        y: capture.y + region.y as i32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_primary_screen() {
        let result = capture_primary_screen();
        assert!(result.is_ok());
        
        let capture = result.unwrap();
        assert!(capture.width > 0);
        assert!(capture.height > 0);
        assert!(!capture.data.is_empty());
    }

    #[test]
    fn test_to_base64() {
        let capture = capture_primary_screen().unwrap();
        let result = to_base64_png(&capture);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}

