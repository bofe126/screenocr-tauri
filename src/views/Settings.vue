<template>
  <el-card class="settings-card" shadow="always">
    <template #header>
      <div class="card-header">
        <el-icon class="header-icon" :size="32">
          <Document />
        </el-icon>
        <h1>ScreenOCR 设置</h1>
      </div>
    </template>

    <el-form :model="config" label-width="140px" label-position="left">
      <el-form-item label="OCR引擎">
        <el-select
          v-model="config.ocrEngine"
          placeholder="选择OCR引擎"
          @change="saveConfig"
          style="width: 100%"
        >
          <el-option label="Tesseract OCR" value="Tesseract" />
          <el-option label="WeChatOCR（开发中）" value="WeChatOCR" disabled />
        </el-select>
      </el-form-item>

      <el-form-item label="触发延时（毫秒）">
        <el-slider
          v-model="config.triggerDelayMs"
          :min="0"
          :max="1000"
          :step="50"
          show-input
          @change="saveConfig"
        />
      </el-form-item>

      <el-form-item label="全局热键">
        <el-input
          v-model="config.hotkey"
          placeholder="按下热键..."
          @keydown="captureHotkey"
          readonly
        >
          <template #append>
            <el-button @click="clearHotkey">清除</el-button>
          </template>
        </el-input>
      </el-form-item>

      <el-form-item label="自动复制">
        <el-switch
          v-model="config.autoCopy"
          active-text="开启"
          inactive-text="关闭"
          @change="saveConfig"
        />
      </el-form-item>

      <el-form-item>
        <el-space>
          <el-button type="primary" @click="testOCR">
            测试 OCR
          </el-button>
          <el-button @click="openHelp">
            查看帮助
          </el-button>
          <el-button @click="resetConfig">
            重置配置
          </el-button>
        </el-space>
      </el-form-item>
    </el-form>

    <el-divider />

    <el-descriptions title="应用信息" :column="1" border>
      <el-descriptions-item label="版本">0.1.0</el-descriptions-item>
      <el-descriptions-item label="技术栈">
        Tauri + Rust + Vue 3 + Element Plus
      </el-descriptions-item>
      <el-descriptions-item label="状态">
        <el-tag type="success">运行中</el-tag>
      </el-descriptions-item>
    </el-descriptions>
  </el-card>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Document } from '@element-plus/icons-vue'

interface AppConfig {
  ocrEngine: string
  triggerDelayMs: number
  hotkey: string
  autoCopy: boolean
}

const config = ref<AppConfig>({
  ocrEngine: 'Tesseract',
  triggerDelayMs: 300,
  hotkey: 'Alt',
  autoCopy: true
})

onMounted(async () => {
  try {
    const loadedConfig = await invoke('get_config') as any
    if (loadedConfig) {
      // 转换 Rust 的 snake_case 为 JavaScript 的 camelCase
      config.value = {
        ocrEngine: loadedConfig.ocr_engine || loadedConfig.ocrEngine || 'Tesseract',
        triggerDelayMs: loadedConfig.trigger_delay_ms !== undefined ? loadedConfig.trigger_delay_ms : (loadedConfig.triggerDelayMs || 300),
        hotkey: loadedConfig.hotkey || 'Alt',
        autoCopy: loadedConfig.auto_copy !== undefined ? loadedConfig.auto_copy : (loadedConfig.autoCopy !== undefined ? loadedConfig.autoCopy : true)
      }
    }
    console.log('配置已加载:', config.value)
  } catch (error) {
    console.warn('使用默认配置:', error)
  }
})

const saveConfig = async () => {
  try {
    // 转换为 Rust 期望的 snake_case 格式
    const backendConfig = {
      ocr_engine: config.value.ocrEngine,
      trigger_delay_ms: config.value.triggerDelayMs,
      hotkey: config.value.hotkey,
      auto_copy: config.value.autoCopy
    }
    await invoke('update_config', { config: backendConfig })
    console.log('配置已保存:', backendConfig)
  } catch (error) {
    ElMessage.error('保存配置失败: ' + error)
  }
}

const captureHotkey = (event: KeyboardEvent) => {
  event.preventDefault()
  const keys = []
  if (event.ctrlKey) keys.push('Ctrl')
  if (event.shiftKey) keys.push('Shift')
  if (event.altKey) keys.push('Alt')
  if (event.metaKey) keys.push('Meta')
  
  if (event.key && !['Control', 'Shift', 'Alt', 'Meta'].includes(event.key)) {
    keys.push(event.key.toUpperCase())
  }
  
  config.value.hotkey = keys.join('+')
  saveConfig()
}

const clearHotkey = () => {
  config.value.hotkey = ''
  saveConfig()
}

const testOCR = async () => {
  try {
    ElMessage.info('正在截图和识别，请稍候...')
    
    // 调用后端 OCR 命令
    const result = await invoke('perform_ocr_on_screen') as {
      text: string
      confidence: number
      language: string
    }
    
    ElMessageBox.alert(
      `<div style="max-height: 400px; overflow-y: auto; white-space: pre-wrap;">
        <h3>识别结果</h3>
        <p><strong>文本内容：</strong></p>
        <pre style="background: #f5f5f5; padding: 10px; border-radius: 4px;">${result.text}</pre>
        <p><strong>识别置信度：</strong> ${(result.confidence * 100).toFixed(1)}%</p>
        <p><strong>语言：</strong> ${result.language}</p>
        <p><small>✅ 文本已自动复制到剪贴板</small></p>
      </div>`,
      'OCR 识别成功',
      {
        dangerouslyUseHTMLString: true,
        confirmButtonText: '关闭'
      }
    )
  } catch (error) {
    ElMessage.error('OCR 测试失败: ' + error)
    console.error('OCR 错误:', error)
  }
}

const openHelp = () => {
  ElMessageBox.alert(
    `
    <h3>使用说明</h3>
    <ol>
      <li>按下设置的快捷键开始OCR识别</li>
      <li>框选需要识别的文本区域</li>
      <li>等待识别完成，文本将自动复制到剪贴板</li>
    </ol>
    <h3>注意事项</h3>
    <ul>
      <li>首次使用需要安装 Tesseract OCR</li>
      <li>WeChatOCR 功能正在开发中</li>
      <li>支持中文和英文识别</li>
    </ul>
    `,
    '帮助',
    {
      dangerouslyUseHTMLString: true,
      confirmButtonText: '知道了'
    }
  )
}

const resetConfig = async () => {
  try {
    await ElMessageBox.confirm('确定要重置所有配置吗？', '警告', {
      type: 'warning',
      confirmButtonText: '确定',
      cancelButtonText: '取消'
    })
    
    // 调用后端重置命令
    const resetResult = await invoke('reset_config') as any
    
    // 更新前端配置
    config.value = {
      ocrEngine: resetResult.ocr_engine || resetResult.ocrEngine || 'Tesseract',
      triggerDelayMs: resetResult.trigger_delay_ms !== undefined ? resetResult.trigger_delay_ms : (resetResult.triggerDelayMs || 300),
      hotkey: resetResult.hotkey || 'Alt',
      autoCopy: resetResult.auto_copy !== undefined ? resetResult.auto_copy : (resetResult.autoCopy !== undefined ? resetResult.autoCopy : true)
    }
    
    ElMessage.success('配置已重置为默认值')
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('重置配置失败: ' + error)
    }
  }
}
</script>

<style scoped>
.settings-card {
  width: 650px;
  max-width: 90vw;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-radius: 16px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-icon {
  color: #667eea;
}

h1 {
  margin: 0;
  font-size: 24px;
  color: #667eea;
  font-weight: 600;
}

:deep(.el-form-item__label) {
  font-weight: 500;
  color: #606266;
}

:deep(.el-descriptions__title) {
  font-weight: 600;
  color: #303133;
}
</style>

