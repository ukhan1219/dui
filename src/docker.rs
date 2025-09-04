use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use crate::utils::{validate_container_name, validate_image_name, format_size};

#[derive(Clone)]
pub struct DockerClient;

#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
    pub created: String,
}

#[derive(Debug, Clone)]
pub struct ContainerStats {
    pub name: String,
    pub cpu_percent: String,
    pub memory_usage: String,
    pub memory_percent: String,
    pub network_io: String,
    pub block_io: String,
}

#[derive(Debug, Clone)]
pub struct Network {
    pub id: String,
    pub name: String,
    pub driver: String,
    pub scope: String,
}

#[derive(Debug, Clone)]
pub struct Volume {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
}

#[derive(Debug, Clone)]
pub struct ContainerProcess {
    pub user: String,
    pub pid: String,
    pub ppid: String,
    pub cpu: String,
    pub mem: String,
    pub vsz: String,
    pub rss: String,
    pub tty: String,
    pub stat: String,
    pub start: String,
    pub time: String,
    pub command: String,
}

impl DockerClient {
    pub fn new() -> Self {
        DockerClient
    }

    pub fn is_docker_available(&self) -> bool {
        Command::new("docker")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn is_docker_daemon_running(&self) -> bool {
        Command::new("docker")
            .args(["info"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn start_docker_daemon(&self) -> Result<(), String> {
        let os = std::env::consts::OS;
        
        match os {
            "macos" => self.start_docker_daemon_macos(),
            "linux" => self.start_docker_daemon_linux(),
            "windows" => self.start_docker_daemon_windows(),
            _ => Err(format!("Unsupported operating system: {}", os)),
        }
    }

    fn start_docker_daemon_macos(&self) -> Result<(), String> {
        // Try to start Docker Desktop on macOS
        let docker_desktop_paths = [
            "/Applications/Docker.app",
            "/System/Applications/Docker.app",
        ];

        for path in &docker_desktop_paths {
            if std::path::Path::new(path).exists() {
                Command::new("open")
                    .arg("-a")
                    .arg(path)
                    .output()
                    .map_err(|e| format!("Failed to start Docker Desktop: {}", e))?;
                
                // Wait for Docker daemon to start
                return self.wait_for_docker_daemon();
            }
        }

        // Try using brew services if Docker Desktop is not found
        let output = Command::new("brew")
            .args(["services", "start", "docker"])
            .output();

        match output {
            Ok(output) if output.status.success() => self.wait_for_docker_daemon(),
            _ => Err("Docker Desktop not found. Please install Docker Desktop or start Docker daemon manually.".to_string()),
        }
    }

    fn start_docker_daemon_linux(&self) -> Result<(), String> {
        // Try systemctl first (most common on modern Linux distros)
        let systemctl_result = Command::new("sudo")
            .args(["systemctl", "start", "docker"])
            .output();

        if let Ok(output) = systemctl_result {
            if output.status.success() {
                return self.wait_for_docker_daemon();
            }
        }

        // Try service command as fallback
        let service_result = Command::new("sudo")
            .args(["service", "docker", "start"])
            .output();

        if let Ok(output) = service_result {
            if output.status.success() {
                return self.wait_for_docker_daemon();
            }
        }

        // Try direct daemon start as last resort
        let daemon_result = Command::new("sudo")
            .args(["dockerd", "&"])
            .output();

        match daemon_result {
            Ok(output) if output.status.success() => self.wait_for_docker_daemon(),
            _ => Err("Failed to start Docker daemon. Please start Docker manually or check your Docker installation.".to_string()),
        }
    }

    fn start_docker_daemon_windows(&self) -> Result<(), String> {
        // Try to start Docker Desktop on Windows
        let docker_desktop_paths = [
            "C:\\Program Files\\Docker\\Docker\\Docker Desktop.exe",
            "C:\\Program Files (x86)\\Docker\\Docker\\Docker Desktop.exe",
        ];

        for path in &docker_desktop_paths {
            if std::path::Path::new(path).exists() {
                Command::new("cmd")
                    .args(["/C", "start", "", path])
                    .output()
                    .map_err(|e| format!("Failed to start Docker Desktop: {}", e))?;
                
                return self.wait_for_docker_daemon();
            }
        }

        // Try PowerShell command
        let ps_result = Command::new("powershell")
            .args(["-Command", "Start-Service", "docker"])
            .output();

        match ps_result {
            Ok(output) if output.status.success() => self.wait_for_docker_daemon(),
            _ => Err("Docker Desktop not found. Please install Docker Desktop or start Docker service manually.".to_string()),
        }
    }

    fn wait_for_docker_daemon(&self) -> Result<(), String> {
        let max_attempts = 30;
        let delay = Duration::from_secs(2);
        
        for attempt in 1..=max_attempts {
            if self.is_docker_daemon_running() {
                return Ok(());
            }
            
            if attempt < max_attempts {
                thread::sleep(delay);
            }
        }
        
        Err("Docker daemon failed to start within expected time (60 seconds). Please check Docker installation.".to_string())
    }

    pub fn ensure_docker_is_running(&self) -> Result<(), String> {
        // First check if docker command is available
        if !self.is_docker_available() {
            return Err("Docker is not installed. Please install Docker first.".to_string());
        }

        // Check if daemon is running
        if self.is_docker_daemon_running() {
            return Ok(());
        }

        // Try to start the daemon
        self.start_docker_daemon()
    }

    // ===== CONTAINER COMMANDS =====

    pub fn create_container(&self, name: &str, image: &str, ports: Option<&str>, volumes: Option<&str>, env: Option<&str>) -> Result<(), String> {
        // Validate container name
        validate_container_name(name)?;
        
        // Validate image name
        validate_image_name(image)?;

        let mut args = vec!["run", "-d"];
        
        // Add name
        args.extend_from_slice(&["--name", name]);
        
        // Add port mapping if provided
        if let Some(port_mapping) = ports {
            args.extend_from_slice(&["-p", port_mapping]);
        }
        
        // Add volume mapping if provided
        if let Some(volume_mapping) = volumes {
            args.extend_from_slice(&["-v", volume_mapping]);
        }
        
        // Add environment variables if provided
        if let Some(env_vars) = env {
            args.extend_from_slice(&["-e", env_vars]);
        }
        
        // Add image
        args.push(image);

        let output = Command::new("docker")
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn attach_container(&self, name: &str) -> Result<(), String> {
        let mut child = Command::new("docker")
            .args(["attach", name])
            .spawn()
            .map_err(|e| format!("Failed to attach to container: {}", e))?;

        child.wait()
            .map_err(|e| format!("Failed to wait for attach process: {}", e))?;

        Ok(())
    }

    pub fn commit_container(&self, container: &str, repository: &str, tag: Option<&str>) -> Result<(), String> {
        let mut args = vec!["commit"];
        
        if let Some(tag_value) = tag {
            args.extend_from_slice(&[repository, tag_value]);
        } else {
            args.push(repository);
        }
        
        args.push(container);

        let output = Command::new("docker")
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn copy_from_container(&self, container: &str, src_path: &str, dest_path: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["cp", &format!("{}:{}", container, src_path), dest_path])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn copy_to_container(&self, src_path: &str, container: &str, dest_path: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["cp", src_path, &format!("{}:{}", container, dest_path)])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn diff_container(&self, container: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(["diff", container])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn export_container(&self, container: &str, output_file: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["export", "-o", output_file, container])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn get_container_history(&self, image: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(["history", image])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn import_image(&self, file: &str, repository: &str, tag: Option<&str>) -> Result<(), String> {
        let mut args = vec!["import"];
        
        if let Some(tag_value) = tag {
            args.extend_from_slice(&[repository, tag_value]);
        } else {
            args.push(repository);
        }
        
        args.push(file);

        let output = Command::new("docker")
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn kill_container(&self, container: &str, signal: Option<&str>) -> Result<(), String> {
        let mut args = vec!["kill"];
        
        if let Some(sig) = signal {
            args.extend_from_slice(&["-s", sig]);
        }
        
        args.push(container);

        let output = Command::new("docker")
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn load_image(&self, file: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["load", "-i", file])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn get_container_ports(&self, container: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(["port", container])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn rename_container(&self, old_name: &str, new_name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["rename", old_name, new_name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn save_image(&self, image: &str, output_file: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["save", "-o", output_file, image])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn get_container_processes(&self, container: &str) -> Result<Vec<ContainerProcess>, String> {
        let output = Command::new("docker")
            .args(["top", container])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut processes = Vec::new();

        // Skip header line
        for line in output_str.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 11 {
                processes.push(ContainerProcess {
                    user: parts[0].to_string(),
                    pid: parts[1].to_string(),
                    ppid: parts[2].to_string(),
                    cpu: parts[3].to_string(),
                    mem: parts[4].to_string(),
                    vsz: parts[5].to_string(),
                    rss: parts[6].to_string(),
                    tty: parts[7].to_string(),
                    stat: parts[8].to_string(),
                    start: parts[9].to_string(),
                    time: parts[10].to_string(),
                    command: parts[11..].join(" "),
                });
            }
        }

        Ok(processes)
    }

    pub fn update_container(&self, container: &str, cpu_period: Option<&str>, cpu_quota: Option<&str>, 
                          memory: Option<&str>, memory_swap: Option<&str>) -> Result<(), String> {
        let mut args = vec!["update"];
        
        if let Some(period) = cpu_period {
            args.extend_from_slice(&["--cpu-period", period]);
        }
        
        if let Some(quota) = cpu_quota {
            args.extend_from_slice(&["--cpu-quota", quota]);
        }
        
        if let Some(mem) = memory {
            args.extend_from_slice(&["--memory", mem]);
        }
        
        if let Some(mem_swap) = memory_swap {
            args.extend_from_slice(&["--memory-swap", mem_swap]);
        }
        
        args.push(container);

        let output = Command::new("docker")
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn wait_for_container(&self, container: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(["wait", container])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    // ===== EXISTING CONTAINER COMMANDS =====

    pub fn get_container_info(&self, name: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(["inspect", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn get_container_size(&self, name: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(["ps", "-s", "--format", "json", "--filter", &format!("name={}", name)])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let Some(size) = json.get("Size").and_then(|v| v.as_str()) {
                        // Parse size and format it
                        if let Ok(size_bytes) = size.parse::<u64>() {
                            return Ok(format_size(size_bytes));
                        }
                        return Ok(size.to_string());
                    }
                }
                Err(_) => continue,
            }
        }

        Err("Container not found or size information unavailable".to_string())
    }

    pub fn list_containers(&self) -> Result<Vec<Container>, String> {
        // Ensure Docker is running before attempting command
        self.ensure_docker_is_running()?;
        
        let output = Command::new("docker")
            .args(["ps", "-a", "--format", "json"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Docker command failed: {}", stderr));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut containers = Vec::new();

        // Parse JSON output - each line is a separate JSON object
        for line in output_str.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let (Some(id), Some(names), Some(image), Some(status), ports) = (
                        json.get("ID").and_then(|v| v.as_str()),
                        json.get("Names").and_then(|v| v.as_str()),
                        json.get("Image").and_then(|v| v.as_str()),
                        json.get("Status").and_then(|v| v.as_str()),
                        json.get("Ports").and_then(|v| v.as_str()).unwrap_or("")
                    ) {
                        containers.push(Container {
                            id: id.to_string(),
                            name: names.to_string(),
                            image: image.to_string(),
                            status: status.to_string(),
                            ports: ports.to_string(),
                        });
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse container JSON: {} for line: {}", e, line);
                }
            }
        }

        Ok(containers)
    }

    pub fn start_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["start", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn stop_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["stop", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn remove_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["rm", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn get_container_logs(&self, name: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(["logs", "--tail", "50", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn get_container_stats(&self) -> Result<Vec<ContainerStats>, String> {
        // Ensure Docker is running before attempting command
        self.ensure_docker_is_running()?;
        
        let output = Command::new("docker")
            .args(["stats", "--no-stream", "--format", "json"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Docker command failed: {}", stderr));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut stats = Vec::new();

        // Parse JSON output - each line is a separate JSON object
        for line in output_str.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let (Some(name), Some(cpu), Some(mem_usage), Some(mem_perc), Some(net_io), Some(block_io)) = (
                        json.get("Name").and_then(|v| v.as_str()),
                        json.get("CPUPerc").and_then(|v| v.as_str()),
                        json.get("MemUsage").and_then(|v| v.as_str()),
                        json.get("MemPerc").and_then(|v| v.as_str()),
                        json.get("NetIO").and_then(|v| v.as_str()),
                        json.get("BlockIO").and_then(|v| v.as_str())
                    ) {
                        stats.push(ContainerStats {
                            name: name.to_string(),
                            cpu_percent: cpu.to_string(),
                            memory_usage: mem_usage.to_string(),
                            memory_percent: mem_perc.to_string(),
                            network_io: net_io.to_string(),
                            block_io: block_io.to_string(),
                        });
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse stats JSON: {} for line: {}", e, line);
                }
            }
        }

        Ok(stats)
    }

    pub fn restart_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["restart", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn pause_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["pause", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn unpause_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["unpause", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn exec_container(&self, name: &str, command: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(["exec", name, "sh", "-c", command])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn inspect_container(&self, name: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(["inspect", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    // ===== IMAGE COMMANDS =====

    pub fn list_images(&self) -> Result<Vec<Image>, String> {
        // Ensure Docker is running before attempting command
        self.ensure_docker_is_running()?;
        
        let output = Command::new("docker")
            .args(["images", "--format", "json"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Docker command failed: {}", stderr));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut images = Vec::new();

        // Parse JSON output - each line is a separate JSON object
        for line in output_str.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let (Some(id), Some(repo), Some(tag), Some(size), Some(created)) = (
                        json.get("ID").and_then(|v| v.as_str()),
                        json.get("Repository").and_then(|v| v.as_str()),
                        json.get("Tag").and_then(|v| v.as_str()),
                        json.get("Size").and_then(|v| v.as_str()),
                        json.get("CreatedAt").and_then(|v| v.as_str())
                    ) {
                        images.push(Image {
                            id: id.to_string(),
                            repository: repo.to_string(),
                            tag: tag.to_string(),
                            size: size.to_string(),
                            created: created.to_string(),
                        });
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse image JSON: {} for line: {}", e, line);
                }
            }
        }

        Ok(images)
    }

    pub fn pull_image(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["pull", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn remove_image(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["rmi", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn build_image(&self, path: &str, tag: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["build", "-t", tag, path])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn tag_image(&self, source: &str, target: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["tag", source, target])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn push_image(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(["push", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    // ===== SYSTEM COMMANDS =====

    pub fn get_system_info(&self) -> Result<String, String> {
        // Ensure Docker is running before attempting command
        self.ensure_docker_is_running()?;
        
        let output = Command::new("docker")
            .args(["system", "info"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn monitor_events(&self) -> Result<(), String> {
        let mut child = Command::new("docker")
            .args(["events"])
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start docker events: {}", e))?;

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(event) => println!("{}", event),
                    Err(e) => eprintln!("Error reading event: {}", e),
                }
            }
        }

        Ok(())
    }

    // ===== NETWORK COMMANDS =====

    pub fn list_networks(&self) -> Result<Vec<Network>, String> {
        let output = Command::new("docker")
            .args(["network", "ls", "--format", "json"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Docker command failed: {}", stderr));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut networks = Vec::new();

        for line in output_str.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let (Some(id), Some(name), Some(driver), Some(scope)) = (
                        json.get("ID").and_then(|v| v.as_str()),
                        json.get("Name").and_then(|v| v.as_str()),
                        json.get("Driver").and_then(|v| v.as_str()),
                        json.get("Scope").and_then(|v| v.as_str())
                    ) {
                        networks.push(Network {
                            id: id.to_string(),
                            name: name.to_string(),
                            driver: driver.to_string(),
                            scope: scope.to_string(),
                        });
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse network JSON: {} for line: {}", e, line);
                }
            }
        }

        Ok(networks)
    }

    // ===== VOLUME COMMANDS =====

    pub fn list_volumes(&self) -> Result<Vec<Volume>, String> {
        let output = Command::new("docker")
            .args(["volume", "ls", "--format", "json"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Docker command failed: {}", stderr));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut volumes = Vec::new();

        for line in output_str.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let (Some(name), Some(driver), Some(mountpoint)) = (
                        json.get("Name").and_then(|v| v.as_str()),
                        json.get("Driver").and_then(|v| v.as_str()),
                        json.get("Mountpoint").and_then(|v| v.as_str())
                    ) {
                        volumes.push(Volume {
                            name: name.to_string(),
                            driver: driver.to_string(),
                            mountpoint: mountpoint.to_string(),
                        });
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse volume JSON: {} for line: {}", e, line);
                }
            }
        }

        Ok(volumes)
    }
}
