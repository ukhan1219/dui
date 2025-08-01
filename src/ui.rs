use colored::*;
use std::io::{self, Write};
use crate::docker::{Container, Image, ContainerStats, Network, Volume, ContainerProcess};

pub struct UserInterface;

impl UserInterface {
    pub fn new() -> Self {
        UserInterface
    }

    pub fn show_help(&self) {
        // Rainbow colors for ASCII art
        let colors = [
            "magenta"
        ];
        
        // ASCII art lines
        let ascii_lines = [
            "",
            "",
            "",
            "░███████   ░██     ░██ ░██████",
            "░██   ░██  ░██     ░██   ░██  ",
            "░██    ░██ ░██     ░██   ░██  ",
            "░██    ░██ ░██     ░██   ░██  ",
            "░██    ░██ ░██     ░██   ░██  ",
            "░██   ░██   ░██   ░██    ░██  ",
            "░███████     ░██████   ░██████",
            "",
            "",
            ""
        ];
        
        // Print ASCII art with rainbow colors
        for (i, line) in ascii_lines.iter().enumerate() {
            let color = colors[i % colors.len()];
            match color {
                
                "magenta" => println!("{}", line.magenta().bold()),
                _ => println!("{}", line.white().bold()),
            }
        }
        
        println!();
        
        println!("{}", "📋 Available Commands:".yellow().bold());
        println!();
        
        // Container Management Section
        println!("{}", "🐳 CONTAINER MANAGEMENT".green().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("  {} {} {}", "list".green().bold(), "".dimmed(), "List all containers (running and stopped)".white());
        println!("  {} {} {}", "create".green().bold(), "<name> <image>".dimmed(), "Create a new container".white());
        println!("  {} {} {}", "start".green().bold(), "<name>".dimmed(), "Start a stopped container".white());
        println!("  {} {} {}", "stop".green().bold(), "<name>".dimmed(), "Stop a running container".white());
        println!("  {} {} {}", "restart".green().bold(), "<name>".dimmed(), "Restart a container".white());
        println!("  {} {} {}", "pause".green().bold(), "<name>".dimmed(), "Pause a running container".white());
        println!("  {} {} {}", "unpause".green().bold(), "<name>".dimmed(), "Unpause a paused container".white());
        println!("  {} {} {}", "remove".green().bold(), "<name>".dimmed(), "Remove a container (will prompt for confirmation)".white());
        println!("  {} {} {}", "logs".green().bold(), "<name>".dimmed(), "Show container logs (last 50 lines)".white());
        println!("  {} {} {}", "exec".green().bold(), "<name> <cmd>".dimmed(), "Execute command in container".white());
        println!("  {} {} {}", "inspect".green().bold(), "<name>".dimmed(), "Inspect container details".white());
        println!("  {} {} {}", "info".green().bold(), "<name>".dimmed(), "Get detailed container information".white());
        println!("  {} {} {}", "size".green().bold(), "<name>".dimmed(), "Get container size information".white());
        println!("  {} {} {}", "attach".green().bold(), "<name>".dimmed(), "Attach to a running container".white());
        println!("  {} {} {}", "commit".green().bold(), "<name> <repo> [tag]".dimmed(), "Create image from container changes".white());
        println!("  {} {} {}", "cp".green().bold(), "<name> <src> <dest>".dimmed(), "Copy files between container and host".white());
        println!("  {} {} {}", "diff".green().bold(), "<name>".dimmed(), "Show container filesystem changes".white());
        println!("  {} {} {}", "export".green().bold(), "<name> <file>".dimmed(), "Export container filesystem".white());
        println!("  {} {} {}", "kill".green().bold(), "<name> [signal]".dimmed(), "Kill a running container".white());
        println!("  {} {} {}", "port".green().bold(), "<name>".dimmed(), "List port mappings".white());
        println!("  {} {} {}", "rename".green().bold(), "<old> <new>".dimmed(), "Rename a container".white());
        println!("  {} {} {}", "top".green().bold(), "<name>".dimmed(), "Show container processes".white());
        println!("  {} {} {}", "update".green().bold(), "<name> [options]".dimmed(), "Update container configuration".white());
        println!("  {} {} {}", "wait".green().bold(), "<name>".dimmed(), "Wait for container to stop".white());
        println!();
        
        // Image Management Section
        println!("{}", "🖼️  IMAGE MANAGEMENT".green().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("  {} {} {}", "list".green().bold(), "".dimmed(), "List all Docker images".white());
        println!("  {} {} {}", "pull".green().bold(), "<name>".dimmed(), "Pull an image from Docker Hub".white());
        println!("  {} {} {}", "build".green().bold(), "<path> <tag>".dimmed(), "Build an image from Dockerfile".white());
        println!("  {} {} {}", "tag".green().bold(), "<source> <target>".dimmed(), "Tag an image".white());
        println!("  {} {} {}", "push".green().bold(), "<name>".dimmed(), "Push an image to registry".white());
        println!("  {} {} {}", "remove".green().bold(), "<name>".dimmed(), "Remove an image (will prompt for confirmation)".white());
        println!("  {} {} {}", "history".green().bold(), "<name>".dimmed(), "Show image history".white());
        println!("  {} {} {}", "import".green().bold(), "<file> <repo> [tag]".dimmed(), "Import image from tarball".white());
        println!("  {} {} {}", "load".green().bold(), "<file>".dimmed(), "Load image from tar archive".white());
        println!("  {} {} {}", "save".green().bold(), "<name> <file>".dimmed(), "Save image to tar archive".white());
        println!();
        
        // Network Management Section
        println!("{}", "🌐 NETWORK MANAGEMENT".green().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("  {} {} {}", "networks".green().bold(), "".dimmed(), "List all Docker networks".white());
        println!();
        
        // Volume Management Section
        println!("{}", "💾 VOLUME MANAGEMENT".green().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("  {} {} {}", "volumes".green().bold(), "".dimmed(), "List all Docker volumes".white());
        println!();
        
        // Monitoring Section
        println!("{}", "📊 MONITORING & SYSTEM".green().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("  {} {} {}", "stats".green().bold(), "".dimmed(), "Show real-time container statistics".white());
        println!("  {} {} {}", "system".green().bold(), "".dimmed(), "Show Docker system information".white());
        println!("  {} {} {}", "events".green().bold(), "".dimmed(), "Monitor Docker events in real-time".white());
        println!("  {} {} {}", "dashboard".green().bold(), "".dimmed(), "Show real-time system dashboard".white());
        println!("  {} {} {}", "charts".green().bold(), "".dimmed(), "Display all system charts".white());
        println!();
        
        // Charts Section
        println!("{}", "📈 CHARTS & VISUALIZATIONS".green().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("  {} {} {}", "cpu".green().bold(), "".dimmed(), "Show CPU usage chart".white());
        println!("  {} {} {}", "memory".green().bold(), "".dimmed(), "Show memory usage chart".white());
        println!("  {} {} {}", "network".green().bold(), "".dimmed(), "Show network traffic chart".white());
        println!("  {} {} {}", "storage".green().bold(), "".dimmed(), "Show storage I/O chart".white());
        println!("  {} {} {}", "status".green().bold(), "".dimmed(), "Show container status chart".white());
        println!("  {} {} {}", "images".green().bold(), "".dimmed(), "Show image size distribution".white());
        println!("  {} {} {}", "pie".green().bold(), "".dimmed(), "Show system resource pie chart (WIP)".white());
        println!();
        
        // Interactive Mode Section
        println!("{}", "🔄 INTERACTIVE MODE".green().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("  {} {} {}", "interactive".green().bold(), "".dimmed(), "Launch interactive mode for continuous operations".white());
        println!();
        
        println!("{}", "📝 Examples:".yellow().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("  {} {}", "dui containers list".cyan(), "→ List all containers".dimmed());
        println!("  {} {}", "dui containers create my-app nginx:latest".cyan(), "→ Create container from image".dimmed());
        println!("  {} {}", "dui containers create my-db postgres:13 -p 5432:5432".cyan(), "→ Create container with port mapping".dimmed());
        println!("  {} {}", "dui containers start my-postgres".cyan(), "→ Start a container named 'my-postgres'".dimmed());
        println!("  {} {}", "dui containers restart my-postgres".cyan(), "→ Restart a container".dimmed());
        println!("  {} {}", "dui containers exec my-postgres ls".cyan(), "→ Execute command in container".dimmed());
        println!("  {} {}", "dui containers info my-postgres".cyan(), "→ Get detailed container information".dimmed());
        println!("  {} {}", "dui containers size my-postgres".cyan(), "→ Get container size".dimmed());
        println!("  {} {}", "dui containers top my-postgres".cyan(), "→ Show container processes".dimmed());
        println!("  {} {}", "dui containers commit my-postgres my-repo:latest".cyan(), "→ Commit container changes".dimmed());
        println!("  {} {}", "dui containers cp my-postgres /data /backup".cyan(), "→ Copy files from container".dimmed());
        println!("  {} {}", "dui containers export my-postgres backup.tar".cyan(), "→ Export container filesystem".dimmed());
        println!("  {} {}", "dui images pull nginx:latest".cyan(), "→ Pull the latest nginx image".dimmed());
        println!("  {} {}", "dui images build . myapp:latest".cyan(), "→ Build image from current directory".dimmed());
        println!("  {} {}", "dui images history nginx:latest".cyan(), "→ Show image history".dimmed());
        println!("  {} {}", "dui images save nginx:latest nginx.tar".cyan(), "→ Save image to file".dimmed());
        println!("  {} {}", "dui images load nginx.tar".cyan(), "→ Load image from file".dimmed());
        println!("  {} {}", "dui networks".cyan(), "→ List all networks".dimmed());
        println!("  {} {}", "dui volumes".cyan(), "→ List all volumes".dimmed());
        println!("  {} {}", "dui monitor dashboard".cyan(), "→ Show real-time dashboard".dimmed());
        println!("  {} {}", "dui charts cpu".cyan(), "→ Show CPU usage chart".dimmed());
        println!("  {} {}", "dui charts pie".cyan(), "→ Show system pie chart (WIP)".dimmed());
        println!("  {} {}", "dui interactive".cyan(), "→ Launch interactive mode".dimmed());
        println!();
        
        println!("{}", "💡 Interactive Mode Features:".yellow().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("{}", "  • Numbered menus for easy selection".dimmed());
        println!("{}", "  • Tab completion for commands and names".dimmed());
        println!("{}", "  • Real-time container and image management".dimmed());
        println!("{}", "  • Advanced operations with confirmation prompts".dimmed());
        println!("{}", "  • Beautiful charts and visualizations".dimmed());
        println!("{}", "  • Real-time system monitoring dashboard".dimmed());
        println!();
        
        println!("{}", "🔧 Tips:".yellow().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!("{}", "  • Container names can be partial matches".dimmed());
        println!("{}", "  • Use 'dui interactive' for continuous operations".dimmed());
        println!("{}", "  • Interactive mode supports numbered menu selection".dimmed());
        println!("{}", "  • The tool will prompt for confirmation before destructive operations".dimmed());
        println!("{}", "  • All commands show colored output for better readability".dimmed());
        println!();
        
        println!("{}", "❓ Need Help?".yellow().bold());
        println!("{}", "  Run 'dui' without arguments to see this help message".dimmed());
        println!("{}", "  Use 'dui interactive' then type 'help' for interactive mode help".dimmed());
        println!();
    }

    pub fn show_interactive_help(&self) {
        println!("{}", "🔄 Interactive Mode Commands:".yellow().bold());
        println!("{}", "─".repeat(50).dimmed());
        println!();
        
        println!("{}", "🐳 Container Commands:".green().bold());
        println!("  {} - List all containers with interactive menu", "containers".cyan());
        println!("  {} - Start a specific container", "start <name>".cyan());
        println!("  {} - Stop a specific container", "stop <name>".cyan());
        println!("  {} - Restart a specific container", "restart <name>".cyan());
        println!("  {} - Pause a specific container", "pause <name>".cyan());
        println!("  {} - Unpause a specific container", "unpause <name>".cyan());
        println!("  {} - Remove a specific container", "remove <name>".cyan());
        println!("  {} - Show container logs", "logs <name>".cyan());
        println!("  {} - Execute command in container", "exec <name> <cmd>".cyan());
        println!("  {} - Inspect container details", "inspect <name>".cyan());
        println!("  {} - Show container processes", "top <name>".cyan());
        println!("  {} - Attach to container", "attach <name>".cyan());
        println!("  {} - Commit container changes", "commit <name> <repo>".cyan());
        println!("  {} - Copy files from container", "cp <name> <src> <dest>".cyan());
        println!("  {} - Show container diff", "diff <name>".cyan());
        println!("  {} - Export container", "export <name> <file>".cyan());
        println!("  {} - Kill container", "kill <name>".cyan());
        println!("  {} - Show port mappings", "port <name>".cyan());
        println!("  {} - Rename container", "rename <old> <new>".cyan());
        println!("  {} - Update container", "update <name>".cyan());
        println!("  {} - Wait for container", "wait <name>".cyan());
        println!();
        
        println!("{}", "🖼️  Image Commands:".green().bold());
        println!("  {} - List all Docker images with interactive menu", "images".cyan());
        println!("  {} - Pull an image", "pull <name>".cyan());
        println!("  {} - Build an image", "build <path> <tag>".cyan());
        println!("  {} - Tag an image", "tag <source> <target>".cyan());
        println!("  {} - Push an image", "push <name>".cyan());
        println!("  {} - Remove an image", "remove <name>".cyan());
        println!("  {} - Show image history", "history <name>".cyan());
        println!("  {} - Import image", "import <file> <repo>".cyan());
        println!("  {} - Load image", "load <file>".cyan());
        println!("  {} - Save image", "save <name> <file>".cyan());
        println!();
        
        println!("{}", "🌐 Network Commands:".green().bold());
        println!("  {} - List all networks", "networks".cyan());
        println!();
        
        println!("{}", "💾 Volume Commands:".green().bold());
        println!("  {} - List all volumes", "volumes".cyan());
        println!();
        
        println!("{}", "📊 Monitoring Commands:".green().bold());
        println!("  {} - Show real-time container statistics", "stats".cyan());
        println!("  {} - Show Docker system information", "system".cyan());
        println!("  {} - Monitor Docker events", "events".cyan());
        println!("  {} - Show real-time system dashboard", "dashboard".cyan());
        println!("  {} - Display all system charts", "charts".cyan());
        println!();
        
        println!("{}", "📈 Chart Commands:".green().bold());
        println!("  {} - Show CPU usage chart", "cpu-chart".cyan());
        println!("  {} - Show memory usage chart", "memory-chart".cyan());
        println!("  {} - Show system pie chart", "pie-chart".cyan());
        println!();
        
        println!("{}", "🔧 Utility Commands:".green().bold());
        println!("  {} - Show this help message", "help".cyan());
        println!("  {} - Exit interactive mode", "exit".cyan());
        println!("  {} - Exit interactive mode", "quit".cyan());
        println!();
        
        println!("{}", "💡 Interactive Features:".yellow().bold());
        println!("{}", "  • Numbered menus for easy selection".dimmed());
        println!("{}", "  • Tab completion for commands and names".dimmed());
        println!("{}", "  • Real-time feedback for all operations".dimmed());
        println!("{}", "  • Confirmation prompts for destructive operations".dimmed());
        println!("{}", "  • Beautiful ASCII charts and visualizations".dimmed());
        println!("{}", "  • Real-time system monitoring dashboard".dimmed());
        println!();
        
        println!("{}", "📝 Example Session:".yellow().bold());
        println!("{}", "  dui> containers".dimmed());
        println!("{}", "  dui> images".dimmed());
        println!("{}", "  dui> networks".dimmed());
        println!("{}", "  dui> dashboard".dimmed());
        println!("{}", "  dui> charts".dimmed());
        println!("{}", "  dui> exit".dimmed());
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

    pub fn display_container_processes(&self, processes: &[ContainerProcess]) {
        if processes.is_empty() {
            self.show_info("No processes found.");
            return;
        }

        println!();
        println!("{}", "📊 Container Processes".cyan().bold());
        println!("{}", "─".repeat(120).dimmed());
        
        // Header
        println!(
            "{:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<20}",
            "USER".bold(),
            "PID".bold(),
            "PPID".bold(),
            "CPU%".bold(),
            "MEM%".bold(),
            "VSZ".bold(),
            "RSS".bold(),
            "TTY".bold(),
            "STAT".bold(),
            "START".bold(),
            "TIME".bold(),
            "COMMAND".bold()
        );
        println!("{}", "─".repeat(120).dimmed());

        for process in processes {
            println!(
                "{:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<20}",
                process.user.white(),
                process.pid.cyan(),
                process.ppid.dimmed(),
                process.cpu.yellow(),
                process.mem.yellow(),
                process.vsz.dimmed(),
                process.rss.dimmed(),
                process.tty.dimmed(),
                process.stat.green(),
                process.start.dimmed(),
                process.time.dimmed(),
                process.command.white()
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

    pub fn display_containers_interactive(&self, containers: &[Container]) {
        if containers.is_empty() {
            self.show_info("No containers found.");
            return;
        }

        println!();
        println!("{}", "📦 Docker Containers (Interactive)".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());
        
        // Header
        println!(
            "{:<4} {:<12} {:<20} {:<25} {:<15} {:<20}",
            "#".bold(),
            "ID".bold(),
            "NAME".bold(),
            "IMAGE".bold(),
            "STATUS".bold(),
            "PORTS".bold()
        );
        println!("{}", "─".repeat(80).dimmed());

        for (i, container) in containers.iter().enumerate() {
            let status_color = if container.status.contains("Up") {
                container.status.green()
            } else {
                container.status.red()
            };

            println!(
                "{:<4} {:<12} {:<20} {:<25} {:<15} {:<20}",
                (i + 1).to_string().yellow().bold(),
                container.id[..12.min(container.id.len())].dimmed(),
                container.name.white(),
                container.image.cyan(),
                status_color,
                container.ports.dimmed()
            );
        }
        
        println!();
        println!("{}", "🔧 Available Actions:".yellow().bold());
        println!("  {} - Start container", "start <number>".cyan());
        println!("  {} - Stop container", "stop <number>".cyan());
        println!("  {} - Restart container", "restart <number>".cyan());
        println!("  {} - Pause container", "pause <number>".cyan());
        println!("  {} - Unpause container", "unpause <number>".cyan());
        println!("  {} - Remove container", "remove <number>".cyan());
        println!("  {} - Show logs", "logs <number>".cyan());
        println!("  {} - Execute command", "exec <number> <cmd>".cyan());
        println!("  {} - Inspect container", "inspect <number>".cyan());
        println!("  {} - Get container info", "info <number>".cyan());
        println!("  {} - Show processes", "top <number>".cyan());
        println!("  {} - Attach to container", "attach <number>".cyan());
        println!("  {} - Commit container", "commit <number> <repo>".cyan());
        println!("  {} - Copy files", "cp <number> <src> <dest>".cyan());
        println!("  {} - Show diff", "diff <number>".cyan());
        println!("  {} - Export container", "export <number> <file>".cyan());
        println!("  {} - Kill container", "kill <number>".cyan());
        println!("  {} - Show ports", "port <number>".cyan());
        println!("  {} - Rename container", "rename <number> <new>".cyan());
        println!("  {} - Update container", "update <number>".cyan());
        println!("  {} - Wait for container", "wait <number>".cyan());
        println!("  {} - Back to main menu", "back".cyan());
        println!();
    }

    pub fn display_images_interactive(&self, images: &[Image]) {
        if images.is_empty() {
            self.show_info("No images found.");
            return;
        }

        println!();
        println!("{}", "🖼️  Docker Images (Interactive)".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());
        
        // Header
        println!(
            "{:<4} {:<12} {:<25} {:<10} {:<12} {:<20}",
            "#".bold(),
            "ID".bold(),
            "REPOSITORY".bold(),
            "TAG".bold(),
            "SIZE".bold(),
            "CREATED".bold()
        );
        println!("{}", "─".repeat(80).dimmed());

        for (i, image) in images.iter().enumerate() {
            println!(
                "{:<4} {:<12} {:<25} {:<10} {:<12} {:<20}",
                (i + 1).to_string().yellow().bold(),
                image.id[..12.min(image.id.len())].dimmed(),
                image.repository.white(),
                image.tag.cyan(),
                image.size.yellow(),
                image.created.dimmed()
            );
        }
        
        println!();
        println!("{}", "🔧 Available Actions:".yellow().bold());
        println!("  {} - Remove image", "remove <number>".cyan());
        println!("  {} - Tag image", "tag <number> <new-tag>".cyan());
        println!("  {} - Push image", "push <number>".cyan());
        println!("  {} - Show history", "history <number>".cyan());
        println!("  {} - Save image", "save <number> <file>".cyan());
        println!("  {} - Back to main menu", "back".cyan());
        println!();
    }

    pub fn display_networks(&self, networks: &[Network]) {
        if networks.is_empty() {
            self.show_info("No networks found.");
            return;
        }

        println!();
        println!("{}", "🌐 Docker Networks".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());
        
        // Header
        println!(
            "{:<12} {:<20} {:<15} {:<10}",
            "ID".bold(),
            "NAME".bold(),
            "DRIVER".bold(),
            "SCOPE".bold()
        );
        println!("{}", "─".repeat(80).dimmed());

        for network in networks {
            println!(
                "{:<12} {:<20} {:<15} {:<10}",
                network.id[..12.min(network.id.len())].dimmed(),
                network.name.white(),
                network.driver.cyan(),
                network.scope.yellow()
            );
        }
        println!();
    }

    pub fn display_volumes(&self, volumes: &[Volume]) {
        if volumes.is_empty() {
            self.show_info("No volumes found.");
            return;
        }

        println!();
        println!("{}", "💾 Docker Volumes".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());
        
        // Header
        println!(
            "{:<20} {:<15} {:<40}",
            "NAME".bold(),
            "DRIVER".bold(),
            "MOUNTPOINT".bold()
        );
        println!("{}", "─".repeat(80).dimmed());

        for volume in volumes {
            println!(
                "{:<20} {:<15} {:<40}",
                volume.name.white(),
                volume.driver.cyan(),
                volume.mountpoint.dimmed()
            );
        }
        println!();
    }
}
