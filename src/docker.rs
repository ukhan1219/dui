use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use serde_json::Value;
use crate::utils::parse_docker_output;

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

    pub fn list_containers(&self) -> Result<Vec<Container>, String> {
        let output = Command::new("docker")
            .args(&["ps", "-a", "--format", "table {{.ID}}\\t{{.Names}}\\t{{.Image}}\\t{{.Status}}\\t{{.Ports}}"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut containers = Vec::new();

        for (i, line) in output_str.lines().enumerate() {
            if i == 0 { continue; } // Skip header
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                containers.push(Container {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    image: parts[2].to_string(),
                    status: parts[3].to_string(),
                    ports: parts[4].to_string(),
                });
            }
        }

        Ok(containers)
    }

    pub fn list_images(&self) -> Result<Vec<Image>, String> {
        let output = Command::new("docker")
            .args(&["images", "--format", "table {{.ID}}\\t{{.Repository}}\\t{{.Tag}}\\t{{.Size}}\\t{{.CreatedAt}}"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut images = Vec::new();

        for (i, line) in output_str.lines().enumerate() {
            if i == 0 { continue; } // Skip header
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                images.push(Image {
                    id: parts[0].to_string(),
                    repository: parts[1].to_string(),
                    tag: parts[2].to_string(),
                    size: parts[3].to_string(),
                    created: parts[4].to_string(),
                });
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
            .args(&["stats", "--no-stream", "--format", "table {{.Name}}\\t{{.CPUPerc}}\\t{{.MemUsage}}\\t{{.MemPerc}}\\t{{.NetIO}}\\t{{.BlockIO}}"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut stats = Vec::new();

        for (i, line) in output_str.lines().enumerate() {
            if i == 0 { continue; } // Skip header
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 6 {
                stats.push(ContainerStats {
                    name: parts[0].to_string(),
                    cpu_percent: parts[1].to_string(),
                    memory_usage: parts[2].to_string(),
                    memory_percent: parts[3].to_string(),
                    network_io: parts[4].to_string(),
                    block_io: parts[5].to_string(),
                });
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
}
