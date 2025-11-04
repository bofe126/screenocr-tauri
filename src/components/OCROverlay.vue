<template>
  <div
    v-if="visible"
    class="ocr-overlay"
    @mousedown="startSelection"
    @mousemove="updateSelection"
    @mouseup="endSelection"
  >
    <!-- 半透明遮罩 -->
    <div class="overlay-mask"></div>
    
    <!-- 选择区域 -->
    <div
      v-if="selecting || selectedArea"
      class="selection-box"
      :style="selectionStyle"
    >
      <div class="selection-border"></div>
      <div class="selection-corner top-left"></div>
      <div class="selection-corner top-right"></div>
      <div class="selection-corner bottom-left"></div>
      <div class="selection-corner bottom-right"></div>
      
      <!-- 尺寸显示 -->
      <div class="selection-info">
        {{ selectionWidth }} × {{ selectionHeight }}
      </div>
    </div>
    
    <!-- 提示文字 -->
    <div class="hint-text" v-if="!selecting && !selectedArea">
      <el-icon :size="48"><Picture /></el-icon>
      <p>拖动鼠标选择要识别的区域</p>
      <p class="hint-sub">按 ESC 取消</p>
    </div>
    
    <!-- 操作按钮 -->
    <div v-if="selectedArea && !selecting" class="action-buttons">
      <el-button type="primary" @click="confirmSelection" :loading="recognizing">
        <el-icon><Select /></el-icon>
        {{ recognizing ? '识别中...' : '确认识别' }}
      </el-button>
      <el-button @click="resetSelection">
        <el-icon><RefreshLeft /></el-icon>
        重新选择
      </el-button>
      <el-button @click="cancel">
        <el-icon><Close /></el-icon>
        取消
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { ElMessage } from 'element-plus'
import { Picture, Select, RefreshLeft, Close } from '@element-plus/icons-vue'

const visible = ref(false)
const selecting = ref(false)
const recognizing = ref(false)
const startX = ref(0)
const startY = ref(0)
const currentX = ref(0)
const currentY = ref(0)
const selectedArea = ref<{x: number, y: number, width: number, height: number} | null>(null)

const selectionStyle = computed(() => {
  if (selectedArea.value) {
    return {
      left: `${selectedArea.value.x}px`,
      top: `${selectedArea.value.y}px`,
      width: `${selectedArea.value.width}px`,
      height: `${selectedArea.value.height}px`
    }
  }
  
  if (!selecting.value) return {}
  
  const x = Math.min(startX.value, currentX.value)
  const y = Math.min(startY.value, currentY.value)
  const width = Math.abs(currentX.value - startX.value)
  const height = Math.abs(currentY.value - startY.value)
  
  return {
    left: `${x}px`,
    top: `${y}px`,
    width: `${width}px`,
    height: `${height}px`
  }
})

const selectionWidth = computed(() => {
  if (selectedArea.value) return selectedArea.value.width
  if (!selecting.value) return 0
  return Math.abs(currentX.value - startX.value)
})

const selectionHeight = computed(() => {
  if (selectedArea.value) return selectedArea.value.height
  if (!selecting.value) return 0
  return Math.abs(currentY.value - startY.value)
})

const startSelection = (event: MouseEvent) => {
  if (selectedArea.value) return
  
  selecting.value = true
  startX.value = event.clientX
  startY.value = event.clientY
  currentX.value = event.clientX
  currentY.value = event.clientY
}

const updateSelection = (event: MouseEvent) => {
  if (!selecting.value) return
  
  currentX.value = event.clientX
  currentY.value = event.clientY
}

const endSelection = () => {
  if (!selecting.value) return
  
  const x = Math.min(startX.value, currentX.value)
  const y = Math.min(startY.value, currentY.value)
  const width = Math.abs(currentX.value - startX.value)
  const height = Math.abs(currentY.value - startY.value)
  
  // 最小选择区域 20x20
  if (width < 20 || height < 20) {
    ElMessage.warning('选择区域太小，请重新选择')
    resetSelection()
    return
  }
  
  selectedArea.value = { x, y, width, height }
  selecting.value = false
}

const resetSelection = () => {
  selecting.value = false
  selectedArea.value = null
  startX.value = 0
  startY.value = 0
  currentX.value = 0
  currentY.value = 0
}

const confirmSelection = async () => {
  if (!selectedArea.value) return
  
  recognizing.value = true
  
  try {
    console.log('正在识别区域:', selectedArea.value)
    
    // 调用后端区域 OCR 识别
    const result = await invoke('perform_ocr_on_region', {
      x: selectedArea.value.x,
      y: selectedArea.value.y,
      width: selectedArea.value.width,
      height: selectedArea.value.height
    }) as {
      text: string
      confidence: number
      language: string
    }
    
    console.log('识别结果:', result)
    
    ElMessage.success(`识别成功！识别了 ${result.text.length} 个字符`)
    
    // 关闭覆盖层
    hide()
    
    // 发送结果事件
    emit('ocr-complete', result)
  } catch (error) {
    console.error('OCR 识别错误:', error)
    ElMessage.error('OCR 识别失败: ' + error)
  } finally {
    recognizing.value = false
  }
}

const cancel = () => {
  hide()
}

const show = () => {
  visible.value = true
  resetSelection()
}

const hide = () => {
  visible.value = false
  resetSelection()
}

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    cancel()
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})

// 暴露方法给父组件
defineExpose({ show, hide })

// 事件
const emit = defineEmits<{
  'ocr-complete': [result: { text: string, confidence: number, language: string }]
}>()
</script>

<style scoped>
.ocr-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
  cursor: crosshair;
}

.overlay-mask {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(2px);
}

.selection-box {
  position: absolute;
  border: 2px solid #409eff;
  background: rgba(64, 158, 255, 0.1);
  box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.5);
  pointer-events: none;
}

.selection-border {
  position: absolute;
  top: -2px;
  left: -2px;
  right: -2px;
  bottom: -2px;
  border: 2px dashed #409eff;
  animation: dash 20s linear infinite;
}

@keyframes dash {
  to {
    stroke-dashoffset: -1000;
  }
}

.selection-corner {
  position: absolute;
  width: 8px;
  height: 8px;
  background: #409eff;
  border: 2px solid white;
  border-radius: 50%;
}

.selection-corner.top-left {
  top: -6px;
  left: -6px;
}

.selection-corner.top-right {
  top: -6px;
  right: -6px;
}

.selection-corner.bottom-left {
  bottom: -6px;
  left: -6px;
}

.selection-corner.bottom-right {
  bottom: -6px;
  right: -6px;
}

.selection-info {
  position: absolute;
  top: -35px;
  left: 0;
  background: #409eff;
  color: white;
  padding: 4px 12px;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  white-space: nowrap;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.hint-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
  color: white;
  font-size: 20px;
  z-index: 10;
  pointer-events: none;
}

.hint-text p {
  margin: 12px 0;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
}

.hint-sub {
  font-size: 16px;
  opacity: 0.8;
}

.action-buttons {
  position: absolute;
  bottom: 40px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 12px;
  z-index: 10;
}

.action-buttons :deep(.el-button) {
  font-size: 16px;
  padding: 12px 24px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}
</style>

