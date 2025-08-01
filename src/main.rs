use clap::{App, Arg, SubCommand};
use colored::*;
use std::process::Command;
use std::io::{self, Write};
use serde_json::Value;

mod docker;
mod ui;
mod utils;

use docker::DockerClient;
use ui::UserInterface;

fn main() {
    let matches = App::new("Docker GUI CLI")
        .version("1.0.0")
        .author("Your Name")
        .about("An intuitive Docker management CLI with GUI-like features")
        .subcommand(
            SubCommand::with_name("containers")
                .about("Manage Docker containers")
                .arg(
                    Arg::with_name("action")
                        .help("Action to perform")
                        .required(true)
                        .possible_values(&["list", "start", "stop", "remove", "logs"])
                        .index(1),
                )
                .arg(
                    Arg::with_name("name")
                        .help("Container name or ID")
                        .takes_value(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("images")
                .about("Manage Docker images")
                .arg(
                    Arg::with_name("action")
                        .help("Action to perform")
                        .required(true)
                        .possible_values(&["list", "pull", "remove", "build"])
                        .index(1),
                )
                .arg(
                    Arg::with_name("name")
                        .help("Image name or ID")
                        .takes_value(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("monitor")
                .about("Monitor Docker resources")
                .arg(
                    Arg::with_name("type")
                        .help("Resource type to monitor")
                        .required(true)
                        .possible_values(&["stats", "system", "events"])
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("interactive")
                .about("Launch interactive mode")
        )
        .get_matches();

    let docker_client = DockerClient::new();
    let ui = UserInterface::new();

    // Check if Docker is available
    if !docker_client.is_docker_available() {
        ui.show_error("Docker is not available. Please make sure Docker is installed and running.");
        return;
    }

    ui.show_welcome();

    match matches.subcommand() {
        ("containers", Some(sub_matches)) => {
            handle_container_command(&docker_client, &ui, sub_matches);
        }
        ("images", Some(sub_matches)) => {
            handle_image_command(&docker_client, &ui, sub_matches);
        }
        ("monitor", Some(sub_matches)) => {
            handle_monitor_command(&docker_client, &ui, sub_matches);
        }
        ("interactive", Some(_)) => {
            run_interactive_mode(&docker_client, &ui);
        }
        _ => {
            ui.show_help();
        }
    }
}

fn handle_container_command(docker: &DockerClient, ui: &UserInterface, matches: &clap::ArgMatches) {
    let action = matches.value_of("action").unwrap();
    let name = matches.value_of("name");

    match action {
        "list" => {
            ui.show_loading("Fetching containers...");
            match docker.list_containers() {
                Ok(containers) => ui.display_containers(&containers),
                Err(e) => ui.show_error(&format!("Failed to list containers: {}", e)),
            }
        }
        "start" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Starting container '{}'...", container_name));
                match docker.start_container(container_name) {
                    Ok(_) => ui.show_success(&format!("Container '{}' started successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to start container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for start action");
            }
        }
        "stop" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Stopping container '{}'...", container_name));
                match docker.stop_container(container_name) {
                    Ok(_) => ui.show_success(&format!("Container '{}' stopped successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to stop container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for stop action");
            }
        }
        "remove" => {
            if let Some(container_name) = name {
                if ui.confirm(&format!("Are you sure you want to remove container '{}'?", container_name)) {
                    ui.show_loading(&format!("Removing container '{}'...", container_name));
                    match docker.remove_container(container_name) {
                        Ok(_) => ui.show_success(&format!("Container '{}' removed successfully", container_name)),
                        Err(e) => ui.show_error(&format!("Failed to remove container: {}", e)),
                    }
                }
            } else {
                ui.show_error("Container name is required for remove action");
            }
        }
        "logs" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Fetching logs for '{}'...", container_name));
                match docker.get_container_logs(container_name) {
                    Ok(logs) => ui.display_logs(&logs),
                    Err(e) => ui.show_error(&format!("Failed to get logs: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for logs action");
            }
        }
        _ => ui.show_error("Unknown container action"),
    }
}

fn handle_image_command(docker: &DockerClient, ui: &UserInterface, matches: &clap::ArgMatches) {
    let action = matches.value_of("action").unwrap();
    let name = matches.value_of("name");

    match action {
        "list" => {
            ui.show_loading("Fetching images...");
            match docker.list_images() {
                Ok(images) => ui.display_images(&images),
                Err(e) => ui.show_error(&format!("Failed to list images: {}", e)),
            }
        }
        "pull" => {
            if let Some(image_name) = name {
                ui.show_loading(&format!("Pulling image '{}'...", image_name));
                match docker.pull_image(image_name) {
                    Ok(_) => ui.show_success(&format!("Image '{}' pulled successfully", image_name)),
                    Err(e) => ui.show_error(&format!("Failed to pull image: {}", e)),
                }
            } else {
                ui.show_error("Image name is required for pull action");
            }
        }
        "remove" => {
            if let Some(image_name) = name {
                if ui.confirm(&format!("Are you sure you want to remove image '{}'?", image_name)) {
                    ui.show_loading(&format!("Removing image '{}'...", image_name));
                    match docker.remove_image(image_name) {
                        Ok(_) => ui.show_success(&format!("Image '{}' removed successfully", image_name)),
                        Err(e) => ui.show_error(&format!("Failed to remove image: {}", e)),
                    }
                }
            } else {
                ui.show_error("Image name is required for remove action");
            }
        }
        _ => ui.show_error("Unknown image action"),
    }
}

fn handle_monitor_command(docker: &DockerClient, ui: &UserInterface, matches: &clap::ArgMatches) {
    let monitor_type = matches.value_of("type").unwrap();

    match monitor_type {
        "stats" => {
            ui.show_loading("Fetching container statistics...");
            match docker.get_container_stats() {
                Ok(stats) => ui.display_stats(&stats),
                Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
            }
        }
        "system" => {
            ui.show_loading("Fetching system information...");
            match docker.get_system_info() {
                Ok(info) => ui.display_system_info(&info),
                Err(e) => ui.show_error(&format!("Failed to get system info: {}", e)),
            }
        }
        "events" => {
            ui.show_info("Monitoring Docker events (Press Ctrl+C to stop)...");
            if let Err(e) = docker.monitor_events() {
                ui.show_error(&format!("Failed to monitor events: {}", e));
            }
        }
        _ => ui.show_error("Unknown monitor type"),
    }
}

fn run_interactive_mode(docker: &DockerClient, ui: &UserInterface) {
    ui.show_info("Entering interactive mode. Type 'help' for available commands or 'exit' to quit.");
    
    loop {
        print!("{} ", "docker-cli>".cyan().bold());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                
                match input {
                    "exit" | "quit" => {
                        ui.show_info("Goodbye!");
                        break;
                    }
                    "help" => ui.show_interactive_help(),
                    "containers" => {
                        match docker.list_containers() {
                            Ok(containers) => ui.display_containers(&containers),
                            Err(e) => ui.show_error(&format!("Failed to list containers: {}", e)),
                        }
                    }
                    "images" => {
                        match docker.list_images() {
                            Ok(images) => ui.display_images(&images),
                            Err(e) => ui.show_error(&format!("Failed to list images: {}", e)),
                        }
                    }
                    "stats" => {
                        match docker.get_container_stats() {
                            Ok(stats) => ui.display_stats(&stats),
                            Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
                        }
                    }
                    _ => ui.show_error("Unknown command. Type 'help' for available commands."),
                }
            }
            Err(e) => {
                ui.show_error(&format!("Error reading input: {}", e));
                break;
            }
        }
    }
}
