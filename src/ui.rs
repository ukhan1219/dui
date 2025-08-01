use colored::*;
use std::io::{self, Write};
use crate::docker::{Container, Image, ContainerStats};

pub struct UserInterface;

impl UserInterface {
    pub fn new() -> Self {
        UserInterface
    }

    pub fn show_welcome(&self) {
        println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan());
        println!("{}", "║                    Docker GUI CLI v1.0.0                    ║".cyan());
        println!("{}", "║              Intuitive Docker Management Tool               ║".cyan());
        println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan());
        println!();
    }

    pub fn show_help(&self) {
        println!("{}", "Available Commands:".yellow().bold());
        println!();
        println!("  {} {}", "containers".green().bold(), "list|start|stop|remove|logs [name]".dimmed());
        println!("    Manage Docker containers");
        println!();
        println!("  {} {}", "images".green().bold(), "list|pull|remove [name]".dimmed());
        println!("    Manage Docker images");
        println!();
        println!("  {} {}", "monitor".green().bold(), "stats|system|events".dimmed());
        println!("    Monitor Docker resources");
        println!();
        println!("  {} {}", "interactive".green().bold(), "".dimmed());
        println!("    Launch interactive mode");
        println!();
        println!("{}", "Examples:".yellow().bold());
        println!("  docker-cli containers list");
        println!("  docker-cli containers start my-container");
        println!("  docker-cli images pull nginx:latest");
        println!("  docker-cli monitor stats");
        println!("  docker-cli interactive");
        println!();
    }

    pub fn show_interactive_help(&self) {
        println!("{}", "Interactive Mode Commands:".yellow().bold());
        println!("  {} - List all containers", "containers".green());
        println!("  {} - List all images", "images".green());
        println!("  {} - Show container statistics", "stats".green());
        println!("  {} - Show this help", "help".green());
        println!("  {} - Exit interactive mode", "exit".green());
        println!();
    }

    pub fn show_loading(&self, message: &str) {
        print!("{} {} ", "⏳".yellow(), message.dimmed());
        io::stdout().flush().unwrap();
        println!();
    }

    pub fn show_success(&self, message: &str) {
        println!("{} {}", "✅".green(), message.green());
    }

    pub fn show_error(&self, message: &str) {
        println!("{} {}", "❌".red(), message.red());
    }

    pub fn show_info(&self, message: &str) {
        println!("{} {}", "ℹ️".blue(), message.blue());
    }

    pub fn confirm(&self, message: &str) -> bool {
        print!("{} {} (y/N): ", "❓".yellow(), message.yellow());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    }

    pub fn display_containers(&self, containers: &[Container]) {
        if containers.is_empty() {
            self.show_info("No containers found.");
            return;
        }

        println!();
        println!("{}", "📦 Docker Containers".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());
        
        // Header
        println!(
            "{:<12} {:<20} {:<25} {:<15} {:<20}",
            "ID".bold(),
            "NAME".bold(),
            "IMAGE".bold(),
            "STATUS".bold(),
            "PORTS".bold()
        );
        println!("{}", "─".repeat(80).dimmed());

        for container in containers {
            let status_color = if container.status.contains("Up") {
                container.status.green()
            } else {
                container.status.red()
            };

            println!(
                "{:<12} {:<20} {:<25} {:<15} {:<20}",
                container.id[..12.min(container.id.len())].dimmed(),
                container.name.white(),
                container.image.cyan(),
                status_color,
                container.ports.dimmed()
            );
        }
        println!();
    }

    pub fn display_images(&self, images: &[Image]) {
        if images.is_empty() {
            self.show_info("No images found.");
            return;
        }

        println!();
        println!("{}", "🖼️  Docker Images".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());
        
        // Header
        println!(
            "{:<12} {:<25} {:<10} {:<12} {:<20}",
            "ID".bold(),
            "REPOSITORY".bold(),
            "TAG".bold(),
            "SIZE".bold(),
            "CREATED".bold()
        );
        println!("{}", "─".repeat(80).dimmed());

        for image in images {
            println!(
                "{:<12} {:<25} {:<10} {:<12} {:<20}",
                image.id[..12.min(image.id.len())].dimmed(),
                image.repository.white(),
                image.tag.cyan(),
                image.size.yellow(),
                image.created.dimmed()
            );
        }
        println!();
    }

    pub fn display_stats(&self, stats: &[ContainerStats]) {
        if stats.is_empty() {
            self.show_info("No running containers to show stats for.");
            return;
        }

        println!();
        println!("{}", "📊 Container Statistics".cyan().bold());
        println!("{}", "─".repeat(90).dimmed());
        
        // Header
        println!(
            "{:<20} {:<10} {:<20} {:<10} {:<15} {:<15}",
            "NAME".bold(),
            "CPU %".bold(),
            "MEMORY USAGE".bold(),
            "MEM %".bold(),
            "NET I/O".bold(),
            "BLOCK I/O".bold()
        );
        println!("{}", "─".repeat(90).dimmed());

        for stat in stats {
            let cpu_color = if stat.cpu_percent.replace('%', "").parse::<f32>().unwrap_or(0.0) > 50.0 {
                stat.cpu_percent.red()
            } else {
                stat.cpu_percent.green()
            };

            let mem_color = if stat.memory_percent.replace('%', "").parse::<f32>().unwrap_or(0.0) > 80.0 {
                stat.memory_percent.red()
            } else {
                stat.memory_percent.green()
            };

            println!(
                "{:<20} {:<10} {:<20} {:<10} {:<15} {:<15}",
                stat.name.white(),
                cpu_color,
                stat.memory_usage.yellow(),
                mem_color,
                stat.network_io.cyan(),
                stat.block_io.dimmed()
            );
        }
        println!();
    }

    pub fn display_logs(&self, logs: &str) {
        println!();
        println!("{}", "📋 Container Logs".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());
        
        if logs.trim().is_empty() {
            self.show_info("No logs available.");
        } else {
            for line in logs.lines().take(50) {
                println!("{}", line.dimmed());
            }
        }
        println!();
    }

    pub fn display_system_info(&self, info: &str) {
        println!();
        println!("{}", "🖥️  Docker System Information".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());
        
        // Parse and display key information
        for line in info.lines().take(20) {
            if line.contains(':') {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    println!("{}: {}", parts[0].trim().yellow(), parts[1].trim().white());
                }
            }
        }
        println!();
    }
}
