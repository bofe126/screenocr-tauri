<template>
  <div class="app-container">
    <Settings />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import Settings from './views/Settings.vue'
import { invoke } from '@tauri-apps/api/tauri'

onMounted(async () => {
  try {
    console.log('ScreenOCR Tauri 应用已启动')
  } catch (error) {
    console.error('初始化失败:', error)
  }
})
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

