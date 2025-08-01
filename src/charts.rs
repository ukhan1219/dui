use colored::*;
use crate::docker::ContainerStats;
use crate::utils::truncate_string;

pub struct ChartRenderer;

impl ChartRenderer {
    pub fn new() -> Self {
        ChartRenderer
    }

    pub fn render_cpu_usage_chart(&self, stats: &[ContainerStats]) {
        if stats.is_empty() {
            println!("{}", "No running containers to display CPU usage".yellow());
            return;
        }

        println!();
        println!("{}", "📊 CPU Usage Chart".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());

        for stat in stats {
            let cpu_percent = stat.cpu_percent.replace('%', "").parse::<f32>().unwrap_or(0.0);
            let bar_length = ((cpu_percent / 100.0) * 50.0) as usize;
            
            let bar = "█".repeat(bar_length);
            let empty = "░".repeat(50 - bar_length);
            
            let color = if cpu_percent > 80.0 {
                bar.red()
            } else if cpu_percent > 50.0 {
                bar.yellow()
            } else {
                bar.green()
            };

            let container_name = truncate_string(&stat.name, 20);

            println!(
                "{:<20} {}{} {}%",
                container_name.white(),
                color,
                empty.dimmed(),
                cpu_percent.to_string().bold()
            );
        }
        println!();
    }

    pub fn render_memory_usage_chart(&self, stats: &[ContainerStats]) {
        if stats.is_empty() {
            println!("{}", "No running containers to display memory usage".yellow());
            return;
        }

        println!();
        println!("{}", "💾 Memory Usage Chart".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());

        for stat in stats {
            let mem_percent = stat.memory_percent.replace('%', "").parse::<f32>().unwrap_or(0.0);
            let bar_length = ((mem_percent / 100.0) * 50.0) as usize;
            
            let bar = "█".repeat(bar_length);
            let empty = "░".repeat(50 - bar_length);
            
            let color = if mem_percent > 80.0 {
                bar.red()
            } else if mem_percent > 50.0 {
                bar.yellow()
            } else {
                bar.green()
            };

            let container_name = truncate_string(&stat.name, 20);

            println!(
                "{:<20} {}{} {}% ({})",
                container_name.white(),
                color,
                empty.dimmed(),
                mem_percent.to_string().bold(),
                stat.memory_usage.cyan()
            );
        }
        println!();
    }

    pub fn render_system_pie_chart(&self, stats: &[ContainerStats]) {
        if stats.is_empty() {
            println!("{}", "No running containers to display system overview".yellow());
            return;
        }

        println!();
        println!("{}", "🍰 System Resource Overview".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());

        let total_cpu: f32 = stats.iter()
            .map(|s| s.cpu_percent.replace('%', "").parse::<f32>().unwrap_or(0.0))
            .sum();

        let total_memory: f32 = stats.iter()
            .map(|s| s.memory_percent.replace('%', "").parse::<f32>().unwrap_or(0.0))
            .sum();

        println!("{}", "CPU Distribution:".yellow().bold());
        for stat in stats {
            let cpu_percent = stat.cpu_percent.replace('%', "").parse::<f32>().unwrap_or(0.0);
            let percentage = if total_cpu > 0.0 { (cpu_percent / total_cpu) * 100.0 } else { 0.0 };
            let slice = self.create_pie_slice(percentage);
            
            println!(
                "  {} {} ({:.1}%)",
                slice,
                stat.name.white(),
                percentage
            );
        }

        println!();
        println!("{}", "Memory Distribution:".yellow().bold());
        for stat in stats {
            let mem_percent = stat.memory_percent.replace('%', "").parse::<f32>().unwrap_or(0.0);
            let percentage = if total_memory > 0.0 { (mem_percent / total_memory) * 100.0 } else { 0.0 };
            let slice = self.create_pie_slice(percentage);
            
            println!(
                "  {} {} ({:.1}%)",
                slice,
                stat.name.white(),
                percentage
            );
        }
        println!();
    }

    fn create_pie_slice(&self, percentage: f32) -> String {
        let symbols = ["◐", "◑", "◒", "◓"];
        let index = ((percentage / 25.0) as usize).min(3);
        symbols[index].to_string()
    }

    pub fn render_network_traffic_chart(&self, stats: &[ContainerStats]) {
        if stats.is_empty() {
            println!("{}", "No running containers to display network traffic".yellow());
            return;
        }

        println!();
        println!("{}", "🌐 Network Traffic Chart".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());

        for stat in stats {
            let net_io = &stat.network_io;
            println!(
                "{:<20} {}",
                stat.name.white(),
                net_io.cyan()
            );
        }
        println!();
    }

    pub fn render_storage_usage_chart(&self, stats: &[ContainerStats]) {
        if stats.is_empty() {
            println!("{}", "No running containers to display storage usage".yellow());
            return;
        }

        println!();
        println!("{}", "💿 Storage I/O Chart".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());

        for stat in stats {
            let block_io = &stat.block_io;
            println!(
                "{:<20} {}",
                stat.name.white(),
                block_io.magenta()
            );
        }
        println!();
    }

    pub fn render_container_status_chart(&self, containers: &[crate::docker::Container]) {
        if containers.is_empty() {
            println!("{}", "No containers to display status chart".yellow());
            return;
        }

        println!();
        println!("{}", "📦 Container Status Overview".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());

        let mut status_counts = std::collections::HashMap::new();
        for container in containers {
            let status = if container.status.contains("Up") {
                "Running".to_string()
            } else if container.status.contains("Exited") {
                "Stopped".to_string()
            } else if container.status.contains("Paused") {
                "Paused".to_string()
            } else {
                "Other".to_string()
            };
            *status_counts.entry(status).or_insert(0) += 1;
        }

        let total = containers.len();
        for (status, count) in status_counts {
            let percentage = (count as f32 / total as f32) * 100.0;
            let bar_length = ((percentage / 100.0) * 30.0) as usize;
            
            let bar = "█".repeat(bar_length);
            let empty = "░".repeat(30 - bar_length);
            
            let color = match status.as_str() {
                "Running" => bar.green(),
                "Stopped" => bar.red(),
                "Paused" => bar.yellow(),
                _ => bar.cyan(),
            };

            println!(
                "{:<10} {}{} {} ({})",
                status.white(),
                color,
                empty.dimmed(),
                count.to_string().bold(),
                format!("{:.1}%", percentage).cyan()
            );
        }
        println!();
    }

    pub fn render_image_size_chart(&self, images: &[crate::docker::Image]) {
        if images.is_empty() {
            println!("{}", "No images to display size chart".yellow());
            return;
        }

        println!();
        println!("{}", "🖼️  Image Size Distribution".cyan().bold());
        println!("{}", "─".repeat(80).dimmed());

        // Sort images by size (parse size string)
        let mut sorted_images: Vec<_> = images.iter().collect();
        sorted_images.sort_by(|a, b| {
            let size_a = self.parse_size(&a.size);
            let size_b = self.parse_size(&b.size);
            size_b.cmp(&size_a) // Descending order
        });

        // Take top 10 largest images
        for image in sorted_images.iter().take(10) {
            let size = &image.size;
            let bar_length = ((self.parse_size(size) as f32 / 1024.0).min(50.0)) as usize; // Normalize to 50 chars
            
            let bar = "█".repeat(bar_length);
            let empty = "░".repeat(50 - bar_length);
            
            println!(
                "{:<25} {}{} {}",
                format!("{}:{}", image.repository, image.tag).white(),
                bar.cyan(),
                empty.dimmed(),
                size.yellow()
            );
        }
        println!();
    }

    fn parse_size(&self, size_str: &str) -> u64 {
        // Parse size strings like "1.2GB", "500MB", etc.
        let size_str = size_str.to_lowercase();
        let size = if size_str.contains("gb") {
            size_str.replace("gb", "").parse::<f64>().unwrap_or(0.0) * 1024.0 * 1024.0 * 1024.0
        } else if size_str.contains("mb") {
            size_str.replace("mb", "").parse::<f64>().unwrap_or(0.0) * 1024.0 * 1024.0
        } else if size_str.contains("kb") {
            size_str.replace("kb", "").parse::<f64>().unwrap_or(0.0) * 1024.0
        } else {
            size_str.parse::<f64>().unwrap_or(0.0)
        };
        size as u64
    }

    pub fn render_real_time_dashboard(&self, stats: &[ContainerStats]) {
        println!();
        println!("{}", "📊 Real-Time System Dashboard".cyan().bold());
        println!("{}", "═".repeat(100).dimmed());
        
        // Header
        println!(
            "{:<20} {:<10} {:<15} {:<15} {:<20} {:<15}",
            "CONTAINER".bold(),
            "CPU %".bold(),
            "MEMORY %".bold(),
            "MEMORY USAGE".bold(),
            "NETWORK I/O".bold(),
            "BLOCK I/O".bold()
        );
        println!("{}", "─".repeat(100).dimmed());

        for stat in stats {
            let cpu_percent = stat.cpu_percent.replace('%', "").parse::<f32>().unwrap_or(0.0);
            let mem_percent = stat.memory_percent.replace('%', "").parse::<f32>().unwrap_or(0.0);
            
            let cpu_color = if cpu_percent > 80.0 {
                cpu_percent.to_string().red()
            } else if cpu_percent > 50.0 {
                cpu_percent.to_string().yellow()
            } else {
                cpu_percent.to_string().green()
            };

            let mem_color = if mem_percent > 80.0 {
                mem_percent.to_string().red()
            } else if mem_percent > 50.0 {
                mem_percent.to_string().yellow()
            } else {
                mem_percent.to_string().green()
            };

            println!(
                "{:<20} {:<10} {:<15} {:<15} {:<20} {:<15}",
                stat.name.white(),
                cpu_color,
                mem_color,
                stat.memory_usage.cyan(),
                stat.network_io.dimmed(),
                stat.block_io.magenta()
            );
        }
        println!("{}", "═".repeat(100).dimmed());
        println!();
    }
} 