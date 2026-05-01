<template>
  <div class="launcher-container">
    <!-- 主内容区 -->
    <div class="main-content">
      <!-- 顶部横幅 -->
      <div class="header-banner">
        <div class="logo-section">
          <img src="/webrpa-logo.png" alt="WebRPA Logo" class="logo-icon" />
          <div class="logo-text">
            <h1>WebRPA 启动器</h1>
            <p>网页机器人流程自动化工具</p>
          </div>
        </div>
        <div class="header-right">
          <div class="version-section">
            <div class="version-info-compact">
              <span class="version-label">当前版本</span>
              <span class="version-value">{{ version }}</span>
              <button class="check-btn-compact" @click="checkUpdate" :disabled="checking">
                <svg v-if="!checking" viewBox="0 0 24 24" fill="none">
                  <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <svg v-else class="spinning" viewBox="0 0 24 24" fill="none">
                  <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <span>{{ checking ? '检查中' : '检查更新' }}</span>
              </button>
            </div>
          </div>
          <div class="header-actions">
            <button class="icon-btn" @click="showConfigModal = true" title="设置">
              <svg viewBox="0 0 24 24" fill="none">
                <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="2"/>
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1Z" stroke="currentColor" stroke-width="1.5"/>
              </svg>
            </button>
            <button class="icon-btn" @click="openGithub" title="GitHub仓库">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
            </button>
            <button class="icon-btn" @click="showSponsor" title="赞助支持">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>
        </div>
      </div>

      <!-- 更新通知 -->
      <div v-if="updateInfo && updateInfo.has_update" class="update-notice">
        <div class="notice-icon">
          <svg viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
            <path d="M12 8v4M12 16h.01" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </div>
        <div class="notice-content">
          <h3>发现新版本 {{ updateInfo.latest_version }}</h3>
          <p class="release-date">发布日期: {{ updateInfo.release_date }}</p>
          <p class="changelog">{{ updateInfo.changelog }}</p>
        </div>
        <div class="update-buttons">
          <button class="download-btn mirror-btn" @click="downloadWithMirror" title="使用国内加速镜像下载">
            <svg viewBox="0 0 24 24" fill="none">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <span>加速下载</span>
          </button>
          <button class="download-btn" @click="downloadUpdate">
            <svg viewBox="0 0 24 24" fill="none">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <span>GitHub下载</span>
          </button>
        </div>
      </div>

      <!-- 服务控制区 -->
      <div class="control-section">
        <div class="section-header">
          <svg viewBox="0 0 24 24" fill="none">
            <rect x="2" y="3" width="20" height="14" rx="2" stroke="currentColor" stroke-width="2"/>
            <path d="M8 21h8M12 17v4" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <h2>服务控制</h2>
        </div>

        <div class="service-status">
          <div class="status-item" :class="{ active: backendRunning }">
            <div class="status-indicator"></div>
            <div class="status-info">
              <span class="status-name">后端服务</span>
              <span class="status-text">{{ backendRunning ? '运行中' : '未启动' }}</span>
            </div>
          </div>
          <div class="status-item" :class="{ active: frontendRunning }">
            <div class="status-indicator"></div>
            <div class="status-info">
              <span class="status-name">前端服务</span>
              <span class="status-text">{{ frontendRunning ? '运行中' : '未启动' }}</span>
            </div>
          </div>
        </div>

        <div class="action-buttons">
          <button 
            class="action-btn start-btn" 
            @click="startServices" 
            :disabled="starting || (backendRunning && frontendRunning)"
          >
            <svg v-if="!starting" viewBox="0 0 24 24" fill="none">
              <path d="M5 3l14 9-14 9V3z" fill="currentColor"/>
            </svg>
            <svg v-else class="spinning" viewBox="0 0 24 24" fill="none">
              <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <span>{{ starting ? '启动中...' : '启动服务' }}</span>
          </button>
          
          <button 
            class="action-btn stop-btn" 
            @click="stopServices" 
            :disabled="!backendRunning && !frontendRunning"
          >
            <svg viewBox="0 0 24 24" fill="none">
              <rect x="6" y="6" width="12" height="12" fill="currentColor"/>
            </svg>
            <span>停止服务</span>
          </button>

          <button 
            class="action-btn open-btn" 
            @click="openBrowser" 
            :disabled="!frontendRunning"
          >
            <svg viewBox="0 0 24 24" fill="none">
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6M15 3h6v6M10 14L21 3" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <span>打开浏览器</span>
          </button>
        </div>
      </div>

      <!-- 日志查看区 -->
      <div class="logs-section">
        <div class="section-header">
          <svg viewBox="0 0 24 24" fill="none">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" stroke="currentColor" stroke-width="2"/>
            <path d="M14 2v6h6M16 13H8M16 17H8M10 9H8" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <h2>运行日志</h2>
        </div>

        <div class="log-buttons">
          <button class="log-btn backend-btn" @click="openBackendLog">
            <svg viewBox="0 0 24 24" fill="none">
              <rect x="2" y="3" width="20" height="14" rx="2" stroke="currentColor" stroke-width="2"/>
              <path d="M8 21h8M12 17v4" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
            <div class="btn-content">
              <span class="btn-title">后端日志</span>
              <span class="btn-desc">查看后端服务运行日志</span>
            </div>
            <svg class="arrow-icon" viewBox="0 0 24 24" fill="none">
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6M15 3h6v6M10 14L21 3" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
          
          <button class="log-btn frontend-btn" @click="openFrontendLog">
            <svg viewBox="0 0 24 24" fill="none">
              <path d="M16 18l6-6-6-6M8 6l-6 6 6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <div class="btn-content">
              <span class="btn-title">前端日志</span>
              <span class="btn-desc">查看前端服务运行日志</span>
            </div>
            <svg class="arrow-icon" viewBox="0 0 24 24" fill="none">
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6M15 3h6v6M10 14L21 3" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- 底部信息栏 -->
      <div class="footer">
        <p>© 2026 青云制作_彭明航 版权所有</p>
        <div class="footer-links">
          <a @click="openGithub">GitHub</a>
          <span>·</span>
          <a @click="showLicense">开源协议</a>
          <span>·</span>
          <a @click="showSponsor">赞助支持</a>
        </div>
      </div>
    </div>

    <!-- 赞助弹窗 -->
    <div v-if="showSponsorModal" class="modal-overlay" @click="showSponsorModal = false">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>赞助支持</h2>
          <button class="close-btn" @click="showSponsorModal = false">
            <svg viewBox="0 0 24 24" fill="none">
              <path d="M18 6L6 18M6 6l12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </button>
        </div>
        <div class="modal-body">
          <p class="sponsor-text">如果 WebRPA 对您有帮助，希望您能支持一下独立开发者！</p>
          <div class="qrcode-container">
            <div class="qrcode-item">
              <img :src="wechatQr" alt="微信赞助" class="qrcode-image" />
              <p>微信赞助</p>
            </div>
            <div class="qrcode-item">
              <img :src="alipayQr" alt="支付宝赞助" class="qrcode-image" />
              <p>支付宝赞助</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 配置弹窗 -->
    <div v-if="showConfigModal" class="modal-overlay" @click="showConfigModal = false">
      <div class="modal-content config-modal" @click.stop>
        <div class="modal-header">
          <h2>启动器设置</h2>
          <button class="close-btn" @click="showConfigModal = false">
            <svg viewBox="0 0 24 24" fill="none">
              <path d="M18 6L6 18M6 6l12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </button>
        </div>
        <div class="modal-body config-body">
          <div class="config-section">
            <h3>
              <svg viewBox="0 0 24 24" fill="none">
                <rect x="2" y="3" width="20" height="14" rx="2" stroke="currentColor" stroke-width="2"/>
                <path d="M8 21h8M12 17v4" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              </svg>
              后端服务配置
            </h3>
            <div class="config-item">
              <label>监听地址</label>
              <div class="input-group">
                <select v-model="configForm.backend.host" class="config-select">
                  <option value="127.0.0.1">127.0.0.1 (仅本机访问)</option>
                  <option value="0.0.0.0">0.0.0.0 (允许局域网访问)</option>
                </select>
                <span class="input-hint">选择是否允许局域网内其他设备访问</span>
              </div>
            </div>
            <div class="config-item">
              <label>端口号</label>
              <div class="input-group">
                <input 
                  type="number" 
                  v-model.number="configForm.backend.port" 
                  class="config-input"
                  min="1024"
                  max="65535"
                  placeholder="8000"
                />
                <span class="input-hint">范围: 1024-65535，默认: 8000</span>
              </div>
            </div>
          </div>

          <div class="config-section">
            <h3>
              <svg viewBox="0 0 24 24" fill="none">
                <path d="M16 18l6-6-6-6M8 6l-6 6 6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              前端服务配置
            </h3>
            <div class="config-item">
              <label>监听地址</label>
              <div class="input-group">
                <select v-model="configForm.frontend.host" class="config-select">
                  <option value="127.0.0.1">127.0.0.1 (仅本机访问)</option>
                  <option value="0.0.0.0">0.0.0.0 (允许局域网访问)</option>
                </select>
                <span class="input-hint">选择是否允许局域网内其他设备访问</span>
              </div>
            </div>
            <div class="config-item">
              <label>端口号</label>
              <div class="input-group">
                <input 
                  type="number" 
                  v-model.number="configForm.frontend.port" 
                  class="config-input"
                  min="1024"
                  max="65535"
                  placeholder="5173"
                />
                <span class="input-hint">范围: 1024-65535，默认: 5173</span>
              </div>
            </div>
          </div>

          <div class="config-tips">
            <svg viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
              <path d="M12 16v-4M12 8h.01" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
            <div>
              <p><strong>提示：</strong></p>
              <ul>
                <li>修改端口号可以实现 WebRPA 多开（每个实例使用不同端口）</li>
                <li>允许局域网访问后，可以在同一网络的其他设备上访问 WebRPA</li>
                <li>修改配置后需要重启服务才能生效</li>
              </ul>
            </div>
          </div>

          <div class="config-actions">
            <button class="config-btn cancel-btn" @click="cancelConfig">
              <svg viewBox="0 0 24 24" fill="none">
                <path d="M18 6L6 18M6 6l12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              </svg>
              取消
            </button>
            <button class="config-btn save-btn" @click="saveConfiguration" :disabled="saving">
              <svg v-if="!saving" viewBox="0 0 24 24" fill="none">
                <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" stroke="currentColor" stroke-width="2"/>
                <path d="M17 21v-8H7v8M7 3v5h8" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              </svg>
              <svg v-else class="spinning" viewBox="0 0 24 24" fill="none">
                <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              {{ saving ? '保存中...' : '保存配置' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, nextTick, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import wechatQr from './assets/wechat-qr.png'
import alipayQr from './assets/alipay-qr.jpg'
import webrpaLogo from './assets/webrpa-logo.png'

const version = ref('加载中...')
const checking = ref(false)
const updateInfo = ref(null)
const starting = ref(false)
const backendRunning = ref(false)
const frontendRunning = ref(false)
const showSponsorModal = ref(false)
const showConfigModal = ref(false)
const saving = ref(false)
const statusCheckInterval = ref(null)
const configForm = ref({
  backend: {
    host: '0.0.0.0',
    port: 8000,
    reload: false
  },
  frontend: {
    host: '0.0.0.0',
    port: 5173
  }
})

// 检查服务状态
const checkServiceStatus = async () => {
  try {
    const [backend, frontend] = await invoke('check_service_status')
    console.log(`状态检查结果: 后端=${backend}, 前端=${frontend}`)
    backendRunning.value = backend
    frontendRunning.value = frontend
  } catch (error) {
    console.error('检查服务状态失败:', error)
  }
}

// 启动状态检查定时器
const startStatusCheck = () => {
  if (statusCheckInterval.value) {
    clearInterval(statusCheckInterval.value)
  }
  statusCheckInterval.value = setInterval(checkServiceStatus, 3000) // 每3秒检查一次
}

// 停止状态检查定时器
const stopStatusCheck = () => {
  if (statusCheckInterval.value) {
    clearInterval(statusCheckInterval.value)
    statusCheckInterval.value = null
  }
}
const checkUpdate = async () => {
  checking.value = true
  try {
    const result = await invoke('check_update', { currentVersion: version.value })
    updateInfo.value = result
    if (result.has_update) {
      console.log(`发现新版本: ${result.latest_version}`)
    } else {
      console.log('当前已是最新版本')
    }
  } catch (error) {
    console.error('检查更新失败:', error)
  } finally {
    checking.value = false
  }
}

// 下载更新
const downloadUpdate = () => {
  if (updateInfo.value && updateInfo.value.update_url) {
    invoke('open_browser', { url: updateInfo.value.update_url })
  }
}

// 使用加速镜像下载
const downloadWithMirror = () => {
  if (updateInfo.value && updateInfo.value.latest_version) {
    const version = updateInfo.value.latest_version
    // 使用GitHub加速镜像
    const mirrorUrl = `https://ghfile.geekertao.top/github.com/pmh1314520/WebRPA/releases/download/v${version}/WebRPA-${version}-FullVersion.7z`
    invoke('open_browser', { url: mirrorUrl })
    console.log('已打开加速镜像下载链接')
    console.log('下载完成后，请解压到WebRPA目录覆盖原文件')
  }
}

// 启动服务
const startServices = async () => {
  starting.value = true
  
  try {
    // 先检查当前状态
    await checkServiceStatus()
    
    let needStartBackend = !backendRunning.value
    let needStartFrontend = !frontendRunning.value
    
    if (!needStartBackend && !needStartFrontend) {
      console.log('所有服务都已在运行中')
      alert('所有服务都已在运行中')
      return
    }
    
    // 启动后端（如果需要）
    if (needStartBackend) {
      console.log('正在启动后端服务...')
      await invoke('start_backend')
      console.log('后端服务启动命令已发送，等待服务完全启动...')
      
      // 轮询检查后端是否真正启动（最多等待30秒）
      let backendStarted = false
      for (let i = 0; i < 60; i++) {
        await new Promise(resolve => setTimeout(resolve, 500))
        await checkServiceStatus()
        if (backendRunning.value) {
          backendStarted = true
          console.log(`后端服务启动成功（耗时约${(i + 1) * 0.5}秒）`)
          break
        }
      }
      
      if (!backendStarted) {
        throw new Error('后端服务启动超时（超过30秒），请检查后端日志')
      }
    }
    
    // 启动前端（如果需要）
    if (needStartFrontend) {
      console.log('正在启动前端服务...')
      await invoke('start_frontend')
      console.log('前端服务启动命令已发送，等待服务完全启动...')
      
      // 轮询检查前端是否真正启动（最多等待20秒）
      let frontendStarted = false
      for (let i = 0; i < 40; i++) {
        await new Promise(resolve => setTimeout(resolve, 500))
        await checkServiceStatus()
        if (frontendRunning.value) {
          frontendStarted = true
          console.log(`前端服务启动成功（耗时约${(i + 1) * 0.5}秒）`)
          break
        }
      }
      
      if (!frontendStarted) {
        throw new Error('前端服务启动超时（超过20秒），请检查前端日志')
      }
    }
    
    // 最终检查状态
    await checkServiceStatus()
    
    // 如果都启动成功，自动打开浏览器
    if (backendRunning.value && frontendRunning.value) {
      await openBrowser()
    }
  } catch (error) {
    console.error('启动服务失败:', error)
    const errorMsg = typeof error === 'string' ? error : String(error)
    
    // 显示详细错误信息
    let friendlyMsg = `启动失败: ${errorMsg}`
    
    if (errorMsg.includes('已被占用') || errorMsg.includes('already in use') || errorMsg.includes('10048')) {
      friendlyMsg += '\n\n提示: 端口被占用，请先停止已运行的服务'
    } else if (errorMsg.includes('未找到')) {
      friendlyMsg += '\n\n提示: 请确保在WebRPA根目录中运行启动器'
    } else if (errorMsg.includes('npm') || errorMsg.includes('node')) {
      friendlyMsg += '\n\n提示: 前端启动失败，请检查Node.js环境'
    } else if (errorMsg.includes('立即退出')) {
      friendlyMsg += '\n\n提示: 前端进程异常退出，请查看前端日志文件获取详细信息'
    } else if (errorMsg.includes('超时')) {
      friendlyMsg += '\n\n提示: 服务启动时间过长，可能是首次启动需要加载依赖，请查看日志文件'
    }
    
    alert(friendlyMsg)
    
    // 重新检查状态
    await checkServiceStatus()
  } finally {
    starting.value = false
  }
}

// 停止服务
const stopServices = async () => {
  try {
    await invoke('stop_services')
    console.log('服务已停止')
    
    // 等待1秒后检查状态
    await new Promise(resolve => setTimeout(resolve, 1000))
    await checkServiceStatus()
  } catch (error) {
    console.error('停止服务失败:', error)
    // 即使停止失败，也要检查实际状态
    await checkServiceStatus()
  }
}

// 打开浏览器
const openBrowser = async () => {
  try {
    const config = await invoke('read_config')
    const url = `http://localhost:${config.frontend.port}?backend_port=${config.backend.port}`
    await invoke('open_browser', { url })
  } catch (error) {
    console.error('打开浏览器失败:', error)
  }
}

// 打开GitHub
const openGithub = () => {
  invoke('open_browser', { url: 'https://github.com/pmh1314520/WebRPA' })
}

// 显示赞助弹窗
const showSponsor = () => {
  showSponsorModal.value = true
}

// 显示开源协议
const showLicense = () => {
  invoke('open_browser', { url: 'https://github.com/pmh1314520/WebRPA/blob/main/LICENSE' })
}

// 加载配置
const loadConfig = async () => {
  try {
    const config = await invoke('read_config')
    configForm.value = JSON.parse(JSON.stringify(config))
  } catch (error) {
    console.error('加载配置失败:', error)
  }
}

// 保存配置
const saveConfiguration = async () => {
  saving.value = true
  try {
    // 验证端口号
    if (configForm.value.backend.port < 1024 || configForm.value.backend.port > 65535) {
      alert('后端端口号必须在 1024-65535 之间')
      return
    }
    if (configForm.value.frontend.port < 1024 || configForm.value.frontend.port > 65535) {
      alert('前端端口号必须在 1024-65535 之间')
      return
    }
    
    await invoke('save_config', { config: configForm.value })
    console.log('配置保存成功')
    showConfigModal.value = false
    
    // 如果服务正在运行，提示需要重启
    if (backendRunning.value || frontendRunning.value) {
      alert('提示: 配置已更新，请重启服务使配置生效')
    }
  } catch (error) {
    console.error('保存配置失败:', error)
    alert(`保存配置失败: ${error}`)
  } finally {
    saving.value = false
  }
}

// 取消配置
const cancelConfig = () => {
  loadConfig() // 重新加载配置
  showConfigModal.value = false
}

// 打开后端日志文件
const openBackendLog = async () => {
  try {
    await invoke('open_backend_log')
  } catch (error) {
    console.error('打开后端日志失败:', error)
    alert(`打开后端日志失败: ${error}`)
  }
}

// 打开前端日志文件
const openFrontendLog = async () => {
  try {
    await invoke('open_frontend_log')
  } catch (error) {
    console.error('打开前端日志失败:', error)
    alert(`打开前端日志失败: ${error}`)
  }
}

onMounted(async () => {
  // 加载版本号
  try {
    version.value = await invoke('get_version')
  } catch (error) {
    console.error('获取版本号失败:', error)
    version.value = '未知'
  }
  
  // 加载配置
  await loadConfig()
  
  // 立即检查一次服务状态
  await checkServiceStatus()
  
  // 启动服务状态检查
  startStatusCheck()
  
  // 启动时自动检查更新
  setTimeout(() => {
    checkUpdate()
  }, 1000)
})

onUnmounted(() => {
  // 停止状态检查定时器
  stopStatusCheck()
})
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.launcher-container {
  width: 100%;
  height: 100vh;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
  overflow: hidden;
}

.main-content {
  width: 100%;
  height: 100%;
  padding: 30px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

/* 顶部横幅 */
.header-banner {
  background: linear-gradient(135deg, #1e88e5 0%, #1565c0 100%);
  border-radius: 16px;
  padding: 30px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 10px 30px rgba(30, 136, 229, 0.3);
  animation: slideDown 0.6s ease;
}

.logo-section {
  display: flex;
  align-items: center;
  gap: 20px;
}

.logo-icon {
  width: 60px;
  height: 60px;
  filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.2));
  border-radius: 8px;
}

.logo-text h1 {
  font-size: 28px;
  font-weight: 700;
  color: white;
  margin-bottom: 5px;
}

.logo-text p {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.9);
}

.header-actions {
  display: flex;
  gap: 12px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 24px;
}

.version-section {
  display: flex;
  align-items: center;
  gap: 12px;
}

.version-info-compact {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 16px;
  background: rgba(255, 255, 255, 0.15);
  border-radius: 12px;
  backdrop-filter: blur(10px);
}

.version-label {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.8);
  font-weight: 500;
}

.version-value {
  font-size: 16px;
  color: white;
  font-weight: 700;
  letter-spacing: 0.5px;
}

.check-btn-compact {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  background: rgba(255, 255, 255, 0.25);
  border: none;
  border-radius: 8px;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
}

.check-btn-compact:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.35);
  transform: translateY(-1px);
}

.check-btn-compact:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.check-btn-compact svg {
  width: 16px;
  height: 16px;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.icon-btn {
  width: 48px;
  height: 48px;
  border: none;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
}

.icon-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.icon-btn svg {
  width: 24px;
  height: 24px;
}

/* 更新通知 */
.update-notice {
  background: linear-gradient(135deg, #ffeaa7 0%, #fdcb6e 100%);
  border-radius: 16px;
  padding: 24px;
  display: flex;
  align-items: center;
  gap: 20px;
  box-shadow: 0 4px 20px rgba(253, 203, 110, 0.3);
  animation: slideIn 0.6s ease;
}

.notice-icon {
  width: 48px;
  height: 48px;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.notice-icon svg {
  width: 28px;
  height: 28px;
  color: #d63031;
}

.notice-content {
  flex: 1;
}

.notice-content h3 {
  font-size: 18px;
  color: #2c3e50;
  margin-bottom: 8px;
}

.release-date {
  font-size: 13px;
  color: #6c757d;
  margin-bottom: 4px;
}

.changelog {
  font-size: 14px;
  color: #2c3e50;
}

.update-buttons {
  display: flex;
  gap: 12px;
  flex-shrink: 0;
}

.download-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  background: white;
  color: #d63031;
  border: 2px solid #d63031;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  flex-shrink: 0;
  white-space: nowrap;
}

.download-btn:hover {
  background: #d63031;
  color: white;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(214, 48, 49, 0.3);
}

.download-btn svg {
  width: 18px;
  height: 18px;
}

.mirror-btn {
  background: linear-gradient(135deg, #1e88e5 0%, #1565c0 100%);
  color: white;
  border: none;
}

.mirror-btn:hover {
  background: linear-gradient(135deg, #1565c0 0%, #0d47a1 100%);
  box-shadow: 0 4px 12px rgba(30, 136, 229, 0.4);
}

/* 服务控制区 */
.control-section {
  background: white;
  border-radius: 16px;
  padding: 20px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  animation: fadeIn 0.6s ease 0.4s both;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 20px;
  color: #2c3e50;
}

.section-header svg {
  width: 28px;
  height: 28px;
  color: #1e88e5;
}

.section-header h2 {
  font-size: 20px;
  font-weight: 600;
}

.service-status {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  margin-bottom: 16px;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: #f8f9fa;
  border-radius: 12px;
  border: 2px solid transparent;
  transition: all 0.3s ease;
}

.status-item.active {
  background: #d4edda;
  border-color: #28a745;
}

.status-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #6c757d;
  transition: all 0.3s ease;
}

.status-item.active .status-indicator {
  background: #28a745;
  animation: pulse 2s ease-in-out infinite;
}

.status-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.status-name {
  font-size: 14px;
  font-weight: 600;
  color: #2c3e50;
}

.status-text {
  font-size: 12px;
  color: #6c757d;
}

.status-item.active .status-text {
  color: #28a745;
  font-weight: 500;
}

.action-buttons {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr;
  gap: 12px;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 20px;
  border: none;
  border-radius: 12px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
}

.action-btn svg {
  width: 20px;
  height: 20px;
}

.start-btn {
  background: linear-gradient(135deg, #1e88e5 0%, #1565c0 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(30, 136, 229, 0.3);
}

.start-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(30, 136, 229, 0.4);
}

.start-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.stop-btn {
  background: white;
  color: #e74c3c;
  border: 2px solid #e74c3c;
}

.stop-btn:hover:not(:disabled) {
  background: #e74c3c;
  color: white;
  transform: translateY(-2px);
}

.stop-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.open-btn {
  background: white;
  color: #1e88e5;
  border: 2px solid #1e88e5;
}

.open-btn:hover:not(:disabled) {
  background: #1e88e5;
  color: white;
  transform: translateY(-2px);
}

.open-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* 日志区 */
.logs-section {
  background: white;
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  animation: fadeIn 0.6s ease 0.6s both;
}

.log-buttons {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 20px;
}

.log-btn {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: #f8f9fa;
  border: 2px solid #e9ecef;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: left;
}

.log-btn:hover {
  background: #e3f2fd;
  border-color: #1e88e5;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(30, 136, 229, 0.15);
}

.log-btn svg:first-child {
  width: 32px;
  height: 32px;
  color: #1e88e5;
  flex-shrink: 0;
}

.btn-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.btn-title {
  font-size: 16px;
  font-weight: 600;
  color: #2c3e50;
}

.btn-desc {
  font-size: 13px;
  color: #6c757d;
}

.arrow-icon {
  width: 20px;
  height: 20px;
  color: #6c757d;
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.log-btn:hover .arrow-icon {
  color: #1e88e5;
  transform: translateX(4px);
}

.log-tips {
  display: flex;
  gap: 12px;
  padding: 16px;
  background: #e8f5e8;
  border-radius: 12px;
  border: 1px solid #c3e6c3;
}

.log-tips svg {
  width: 24px;
  height: 24px;
  color: #28a745;
  flex-shrink: 0;
  margin-top: 2px;
}

.log-tips p {
  margin: 0 0 8px 0;
  font-size: 14px;
  color: #28a745;
}

.log-tips ul {
  margin: 0;
  padding-left: 20px;
  font-size: 13px;
  color: #2c3e50;
}

.log-tips li {
  margin: 4px 0;
}

/* 底部信息栏 */
.footer {
  background: white;
  border-radius: 16px;
  padding: 20px;
  text-align: center;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  animation: fadeIn 0.6s ease 0.8s both;
}

.footer p {
  font-size: 13px;
  color: #6c757d;
  margin-bottom: 8px;
}

.footer-links {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 12px;
  font-size: 13px;
}

.footer-links a {
  color: #1e88e5;
  cursor: pointer;
  transition: all 0.3s ease;
}

.footer-links a:hover {
  color: #1565c0;
  text-decoration: underline;
}

.footer-links span {
  color: #dee2e6;
}

/* 弹窗 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.3s ease;
}

.modal-content {
  background: white;
  border-radius: 16px;
  width: 90%;
  max-width: 500px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  animation: scaleIn 0.3s ease;
}

.config-modal {
  max-width: 700px;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px;
  border-bottom: 1px solid #e9ecef;
}

.modal-header h2 {
  font-size: 20px;
  color: #2c3e50;
}

.close-btn {
  width: 36px;
  height: 36px;
  border: none;
  background: #f8f9fa;
  border-radius: 8px;
  color: #6c757d;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.close-btn:hover {
  background: #e9ecef;
  color: #2c3e50;
}

.close-btn svg {
  width: 20px;
  height: 20px;
}

.modal-body {
  padding: 24px;
}

.sponsor-text {
  font-size: 14px;
  color: #6c757d;
  margin-bottom: 20px;
  text-align: center;
}

.qrcode-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
}

.qrcode-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.qrcode-image {
  width: 200px;
  height: 200px;
  border-radius: 12px;
  border: 2px solid #e9ecef;
  object-fit: contain;
  background: white;
  padding: 8px;
  transition: all 0.3s ease;
}

.qrcode-image:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.qrcode-item p {
  font-size: 14px;
  color: #2c3e50;
  font-weight: 600;
}

/* 动画 */
@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateX(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.9);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

@keyframes pulse {
  0%, 100% {
    box-shadow: 0 0 0 0 rgba(40, 167, 69, 0.7);
  }
  50% {
    box-shadow: 0 0 0 8px rgba(40, 167, 69, 0);
  }
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

/* 滚动条样式 */
.logs-content::-webkit-scrollbar {
  width: 8px;
}

.logs-content::-webkit-scrollbar-track {
  background: #2d2d2d;
  border-radius: 4px;
}

.logs-content::-webkit-scrollbar-thumb {
  background: #555;
  border-radius: 4px;
}

.logs-content::-webkit-scrollbar-thumb:hover {
  background: #666;
}

.main-content::-webkit-scrollbar {
  width: 10px;
}

.main-content::-webkit-scrollbar-track {
  background: transparent;
}

.main-content::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 5px;
}

.main-content::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

/* 配置弹窗样式 */
.config-body {
  max-height: 70vh;
  overflow-y: auto;
}

.config-section {
  margin-bottom: 30px;
  padding-bottom: 30px;
  border-bottom: 1px solid #e9ecef;
}

.config-section:last-of-type {
  border-bottom: none;
}

.config-section h3 {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 16px;
  color: #1e88e5;
  margin-bottom: 20px;
}

.config-section h3 svg {
  width: 20px;
  height: 20px;
}

.config-item {
  margin-bottom: 20px;
}

.config-item label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 8px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.config-input,
.config-select {
  width: 100%;
  padding: 10px 14px;
  border: 2px solid #e9ecef;
  border-radius: 8px;
  font-size: 14px;
  color: #2c3e50;
  transition: all 0.3s ease;
  background: white;
}

.config-input:focus,
.config-select:focus {
  outline: none;
  border-color: #1e88e5;
  box-shadow: 0 0 0 3px rgba(30, 136, 229, 0.1);
}

.config-input:hover,
.config-select:hover {
  border-color: #1e88e5;
}

.input-hint {
  font-size: 12px;
  color: #6c757d;
}

.config-tips {
  display: flex;
  gap: 12px;
  padding: 16px;
  background: #e3f2fd;
  border-radius: 12px;
  margin-bottom: 20px;
}

.config-tips svg {
  width: 24px;
  height: 24px;
  color: #1e88e5;
  flex-shrink: 0;
  margin-top: 2px;
}

.config-tips p {
  margin: 0 0 8px 0;
  font-size: 14px;
  color: #1e88e5;
}

.config-tips ul {
  margin: 0;
  padding-left: 20px;
  font-size: 13px;
  color: #2c3e50;
}

.config-tips li {
  margin: 4px 0;
}

.config-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.config-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
}

.config-btn svg {
  width: 18px;
  height: 18px;
}

.cancel-btn {
  background: white;
  color: #6c757d;
  border: 2px solid #e9ecef;
}

.cancel-btn:hover {
  background: #f8f9fa;
  border-color: #6c757d;
}

.save-btn {
  background: linear-gradient(135deg, #1e88e5 0%, #1565c0 100%);
  color: white;
}

.save-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(30, 136, 229, 0.4);
}

.save-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.config-body::-webkit-scrollbar {
  width: 8px;
}

.config-body::-webkit-scrollbar-track {
  background: #f8f9fa;
  border-radius: 4px;
}

.config-body::-webkit-scrollbar-thumb {
  background: #dee2e6;
  border-radius: 4px;
}

.config-body::-webkit-scrollbar-thumb:hover {
  background: #adb5bd;
}
</style>
