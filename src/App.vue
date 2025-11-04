<template>
  <div class="app-container">
    <Settings />
    
    <!-- OCR 选择覆盖层 -->
    <OCROverlay ref="ocrOverlay" @ocr-complete="handleOCRComplete" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { ElMessage, ElNotification } from 'element-plus'
import Settings from './views/Settings.vue'
import OCROverlay from './components/OCROverlay.vue'

const ocrOverlay = ref<InstanceType<typeof OCROverlay> | null>(null)

let unlistenHotkey: (() => void) | null = null
let unlistenOCRResult: (() => void) | null = null
let unlistenOCRError: (() => void) | null = null

onMounted(async () => {
  try {
    console.log('ScreenOCR Tauri 应用已启动')
    
    // 监听热键触发事件
    unlistenHotkey = await listen('hotkey-triggered', () => {
      console.log('热键触发，显示 OCR 覆盖层')
      ocrOverlay.value?.show()
    })
    
    // 监听 OCR 结果
    unlistenOCRResult = await listen('ocr-result', (event: any) => {
      console.log('OCR 识别成功:', event.payload)
      const result = event.payload
      
      ElNotification({
        title: 'OCR 识别成功',
        message: `识别了 ${result.text.length} 个字符，已复制到剪贴板`,
        type: 'success',
        duration: 3000
      })
    })
    
    // 监听 OCR 错误
    unlistenOCRError = await listen('ocr-error', (event: any) => {
      console.error('OCR 识别失败:', event.payload)
      ElMessage.error('OCR 识别失败: ' + event.payload)
    })
    
  } catch (error) {
    console.error('初始化失败:', error)
  }
})

onUnmounted(() => {
  // 清理事件监听
  if (unlistenHotkey) unlistenHotkey()
  if (unlistenOCRResult) unlistenOCRResult()
  if (unlistenOCRError) unlistenOCRError()
})

const handleOCRComplete = (result: { text: string, confidence: number, language: string }) => {
  console.log('OCR 完成:', result)
  
  ElNotification({
    title: 'OCR 识别完成',
    message: `识别了 ${result.text.length} 个字符\n置信度: ${(result.confidence * 100).toFixed(1)}%`,
    type: 'success',
    duration: 4000
  })
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'PingFang SC', 
    'Hiragino Sans GB', 'Microsoft YaHei', sans-serif;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  min-height: 100vh;
}

.app-container {
  width: 100%;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>

