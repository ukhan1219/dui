use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
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

    pub fn get_container_info(&self, name: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(&["inspect", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn get_container_size(&self, name: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(&["ps", "-s", "--format", "json", "--filter", &format!("name={}", name)])
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
        let output = Command::new("docker")
            .args(&["ps", "-a", "--format", "json"])
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

    pub fn list_images(&self) -> Result<Vec<Image>, String> {
        let output = Command::new("docker")
            .args(&["images", "--format", "json"])
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

    pub fn start_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["start", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn stop_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["stop", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn remove_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["rm", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn pull_image(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["pull", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn remove_image(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["rmi", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn get_container_logs(&self, name: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(&["logs", "--tail", "50", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn get_container_stats(&self) -> Result<Vec<ContainerStats>, String> {
        let output = Command::new("docker")
            .args(&["stats", "--no-stream", "--format", "json"])
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

    pub fn get_system_info(&self) -> Result<String, String> {
        let output = Command::new("docker")
            .args(&["system", "info"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn monitor_events(&self) -> Result<(), String> {
        let mut child = Command::new("docker")
            .args(&["events"])
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

    pub fn restart_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["restart", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn pause_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["pause", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn unpause_container(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["unpause", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn exec_container(&self, name: &str, command: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(&["exec", name, "sh", "-c", command])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn inspect_container(&self, name: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .args(&["inspect", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn list_networks(&self) -> Result<Vec<Network>, String> {
        let output = Command::new("docker")
            .args(&["network", "ls", "--format", "json"])
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

    pub fn list_volumes(&self) -> Result<Vec<Volume>, String> {
        let output = Command::new("docker")
            .args(&["volume", "ls", "--format", "json"])
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

    pub fn build_image(&self, path: &str, tag: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["build", "-t", tag, path])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn tag_image(&self, source: &str, target: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["tag", source, target])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }

    pub fn push_image(&self, name: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["push", name])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }
}
