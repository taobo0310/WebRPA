// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio, Child};
use std::sync::{Arc, Mutex};
use std::io::{BufRead, BufReader};
use std::thread;
use tauri::{Manager, Window};
use serde::{Deserialize, Serialize};
use reqwest;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    backend: BackendConfig,
    frontend: FrontendConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct BackendConfig {
    host: String,
    port: u16,
    reload: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct FrontendConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VersionInfo {
    current_version: String,
    latest_version: String,
    has_update: bool,
    update_url: String,
    release_date: String,
    changelog: String,
}

#[derive(Debug, Deserialize)]
struct RemoteVersionInfo {
    version: String,
    #[serde(rename = "releaseDate")]
    release_date: Option<String>,
    changelog: Option<String>,
}

struct AppState {
    backend_process: Arc<Mutex<Option<Child>>>,
    frontend_process: Arc<Mutex<Option<Child>>>,
    backend_pid: Arc<Mutex<Option<u32>>>,
    frontend_pid: Arc<Mutex<Option<u32>>>,
}

// Windows下杀死进程树
#[cfg(target_os = "windows")]
fn kill_process_tree(pid: u32) {
    let _ = Command::new("taskkill")
        .args(&["/F", "/T", "/PID", &pid.to_string()])
        .creation_flags(0x08000000)
        .output();
}

#[cfg(not(target_os = "windows"))]
fn kill_process_tree(pid: u32) {
    let _ = Command::new("kill")
        .args(&["-9", &pid.to_string()])
        .output();
}

// 检查端口是否被占用
fn is_port_in_use(port: u16) -> bool {
    use std::net::{TcpListener, SocketAddr};
    use std::str::FromStr;
    
    let addresses = [
        format!("127.0.0.1:{}", port),
        format!("0.0.0.0:{}", port),
    ];
    
    for addr_str in &addresses {
        if let Ok(addr) = SocketAddr::from_str(addr_str) {
            if TcpListener::bind(addr).is_err() {
                return true;
            }
        }
    }
    
    false
}

// 移除ANSI转义序列
fn strip_ansi_codes(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&next_ch) = chars.peek() {
                    chars.next();
                    if next_ch.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}
// 读取配置文件
#[tauri::command]
async fn read_config() -> Result<Config, String> {
    let config_path = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("WebRPAConfig.json");
    
    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("读取配置文件失败: {}", e))?;
    
    let config: Config = serde_json::from_str(&content)
        .map_err(|e| format!("解析配置文件失败: {}", e))?;
    
    Ok(config)
}

// 保存配置文件
#[tauri::command]
async fn save_config(config: Config) -> Result<(), String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| e.to_string())?;

    let config_path = current_dir.join("WebRPAConfig.json");
    
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;
    
    std::fs::write(&config_path, &json)
        .map_err(|e| format!("保存配置文件失败: {}", e))?;

    // 同步写入前端 public 配置，确保 /WebRPAConfig.json 与启动器配置一致
    let frontend_public_config_path = current_dir
        .join("frontend")
        .join("public")
        .join("WebRPAConfig.json");

    if let Some(parent) = frontend_public_config_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建前端配置目录失败: {}", e))?;
    }

    std::fs::write(&frontend_public_config_path, &json)
        .map_err(|e| format!("同步前端配置文件失败: {}", e))?;
    
    Ok(())
}

// 启动后端服务
#[tauri::command]
async fn start_backend(_window: Window, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let config = read_config().await?;
    if is_port_in_use(config.backend.port) {
        return Err(format!("后端服务已在运行（端口{}已被占用）", config.backend.port));
    }
    
    let root_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let python_exe = root_dir.join("Python313").join("python.exe");
    let backend_script = root_dir.join("backend").join("run.py");
    
    if !python_exe.exists() {
        return Err(format!("未找到Python可执行文件，路径: {}", python_exe.display()));
    }
    
    if !backend_script.exists() {
        return Err(format!("未找到后端启动脚本，路径: {}", backend_script.display()));
    }
    
    // 创建日志目录并重置日志文件
    let log_dir = root_dir.join("backend").join("logs");
    std::fs::create_dir_all(&log_dir)
        .map_err(|e| format!("创建日志目录失败: {}", e))?;
    
    let log_file = log_dir.join("backend.log");
    
    let init_log = format!("# WebRPA 后端日志 - 启动时间: {}\n[{}] Python路径: {}\n[{}] 后端脚本: {}\n[{}] 工作目录: {}\n[{}] 配置: host={}, port={}\n", 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        python_exe.display(),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        backend_script.display(),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        root_dir.display(),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        config.backend.host,
        config.backend.port
    );
    std::fs::write(&log_file, init_log)
        .map_err(|e| format!("重置日志文件失败: {}", e))?;
    
    #[cfg(target_os = "windows")]
    let mut child = {
        let mut cmd = Command::new(&python_exe);
        cmd.arg(&backend_script)
            .current_dir(&root_dir)
            .env("PYTHONIOENCODING", "utf-8")
            .env("PYTHONUNBUFFERED", "1")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .creation_flags(0x08000000);
        
        let cmd_log = format!("[{}] 执行命令: {} {}\n", 
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            python_exe.display(),
            backend_script.display()
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .and_then(|mut file| {
                use std::io::Write;
                file.write_all(cmd_log.as_bytes())
            });
        
        cmd.spawn()
            .map_err(|e| format!("启动后端失败: {} (Python路径: {})", e, python_exe.display()))?
    };
    
    #[cfg(not(target_os = "windows"))]
    let mut child = Command::new(&python_exe)
        .arg(&backend_script)
        .current_dir(&root_dir)
        .env("PYTHONIOENCODING", "utf-8")
        .env("PYTHONUNBUFFERED", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("启动后端失败: {} (Python路径: {})", e, python_exe.display()))?;
    
    let pid = child.id();
    *state.backend_pid.lock().unwrap() = Some(pid);
    
    let pid_log = format!("[{}] 后端进程已启动，PID: {}\n", 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        pid
    );
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .and_then(|mut file| {
            use std::io::Write;
            file.write_all(pid_log.as_bytes())
        });
    
    // 处理stdout
    if let Some(stdout) = child.stdout.take() {
        let log_file_clone = log_file.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let clean_line = strip_ansi_codes(&line);
                    if !clean_line.trim().is_empty() {
                        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                        let log_entry = format!("[{}] {}\n", timestamp, clean_line);
                        let _ = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&log_file_clone)
                            .and_then(|mut file| {
                                use std::io::Write;
                                file.write_all(log_entry.as_bytes())
                            });
                    }
                }
            }
        });
    }
    
    // 处理stderr
    if let Some(stderr) = child.stderr.take() {
        let log_file_clone = log_file.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let clean_line = strip_ansi_codes(&line);
                    if !clean_line.trim().is_empty() {
                        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                        let log_entry = format!("[{}] [ERROR] {}\n", timestamp, clean_line);
                        let _ = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&log_file_clone)
                            .and_then(|mut file| {
                                use std::io::Write;
                                file.write_all(log_entry.as_bytes())
                            });
                    }
                }
            }
        });
    }
    
    *state.backend_process.lock().unwrap() = Some(child);
    Ok(())
}
// 启动前端服务
#[tauri::command]
async fn start_frontend(_window: Window, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let config = read_config().await?;
    if is_port_in_use(config.frontend.port) {
        return Err(format!("前端服务已在运行（端口{}已被占用）", config.frontend.port));
    }
    
    let root_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let frontend_dir = root_dir.join("frontend");
    
    if !frontend_dir.exists() {
        return Err(format!("未找到前端目录，路径: {}", frontend_dir.display()));
    }
    
    let log_dir = frontend_dir.join("logs");
    std::fs::create_dir_all(&log_dir)
        .map_err(|e| format!("创建日志目录失败: {}", e))?;
    
    let log_file = log_dir.join("frontend.log");
    std::fs::write(&log_file, format!("# WebRPA 前端日志 - 启动时间: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")))
        .map_err(|e| format!("重置日志文件失败: {}", e))?;
    
    let npm_cmd = root_dir.join("nodejs").join("npm.cmd");
    if !npm_cmd.exists() {
        return Err(format!("未找到npm.cmd可执行文件，路径: {}", npm_cmd.display()));
    }
    
    let package_json = frontend_dir.join("package.json");
    if !package_json.exists() {
        return Err(format!("未找到package.json文件，路径: {}", package_json.display()));
    }
    
    #[cfg(target_os = "windows")]
    let mut child = {
        let start_log = format!("[{}] 正在启动前端服务...\n[{}] npm路径: {}\n[{}] 工作目录: {}\n[{}] 执行命令: npm run dev\n", 
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            npm_cmd.display(),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            frontend_dir.display(),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .and_then(|mut file| {
                use std::io::Write;
                file.write_all(start_log.as_bytes())
            });
        
        if !npm_cmd.is_file() {
            return Err(format!("npm.cmd文件不存在或不是文件: {}", npm_cmd.display()));
        }
        
        let mut cmd = Command::new(&npm_cmd);
        cmd.args(&["run", "dev"])
            .current_dir(&frontend_dir)
            .env("NODE_OPTIONS", "--no-warnings")
            .env("FORCE_COLOR", "0")
            .env("NO_COLOR", "1")
            .env("PATH", format!("{};{}", root_dir.join("nodejs").display(), std::env::var("PATH").unwrap_or_default()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .creation_flags(0x08000000);
        
        cmd.spawn()
            .map_err(|e| format!("启动前端失败: {} (npm路径: {}, 工作目录: {})", e, npm_cmd.display(), frontend_dir.display()))?
    };
    
    #[cfg(not(target_os = "windows"))]
    let mut child = {
        let npm_cmd = root_dir.join("nodejs").join("npm");
        if !npm_cmd.exists() {
            return Err(format!("未找到npm可执行文件，路径: {}", npm_cmd.display()));
        }
        Command::new(&npm_cmd)
            .args(&["run", "dev"])
            .current_dir(&frontend_dir)
            .env("FORCE_COLOR", "0")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("启动前端失败: {} (npm路径: {}, 工作目录: {})", e, npm_cmd.display(), frontend_dir.display()))?
    };
    
    let pid = child.id();
    *state.frontend_pid.lock().unwrap() = Some(pid);
    
    let pid_log = format!("[{}] 前端进程已启动，PID: {}\n", 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        pid
    );
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .and_then(|mut file| {
            use std::io::Write;
            file.write_all(pid_log.as_bytes())
        });
    
    let log_file_clone = log_file.clone();
    let _frontend_process = state.frontend_process.clone();
    let _frontend_pid = state.frontend_pid.clone();
    
    thread::spawn(move || {
        if let Some(stdout) = child.stdout.take() {
            let log_file_stdout = log_file_clone.clone();
            let _stdout_thread = thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let clean_line = strip_ansi_codes(&line);
                        if !clean_line.trim().is_empty() {
                            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                            let log_entry = format!("[{}] {}\n", timestamp, clean_line);
                            let _ = std::fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(&log_file_stdout)
                                .and_then(|mut file| {
                                    use std::io::Write;
                                    file.write_all(log_entry.as_bytes())
                                });
                        }
                    }
                }
            });
        }
        
        if let Some(stderr) = child.stderr.take() {
            let log_file_stderr = log_file_clone.clone();
            let _stderr_thread = thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let clean_line = strip_ansi_codes(&line);
                        if !clean_line.trim().is_empty() {
                            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                            let log_entry = format!("[{}] [ERROR] {}\n", timestamp, clean_line);
                            let _ = std::fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(&log_file_stderr)
                                .and_then(|mut file| {
                                    use std::io::Write;
                                    file.write_all(log_entry.as_bytes())
                                });
                        }
                    }
                }
            });
        }
        
        match child.wait() {
            Ok(status) => {
                let exit_log = format!("[{}] 前端进程已退出，退出状态: {}\n", 
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    status
                );
                let _ = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_file_clone)
                    .and_then(|mut file| {
                        use std::io::Write;
                        file.write_all(exit_log.as_bytes())
                    });
                
                *_frontend_process.lock().unwrap() = None;
                *_frontend_pid.lock().unwrap() = None;
            }
            Err(e) => {
                let error_log = format!("[{}] 等待前端进程失败: {}\n", 
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    e
                );
                let _ = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_file_clone)
                    .and_then(|mut file| {
                        use std::io::Write;
                        file.write_all(error_log.as_bytes())
                    });
            }
        }
    });
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    if let Some(pid) = *state.frontend_pid.lock().unwrap() {
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("tasklist")
                .args(&["/FI", &format!("PID eq {}", pid)])
                .creation_flags(0x08000000)
                .output();
            
            match output {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    if !output_str.contains(&pid.to_string()) {
                        return Err("前端进程启动后立即退出，请检查日志文件".to_string());
                    }
                }
                Err(_) => {}
            }
        }
        
        let success_log = format!("[{}] 前端服务启动成功，正在监听端口 {}\n", 
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            config.frontend.port
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .and_then(|mut file| {
                use std::io::Write;
                file.write_all(success_log.as_bytes())
            });
    }
    
    Ok(())
}
// 根据端口查找并杀死进程
#[cfg(target_os = "windows")]
fn kill_processes_by_port(port: u16) -> Result<(), String> {
    let output = std::process::Command::new("netstat")
        .args(&["-ano"])
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| format!("执行netstat失败: {}", e))?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    let mut pids_to_kill = Vec::new();
    for line in output_str.lines() {
        if line.contains(&format!(":{}", port)) && line.contains("LISTENING") {
            if let Some(pid_str) = line.split_whitespace().last() {
                if let Ok(pid) = pid_str.parse::<u32>() {
                    if pid != 0 {
                        pids_to_kill.push(pid);
                    }
                }
            }
        }
    }
    
    for pid in pids_to_kill {
        let _ = std::process::Command::new("taskkill")
            .args(&["/F", "/T", "/PID", &pid.to_string()])
            .creation_flags(0x08000000)
            .output();
    }
    
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn kill_processes_by_port(port: u16) -> Result<(), String> {
    let output = std::process::Command::new("lsof")
        .args(&["-ti", &format!(":{}", port)])
        .output()
        .map_err(|e| format!("执行lsof失败: {}", e))?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    for line in output_str.lines() {
        if let Ok(pid) = line.trim().parse::<u32>() {
            let _ = std::process::Command::new("kill")
                .args(&["-9", &pid.to_string()])
                .output();
        }
    }
    
    Ok(())
}

// 停止服务
#[tauri::command]
async fn stop_services(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let config = read_config().await?;
    
    if let Some(pid) = state.backend_pid.lock().unwrap().take() {
        kill_process_tree(pid);
    }
    if let Some(mut child) = state.backend_process.lock().unwrap().take() {
        let _ = child.kill();
    }
    
    if let Some(pid) = state.frontend_pid.lock().unwrap().take() {
        kill_process_tree(pid);
    }
    if let Some(mut child) = state.frontend_process.lock().unwrap().take() {
        let _ = child.kill();
    }
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    if is_port_in_use(config.backend.port) {
        kill_processes_by_port(config.backend.port)?;
    }
    
    if is_port_in_use(config.frontend.port) {
        kill_processes_by_port(config.frontend.port)?;
    }
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    Ok(())
}

// 读取本地版本号
fn get_local_version() -> Result<String, String> {
    let version_file = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("frontend")
        .join("src")
        .join("services")
        .join("version.ts");
    
    if !version_file.exists() {
        return Err("版本文件不存在".to_string());
    }
    
    let content = std::fs::read_to_string(&version_file)
        .map_err(|e| format!("读取版本文件失败: {}", e))?;
    
    for line in content.lines() {
        if line.contains("CURRENT_VERSION") && line.contains("=") {
            if let Some(start) = line.find('\'') {
                if let Some(end) = line.rfind('\'') {
                    if start < end {
                        return Ok(line[start + 1..end].to_string());
                    }
                }
            }
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if start < end {
                        return Ok(line[start + 1..end].to_string());
                    }
                }
            }
        }
    }
    
    Err("无法从版本文件中提取版本号".to_string())
}

#[tauri::command]
async fn get_version() -> Result<String, String> {
    get_local_version()
}

#[tauri::command]
async fn check_service_status() -> Result<(bool, bool), String> {
    let config = read_config().await?;
    let backend_running = is_port_in_use(config.backend.port);
    let frontend_running = is_port_in_use(config.frontend.port);
    Ok((backend_running, frontend_running))
}

#[tauri::command]
async fn check_update(current_version: String) -> Result<VersionInfo, String> {
    let remote_url = "https://hub.pmhs.top/api/version";
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;
    
    let response = client.get(remote_url)
        .send()
        .await
        .map_err(|e| format!("获取远程版本信息失败: {}", e))?;
    
    let remote_info: RemoteVersionInfo = response.json()
        .await
        .map_err(|e| format!("解析远程版本信息失败: {}", e))?;
    
    let has_update = compare_versions(&current_version, &remote_info.version);
    
    let update_url = format!("https://github.com/pmh1314520/WebRPA/releases/tag/v{}", remote_info.version);
    
    Ok(VersionInfo {
        current_version,
        latest_version: remote_info.version,
        has_update,
        update_url,
        release_date: remote_info.release_date.unwrap_or_else(|| "未知".to_string()),
        changelog: remote_info.changelog.unwrap_or_else(|| "无更新说明".to_string()),
    })
}

fn compare_versions(local: &str, remote: &str) -> bool {
    let local_parts: Vec<u32> = local.split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    let remote_parts: Vec<u32> = remote.split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    for i in 0..local_parts.len().max(remote_parts.len()) {
        let local_part = local_parts.get(i).unwrap_or(&0);
        let remote_part = remote_parts.get(i).unwrap_or(&0);
        
        if remote_part > local_part {
            return true;
        } else if remote_part < local_part {
            return false;
        }
    }
    
    false
}
// 打开后端日志文件
#[tauri::command]
async fn open_backend_log() -> Result<(), String> {
    let log_path = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("backend")
        .join("logs")
        .join("backend.log");
    
    // 如果日志文件不存在，创建一个空的日志文件
    if !log_path.exists() {
        std::fs::create_dir_all(log_path.parent().unwrap())
            .map_err(|e| format!("创建日志目录失败: {}", e))?;
        std::fs::write(&log_path, "# WebRPA 后端日志\n# 日志文件将在服务启动后自动更新\n")
            .map_err(|e| format!("创建日志文件失败: {}", e))?;
    }
    
    // 使用file://协议在默认浏览器中打开日志文件
    let file_url = format!("file:///{}", log_path.to_string_lossy().replace("\\", "/"));
    
    #[cfg(target_os = "windows")]
    {
        // 在Windows上直接调用默认浏览器
        // 首先尝试获取默认浏览器路径
        let output = std::process::Command::new("reg")
            .args(&["query", "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\Shell\\Associations\\UrlAssociations\\http\\UserChoice", "/v", "ProgId"])
            .creation_flags(0x08000000)
            .output();
        
        if let Ok(output) = output {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if output_str.contains("ChromeHTML") {
                // Chrome浏览器
                std::process::Command::new("chrome")
                    .arg(&file_url)
                    .creation_flags(0x08000000)
                    .spawn()
                    .or_else(|_| {
                        // 如果chrome命令不存在，尝试完整路径
                        std::process::Command::new("C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe")
                            .arg(&file_url)
                            .creation_flags(0x08000000)
                            .spawn()
                    })
                    .or_else(|_| {
                        // 尝试用户目录下的Chrome
                        let user_chrome = format!("{}\\AppData\\Local\\Google\\Chrome\\Application\\chrome.exe", 
                            std::env::var("USERPROFILE").unwrap_or_default());
                        std::process::Command::new(&user_chrome)
                            .arg(&file_url)
                            .creation_flags(0x08000000)
                            .spawn()
                    })
                    .map_err(|e| format!("启动Chrome失败: {}", e))?;
            } else if output_str.contains("MSEdgeHTM") {
                // Edge浏览器
                std::process::Command::new("msedge")
                    .arg(&file_url)
                    .creation_flags(0x08000000)
                    .spawn()
                    .or_else(|_| {
                        std::process::Command::new("C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe")
                            .arg(&file_url)
                            .creation_flags(0x08000000)
                            .spawn()
                    })
                    .map_err(|e| format!("启动Edge失败: {}", e))?;
            } else {
                // 其他浏览器或回退方案，直接用explorer打开URL
                std::process::Command::new("explorer")
                    .arg(&file_url)
                    .creation_flags(0x08000000)
                    .spawn()
                    .map_err(|e| format!("启动浏览器失败: {}", e))?;
            }
        } else {
            // 如果无法获取默认浏览器，尝试常见浏览器
            std::process::Command::new("chrome")
                .arg(&file_url)
                .creation_flags(0x08000000)
                .spawn()
                .or_else(|_| {
                    std::process::Command::new("msedge")
                        .arg(&file_url)
                        .creation_flags(0x08000000)
                        .spawn()
                })
                .or_else(|_| {
                    std::process::Command::new("firefox")
                        .arg(&file_url)
                        .creation_flags(0x08000000)
                        .spawn()
                })
                .or_else(|_| {
                    // 最后回退到explorer
                    std::process::Command::new("explorer")
                        .arg(&file_url)
                        .creation_flags(0x08000000)
                        .spawn()
                })
                .map_err(|e| format!("启动浏览器失败: {}", e))?;
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("xdg-open")
            .arg(&file_url)
            .spawn()
            .map_err(|e| format!("打开日志文件失败: {}", e))?;
    }
    
    Ok(())
}

// 打开前端日志文件
#[tauri::command]
async fn open_frontend_log() -> Result<(), String> {
    let log_path = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("frontend")
        .join("logs")
        .join("frontend.log");
    
    // 如果日志文件不存在，创建一个空的日志文件
    if !log_path.exists() {
        std::fs::create_dir_all(log_path.parent().unwrap())
            .map_err(|e| format!("创建日志目录失败: {}", e))?;
        std::fs::write(&log_path, "# WebRPA 前端日志\n# 日志文件将在服务启动后自动更新\n")
            .map_err(|e| format!("创建日志文件失败: {}", e))?;
    }
    
    // 使用file://协议在默认浏览器中打开日志文件
    let file_url = format!("file:///{}", log_path.to_string_lossy().replace("\\", "/"));
    
    #[cfg(target_os = "windows")]
    {
        // 在Windows上直接调用默认浏览器
        // 首先尝试获取默认浏览器路径
        let output = std::process::Command::new("reg")
            .args(&["query", "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\Shell\\Associations\\UrlAssociations\\http\\UserChoice", "/v", "ProgId"])
            .creation_flags(0x08000000)
            .output();
        
        if let Ok(output) = output {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if output_str.contains("ChromeHTML") {
                // Chrome浏览器
                std::process::Command::new("chrome")
                    .arg(&file_url)
                    .creation_flags(0x08000000)
                    .spawn()
                    .or_else(|_| {
                        // 如果chrome命令不存在，尝试完整路径
                        std::process::Command::new("C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe")
                            .arg(&file_url)
                            .creation_flags(0x08000000)
                            .spawn()
                    })
                    .or_else(|_| {
                        // 尝试用户目录下的Chrome
                        let user_chrome = format!("{}\\AppData\\Local\\Google\\Chrome\\Application\\chrome.exe", 
                            std::env::var("USERPROFILE").unwrap_or_default());
                        std::process::Command::new(&user_chrome)
                            .arg(&file_url)
                            .creation_flags(0x08000000)
                            .spawn()
                    })
                    .map_err(|e| format!("启动Chrome失败: {}", e))?;
            } else if output_str.contains("MSEdgeHTM") {
                // Edge浏览器
                std::process::Command::new("msedge")
                    .arg(&file_url)
                    .creation_flags(0x08000000)
                    .spawn()
                    .or_else(|_| {
                        std::process::Command::new("C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe")
                            .arg(&file_url)
                            .creation_flags(0x08000000)
                            .spawn()
                    })
                    .map_err(|e| format!("启动Edge失败: {}", e))?;
            } else {
                // 其他浏览器或回退方案，直接用explorer打开URL
                std::process::Command::new("explorer")
                    .arg(&file_url)
                    .creation_flags(0x08000000)
                    .spawn()
                    .map_err(|e| format!("启动浏览器失败: {}", e))?;
            }
        } else {
            // 如果无法获取默认浏览器，尝试常见浏览器
            std::process::Command::new("chrome")
                .arg(&file_url)
                .creation_flags(0x08000000)
                .spawn()
                .or_else(|_| {
                    std::process::Command::new("msedge")
                        .arg(&file_url)
                        .creation_flags(0x08000000)
                        .spawn()
                })
                .or_else(|_| {
                    std::process::Command::new("firefox")
                        .arg(&file_url)
                        .creation_flags(0x08000000)
                        .spawn()
                })
                .or_else(|_| {
                    // 最后回退到explorer
                    std::process::Command::new("explorer")
                        .arg(&file_url)
                        .creation_flags(0x08000000)
                        .spawn()
                })
                .map_err(|e| format!("启动浏览器失败: {}", e))?;
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("xdg-open")
            .arg(&file_url)
            .spawn()
            .map_err(|e| format!("打开日志文件失败: {}", e))?;
    }
    
    Ok(())
}

// 打开浏览器
#[tauri::command]
async fn open_browser(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/c", "start", &url])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .spawn()
            .map_err(|e| format!("打开浏览器失败: {}", e))?;
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("打开浏览器失败: {}", e))?;
    }
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            backend_process: Arc::new(Mutex::new(None)),
            frontend_process: Arc::new(Mutex::new(None)),
            backend_pid: Arc::new(Mutex::new(None)),
            frontend_pid: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            read_config,
            save_config,
            start_backend,
            start_frontend,
            stop_services,
            check_update,
            check_service_status,
            open_browser,
            get_version,
            open_backend_log,
            open_frontend_log
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // 窗口关闭时停止所有服务
                let app_handle = window.app_handle();
                if let Some(state) = app_handle.try_state::<AppState>() {
                    // 使用进程树杀死后端
                    if let Some(pid) = state.backend_pid.lock().unwrap().take() {
                        kill_process_tree(pid);
                    }
                    if let Some(mut child) = state.backend_process.lock().unwrap().take() {
                        let _ = child.kill();
                    }
                    
                    // 使用进程树杀死前端
                    if let Some(pid) = state.frontend_pid.lock().unwrap().take() {
                        kill_process_tree(pid);
                    }
                    if let Some(mut child) = state.frontend_process.lock().unwrap().take() {
                        let _ = child.kill();
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}