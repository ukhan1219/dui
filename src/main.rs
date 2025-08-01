use clap::{App, Arg, SubCommand};
use rustyline::error::ReadlineError;
use std::io::Write;

mod docker;
mod ui;
mod utils;
mod completion;
mod charts;

use docker::DockerClient;
use ui::UserInterface;
use completion::create_editor;
use charts::ChartRenderer;

fn main() {
    let matches = App::new("DUI")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Usman Khan <usman@usmankhan.dev>")
        .about("An intuitive Docker management CLI with GUI-like features")
        .subcommand(
            SubCommand::with_name("containers")
                .about("Manage Docker containers")
                .arg(
                    Arg::with_name("action")
                        .help("Action to perform")
                        .required(true)
                        .possible_values(&[
                            "list", "start", "stop", "restart", "pause", "unpause", "remove", 
                            "logs", "exec", "inspect", "create", "size", "info", "attach", 
                            "commit", "cp", "diff", "export", "kill", "port", "rename", 
                            "top", "update", "wait"
                        ])
                        .index(1),
                )
                .arg(
                    Arg::with_name("name")
                        .help("Container name or ID")
                        .takes_value(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("command")
                        .help("Command to execute (for exec action)")
                        .takes_value(true)
                        .index(3),
                )
                .arg(
                    Arg::with_name("image")
                    .help("Image name (for create action)")
                    .takes_value(true)
                    .index(4),
                )
                .arg(
                    Arg::with_name("ports")
                    .help("Port mapping (for create action)")
                    .takes_value(true)
                    .index(5),
                )
                .arg(
                    Arg::with_name("volumes")
                    .help("Volume mapping (for create action)")
                    .takes_value(true)
                    .index(6),
                )
                .arg(
                    Arg::with_name("env")
                    .help("Environment variables (for create action)")
                    .takes_value(true)
                    .index(7),
                )
                .arg(
                    Arg::with_name("repository")
                    .help("Repository name (for commit action)")
                    .takes_value(true)
                    .index(8),
                )
                .arg(
                    Arg::with_name("tag")
                    .help("Tag name (for commit action)")
                    .takes_value(true)
                    .index(9),
                )
                .arg(
                    Arg::with_name("src_path")
                    .help("Source path (for cp action)")
                    .takes_value(true)
                    .index(10),
                )
                .arg(
                    Arg::with_name("dest_path")
                    .help("Destination path (for cp action)")
                    .takes_value(true)
                    .index(11),
                )
                .arg(
                    Arg::with_name("output_file")
                    .help("Output file (for export action)")
                    .takes_value(true)
                    .index(12),
                )
                .arg(
                    Arg::with_name("signal")
                    .help("Signal (for kill action)")
                    .takes_value(true)
                    .index(13),
                )
                .arg(
                    Arg::with_name("new_name")
                    .help("New name (for rename action)")
                    .takes_value(true)
                    .index(14),
                )
                .arg(
                    Arg::with_name("cpu_period")
                    .help("CPU period (for update action)")
                    .takes_value(true)
                    .index(15),
                )
                .arg(
                    Arg::with_name("cpu_quota")
                    .help("CPU quota (for update action)")
                    .takes_value(true)
                    .index(16),
                )
                .arg(
                    Arg::with_name("memory")
                    .help("Memory limit (for update action)")
                    .takes_value(true)
                    .index(17),
                )
                .arg(
                    Arg::with_name("memory_swap")
                    .help("Memory swap limit (for update action)")
                    .takes_value(true)
                    .index(18),
                ),
        )
        .subcommand(
            SubCommand::with_name("images")
                .about("Manage Docker images")
                .arg(
                    Arg::with_name("action")
                        .help("Action to perform")
                        .required(true)
                        .possible_values(&["list", "pull", "build", "tag", "push", "remove", "history", "import", "load", "save"])
                        .index(1),
                )
                .arg(
                    Arg::with_name("name")
                        .help("Image name, path, or source")
                        .takes_value(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("target")
                        .help("Target name or tag")
                        .takes_value(true)
                        .index(3),
                )
                .arg(
                    Arg::with_name("file")
                    .help("File path (for import/load/save actions)")
                    .takes_value(true)
                    .index(4),
                )
                .arg(
                    Arg::with_name("repository")
                    .help("Repository name (for import action)")
                    .takes_value(true)
                    .index(5),
                ),
        )
        .subcommand(
            SubCommand::with_name("networks")
                .about("List Docker networks")
        )
        .subcommand(
            SubCommand::with_name("volumes")
                .about("List Docker volumes")
        )
        .subcommand(
            SubCommand::with_name("monitor")
                .about("Monitor Docker resources")
                .arg(
                    Arg::with_name("type")
                        .help("Resource type to monitor")
                        .required(true)
                        .possible_values(&["stats", "system", "events", "dashboard", "charts"])
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("charts")
                .about("Display system charts and visualizations")
                .arg(
                    Arg::with_name("type")
                        .help("Chart type to display")
                        .required(true)
                        .possible_values(&["cpu", "memory", "network", "storage", "status", "images", "pie", "dashboard"])
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
    let charts = ChartRenderer::new();

    // Check if Docker is available
    if !docker_client.is_docker_available() {
        ui.show_error("Docker is not available. Please make sure Docker is installed and running.");
        return;
    }

    match matches.subcommand() {
        ("containers", Some(sub_matches)) => {
            handle_container_command(&docker_client, &ui, sub_matches);
        }
        ("images", Some(sub_matches)) => {
            handle_image_command(&docker_client, &ui, sub_matches);
        }
        ("networks", Some(_)) => {
            handle_networks_command(&docker_client, &ui);
        }
        ("volumes", Some(_)) => {
            handle_volumes_command(&docker_client, &ui);
        }
        ("monitor", Some(sub_matches)) => {
            handle_monitor_command(&docker_client, &ui, &charts, sub_matches);
        }
        ("charts", Some(sub_matches)) => {
            handle_charts_command(&docker_client, &charts, sub_matches);
        }
        ("interactive", Some(_)) => {
            run_interactive_mode(&docker_client, &ui, &charts);
        }
        _ => {
            ui.show_help();
        }
    }
}

fn handle_container_command(docker: &DockerClient, ui: &UserInterface, matches: &clap::ArgMatches) {
    let action = matches.value_of("action").unwrap();
    let name = matches.value_of("name");
    let command = matches.value_of("command");
    let image = matches.value_of("image");
    let ports = matches.value_of("ports");
    let volumes = matches.value_of("volumes");
    let env = matches.value_of("env");
    let repository = matches.value_of("repository");
    let tag = matches.value_of("tag");
    let src_path = matches.value_of("src_path");
    let dest_path = matches.value_of("dest_path");
    let output_file = matches.value_of("output_file");
    let signal = matches.value_of("signal");
    let new_name = matches.value_of("new_name");
    let cpu_period = matches.value_of("cpu_period");
    let cpu_quota = matches.value_of("cpu_quota");
    let memory = matches.value_of("memory");
    let memory_swap = matches.value_of("memory_swap");

    match action {
        "list" => {
            ui.show_loading("Fetching containers...");
            match docker.list_containers() {
                Ok(containers) => ui.display_containers(&containers),
                Err(e) => ui.show_error(&format!("Failed to list containers: {}", e)),
            }
        }
        "create" => {
            if let (Some(container_name), Some(image_name)) = (name, image) {
                ui.show_loading(&format!("Creating container '{}' from image '{}'...", container_name, image_name));
                match docker.create_container(container_name, image_name, ports, volumes, env) {
                    Ok(_) => ui.show_success(&format!("Container '{}' created successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to create container: {}", e)),
                }
            } else {
                ui.show_error("Container name and image are required for create action");
            }
        }
        "attach" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Attaching to container '{}'...", container_name));
                match docker.attach_container(container_name) {
                    Ok(_) => ui.show_success(&format!("Attached to container '{}'", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to attach to container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for attach action");
            }
        }
        "commit" => {
            if let (Some(container_name), Some(repo_name)) = (name, repository) {
                ui.show_loading(&format!("Committing container '{}' to '{}'...", container_name, repo_name));
                match docker.commit_container(container_name, repo_name, tag) {
                    Ok(_) => ui.show_success(&format!("Container '{}' committed successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to commit container: {}", e)),
                }
            } else {
                ui.show_error("Container name and repository are required for commit action");
            }
        }
        "cp" => {
            if let (Some(container_name), Some(src), Some(dest)) = (name, src_path, dest_path) {
                ui.show_loading(&format!("Copying from container '{}'...", container_name));
                match docker.copy_from_container(container_name, src, dest) {
                    Ok(_) => ui.show_success(&format!("Copied from container '{}' successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to copy from container: {}", e)),
                }
            } else {
                ui.show_error("Container name, source path, and destination path are required for cp action");
            }
        }
        "diff" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Getting diff for container '{}'...", container_name));
                match docker.diff_container(container_name) {
                    Ok(diff) => {
                        println!("{}", diff);
                    },
                    Err(e) => ui.show_error(&format!("Failed to get container diff: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for diff action");
            }
        }
        "export" => {
            if let (Some(container_name), Some(output)) = (name, output_file) {
                ui.show_loading(&format!("Exporting container '{}' to '{}'...", container_name, output));
                match docker.export_container(container_name, output) {
                    Ok(_) => ui.show_success(&format!("Container '{}' exported successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to export container: {}", e)),
                }
            } else {
                ui.show_error("Container name and output file are required for export action");
            }
        }
        "kill" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Killing container '{}'...", container_name));
                match docker.kill_container(container_name, signal) {
                    Ok(_) => ui.show_success(&format!("Container '{}' killed successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to kill container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for kill action");
            }
        }
        "port" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Getting port mappings for container '{}'...", container_name));
                match docker.get_container_ports(container_name) {
                    Ok(ports) => {
                        println!("{}", ports);
                    },
                    Err(e) => ui.show_error(&format!("Failed to get container ports: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for port action");
            }
        }
        "rename" => {
            if let (Some(old_name), Some(new_name_val)) = (name, new_name) {
                ui.show_loading(&format!("Renaming container '{}' to '{}'...", old_name, new_name_val));
                match docker.rename_container(old_name, new_name_val) {
                    Ok(_) => ui.show_success(&format!("Container renamed successfully from '{}' to '{}'", old_name, new_name_val)),
                    Err(e) => ui.show_error(&format!("Failed to rename container: {}", e)),
                }
            } else {
                ui.show_error("Container name and new name are required for rename action");
            }
        }
        "top" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Getting processes for container '{}'...", container_name));
                match docker.get_container_processes(container_name) {
                    Ok(processes) => ui.display_container_processes(&processes),
                    Err(e) => ui.show_error(&format!("Failed to get container processes: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for top action");
            }
        }
        "update" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Updating container '{}'...", container_name));
                match docker.update_container(container_name, cpu_period, cpu_quota, memory, memory_swap) {
                    Ok(_) => ui.show_success(&format!("Container '{}' updated successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to update container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for update action");
            }
        }
        "wait" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Waiting for container '{}'...", container_name));
                match docker.wait_for_container(container_name) {
                    Ok(exit_code) => ui.show_success(&format!("Container '{}' exited with code: {}", container_name, exit_code)),
                    Err(e) => ui.show_error(&format!("Failed to wait for container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for wait action");
            }
        }
        "size" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Getting size for container '{}'...", container_name));
                match docker.get_container_size(container_name) {
                    Ok(size) => ui.show_success(&format!("Container '{}' size: {}", container_name, size)),
                    Err(e) => ui.show_error(&format!("Failed to get container size: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for size action");
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
        "restart" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Restarting container '{}'...", container_name));
                match docker.restart_container(container_name) {
                    Ok(_) => ui.show_success(&format!("Container '{}' restarted successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to restart container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for restart action");
            }
        }
        "pause" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Pausing container '{}'...", container_name));
                match docker.pause_container(container_name) {
                    Ok(_) => ui.show_success(&format!("Container '{}' paused successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to pause container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for pause action");
            }
        }
        "unpause" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Unpausing container '{}'...", container_name));
                match docker.unpause_container(container_name) {
                    Ok(_) => ui.show_success(&format!("Container '{}' unpaused successfully", container_name)),
                    Err(e) => ui.show_error(&format!("Failed to unpause container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for unpause action");
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
        "exec" => {
            if let Some(container_name) = name {
                if let Some(cmd) = command {
                    ui.show_loading(&format!("Executing '{}' in container '{}'...", cmd, container_name));
                    match docker.exec_container(container_name, cmd) {
                        Ok(output) => {
                            println!("{}", output);
                        },
                        Err(e) => ui.show_error(&format!("Failed to execute command: {}", e)),
                    }
                } else {
                    ui.show_error("Command is required for exec action");
                }
            } else {
                ui.show_error("Container name is required for exec action");
            }
        }
        "inspect" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Inspecting container '{}'...", container_name));
                match docker.inspect_container(container_name) {
                    Ok(info) => {
                        println!("{}", info);
                    },
                    Err(e) => ui.show_error(&format!("Failed to inspect container: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for inspect action");
            }
        }
        "info" => {
            if let Some(container_name) = name {
                ui.show_loading(&format!("Fetching info for container '{}'...", container_name));
                match docker.get_container_info(container_name) {
                    Ok(info) => {
                        println!("{}", info);
                    },
                    Err(e) => ui.show_error(&format!("Failed to get container info: {}", e)),
                }
            } else {
                ui.show_error("Container name is required for info action");
            }
        }
        _ => ui.show_error("Unknown container action"),
    }
}

fn handle_image_command(docker: &DockerClient, ui: &UserInterface, matches: &clap::ArgMatches) {
    let action = matches.value_of("action").unwrap();
    let name = matches.value_of("name");
    let target = matches.value_of("target");
    let file = matches.value_of("file");
    let repository = matches.value_of("repository");

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
        "build" => {
            if let Some(path) = name {
                if let Some(tag) = target {
                    ui.show_loading(&format!("Building image '{}' from '{}'...", tag, path));
                    match docker.build_image(path, tag) {
                        Ok(_) => ui.show_success(&format!("Image '{}' built successfully", tag)),
                        Err(e) => ui.show_error(&format!("Failed to build image: {}", e)),
                    }
                } else {
                    ui.show_error("Tag is required for build action");
                }
            } else {
                ui.show_error("Path is required for build action");
            }
        }
        "tag" => {
            if let Some(source) = name {
                if let Some(target_tag) = target {
                    ui.show_loading(&format!("Tagging '{}' as '{}'...", source, target_tag));
                    match docker.tag_image(source, target_tag) {
                        Ok(_) => ui.show_success(&format!("Image tagged successfully as '{}'", target_tag)),
                        Err(e) => ui.show_error(&format!("Failed to tag image: {}", e)),
                    }
                } else {
                    ui.show_error("Target tag is required for tag action");
                }
            } else {
                ui.show_error("Source image is required for tag action");
            }
        }
        "push" => {
            if let Some(image_name) = name {
                ui.show_loading(&format!("Pushing image '{}'...", image_name));
                match docker.push_image(image_name) {
                    Ok(_) => ui.show_success(&format!("Image '{}' pushed successfully", image_name)),
                    Err(e) => ui.show_error(&format!("Failed to push image: {}", e)),
                }
            } else {
                ui.show_error("Image name is required for push action");
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
        "history" => {
            if let Some(image_name) = name {
                ui.show_loading(&format!("Getting history for image '{}'...", image_name));
                match docker.get_container_history(image_name) {
                    Ok(history) => {
                        println!("{}", history);
                    },
                    Err(e) => ui.show_error(&format!("Failed to get image history: {}", e)),
                }
            } else {
                ui.show_error("Image name is required for history action");
            }
        }
        "import" => {
            if let (Some(file_path), Some(repo_name)) = (file, repository) {
                ui.show_loading(&format!("Importing '{}' as '{}'...", file_path, repo_name));
                match docker.import_image(file_path, repo_name, target) {
                    Ok(_) => ui.show_success(&format!("Image imported successfully as '{}'", repo_name)),
                    Err(e) => ui.show_error(&format!("Failed to import image: {}", e)),
                }
            } else {
                ui.show_error("File path and repository name are required for import action");
            }
        }
        "load" => {
            if let Some(file_path) = file {
                ui.show_loading(&format!("Loading image from '{}'...", file_path));
                match docker.load_image(file_path) {
                    Ok(_) => ui.show_success(&format!("Image loaded successfully from '{}'", file_path)),
                    Err(e) => ui.show_error(&format!("Failed to load image: {}", e)),
                }
            } else {
                ui.show_error("File path is required for load action");
            }
        }
        "save" => {
            if let (Some(image_name), Some(output_file)) = (name, file) {
                ui.show_loading(&format!("Saving image '{}' to '{}'...", image_name, output_file));
                match docker.save_image(image_name, output_file) {
                    Ok(_) => ui.show_success(&format!("Image '{}' saved successfully to '{}'", image_name, output_file)),
                    Err(e) => ui.show_error(&format!("Failed to save image: {}", e)),
                }
            } else {
                ui.show_error("Image name and output file are required for save action");
            }
        }
        _ => ui.show_error("Unknown image action"),
    }
}

fn handle_networks_command(docker: &DockerClient, ui: &UserInterface) {
    ui.show_loading("Fetching networks...");
    match docker.list_networks() {
        Ok(networks) => ui.display_networks(&networks),
        Err(e) => ui.show_error(&format!("Failed to list networks: {}", e)),
    }
}

fn handle_volumes_command(docker: &DockerClient, ui: &UserInterface) {
    ui.show_loading("Fetching volumes...");
    match docker.list_volumes() {
        Ok(volumes) => ui.display_volumes(&volumes),
        Err(e) => ui.show_error(&format!("Failed to list volumes: {}", e)),
    }
}

fn handle_monitor_command(docker: &DockerClient, ui: &UserInterface, charts: &ChartRenderer, matches: &clap::ArgMatches) {
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
        "dashboard" => {
            ui.show_loading("Fetching real-time dashboard data...");
            match docker.get_container_stats() {
                Ok(stats) => charts.render_real_time_dashboard(&stats),
                Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
            }
        }
        "charts" => {
            ui.show_loading("Fetching data for charts...");
            match docker.get_container_stats() {
                Ok(stats) => {
                    charts.render_cpu_usage_chart(&stats);
                    charts.render_memory_usage_chart(&stats);
                    charts.render_system_pie_chart(&stats);
                },
                Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
            }
        }
        _ => ui.show_error("Unknown monitor type"),
    }
}

fn handle_charts_command(docker: &DockerClient, charts: &ChartRenderer, matches: &clap::ArgMatches) {
    let chart_type = matches.value_of("type").unwrap();

    match chart_type {
        "cpu" => {
            match docker.get_container_stats() {
                Ok(stats) => charts.render_cpu_usage_chart(&stats),
                Err(e) => eprintln!("Failed to get stats: {}", e),
            }
        }
        "memory" => {
            match docker.get_container_stats() {
                Ok(stats) => charts.render_memory_usage_chart(&stats),
                Err(e) => eprintln!("Failed to get stats: {}", e),
            }
        }
        "network" => {
            match docker.get_container_stats() {
                Ok(stats) => charts.render_network_traffic_chart(&stats),
                Err(e) => eprintln!("Failed to get stats: {}", e),
            }
        }
        "storage" => {
            match docker.get_container_stats() {
                Ok(stats) => charts.render_storage_usage_chart(&stats),
                Err(e) => eprintln!("Failed to get stats: {}", e),
            }
        }
        "status" => {
            match docker.list_containers() {
                Ok(containers) => charts.render_container_status_chart(&containers),
                Err(e) => eprintln!("Failed to get containers: {}", e),
            }
        }
        "images" => {
            match docker.list_images() {
                Ok(images) => charts.render_image_size_chart(&images),
                Err(e) => eprintln!("Failed to get images: {}", e),
            }
        }
        "pie" => {
            match docker.get_container_stats() {
                Ok(stats) => charts.render_system_pie_chart(&stats),
                Err(e) => eprintln!("Failed to get stats: {}", e),
            }
        }
        "dashboard" => {
            match docker.get_container_stats() {
                Ok(stats) => charts.render_real_time_dashboard(&stats),
                Err(e) => eprintln!("Failed to get stats: {}", e),
            }
        }
        _ => eprintln!("Unknown chart type"),
    }
}

fn run_interactive_mode(docker: &DockerClient, ui: &UserInterface, charts: &ChartRenderer) {
    ui.show_info("Entering interactive mode. Type 'help' for available commands or 'exit' to quit.");
    ui.show_info("Use TAB for command completion and container/image name suggestions.");
    
    // Create editor with tab completion
    let mut editor = match create_editor(docker.clone()) {
        Ok(editor) => editor,
        Err(e) => {
            ui.show_error(&format!("Failed to initialize tab completion: {}", e));
            return;
        }
    };

    loop {
        let readline = editor.readline("dui> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                
                let parts: Vec<&str> = input.split_whitespace().collect();
                match parts.as_slice() {
                    ["exit"] | ["quit"] => {
                        ui.show_info("Goodbye!");
                        break;
                    }
                    ["help"] => ui.show_interactive_help(),
                    ["containers"] => {
                        match docker.list_containers() {
                            Ok(containers) => {
                                ui.display_containers_interactive(&containers);
                                handle_interactive_container_menu(docker, ui, &containers);
                            },
                            Err(e) => ui.show_error(&format!("Failed to list containers: {}", e)),
                        }
                    }
                    ["images"] => {
                        match docker.list_images() {
                            Ok(images) => {
                                ui.display_images_interactive(&images);
                                handle_interactive_image_menu(docker, ui, &images);
                            },
                            Err(e) => ui.show_error(&format!("Failed to list images: {}", e)),
                        }
                    }
                    ["networks"] => {
                        match docker.list_networks() {
                            Ok(networks) => ui.display_networks(&networks),
                            Err(e) => ui.show_error(&format!("Failed to list networks: {}", e)),
                        }
                    }
                    ["volumes"] => {
                        match docker.list_volumes() {
                            Ok(volumes) => ui.display_volumes(&volumes),
                            Err(e) => ui.show_error(&format!("Failed to list volumes: {}", e)),
                        }
                    }
                    ["stats"] => {
                        match docker.get_container_stats() {
                            Ok(stats) => ui.display_stats(&stats),
                            Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
                        }
                    }
                    ["system"] => {
                        match docker.get_system_info() {
                            Ok(info) => ui.display_system_info(&info),
                            Err(e) => ui.show_error(&format!("Failed to get system info: {}", e)),
                        }
                    }
                    ["events"] => {
                        ui.show_info("Monitoring Docker events (Press Ctrl+C to stop)...");
                        if let Err(e) = docker.monitor_events() {
                            ui.show_error(&format!("Failed to monitor events: {}", e));
                        }
                    }
                    ["dashboard"] => {
                        match docker.get_container_stats() {
                            Ok(stats) => charts.render_real_time_dashboard(&stats),
                            Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
                        }
                    }
                    ["charts"] => {
                        match docker.get_container_stats() {
                            Ok(stats) => {
                                charts.render_cpu_usage_chart(&stats);
                                charts.render_memory_usage_chart(&stats);
                                charts.render_system_pie_chart(&stats);
                            },
                            Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
                        }
                    }
                    ["cpu-chart"] => {
                        match docker.get_container_stats() {
                            Ok(stats) => charts.render_cpu_usage_chart(&stats),
                            Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
                        }
                    }
                    ["memory-chart"] => {
                        match docker.get_container_stats() {
                            Ok(stats) => charts.render_memory_usage_chart(&stats),
                            Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
                        }
                    }
                    ["pie-chart"] => {
                        match docker.get_container_stats() {
                            Ok(stats) => charts.render_system_pie_chart(&stats),
                            Err(e) => ui.show_error(&format!("Failed to get stats: {}", e)),
                        }
                    }
                    _ => ui.show_error("Unknown command. Type 'help' for available commands."),
                }
            }
            Err(ReadlineError::Interrupted) => {
                ui.show_info("Use Ctrl-D or type 'exit' to quit.");
            }
            Err(ReadlineError::Eof) => {
                ui.show_info("Goodbye!");
                break;
            }
            Err(err) => {
                ui.show_error(&format!("Error reading input: {}", err));
                break;
            }
        }
    }
}

fn handle_interactive_container_menu(docker: &DockerClient, ui: &UserInterface, containers: &[docker::Container]) {
    loop {
        let mut input = String::new();
        print!("Enter action (or 'back'): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input == "back" {
            break;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            ["start", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Starting container '{}'...", container.name));
                        match docker.start_container(&container.name) {
                            Ok(_) => ui.show_success(&format!("Container '{}' started successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to start container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["stop", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Stopping container '{}'...", container.name));
                        match docker.stop_container(&container.name) {
                            Ok(_) => ui.show_success(&format!("Container '{}' stopped successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to stop container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["restart", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Restarting container '{}'...", container.name));
                        match docker.restart_container(&container.name) {
                            Ok(_) => ui.show_success(&format!("Container '{}' restarted successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to restart container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["pause", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Pausing container '{}'...", container.name));
                        match docker.pause_container(&container.name) {
                            Ok(_) => ui.show_success(&format!("Container '{}' paused successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to pause container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["unpause", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Unpausing container '{}'...", container.name));
                        match docker.unpause_container(&container.name) {
                            Ok(_) => ui.show_success(&format!("Container '{}' unpaused successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to unpause container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["remove", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        if ui.confirm(&format!("Are you sure you want to remove container '{}'?", container.name)) {
                            ui.show_loading(&format!("Removing container '{}'...", container.name));
                            match docker.remove_container(&container.name) {
                                Ok(_) => ui.show_success(&format!("Container '{}' removed successfully", container.name)),
                                Err(e) => ui.show_error(&format!("Failed to remove container: {}", e)),
                            }
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["logs", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Fetching logs for '{}'...", container.name));
                        match docker.get_container_logs(&container.name) {
                            Ok(logs) => ui.display_logs(&logs),
                            Err(e) => ui.show_error(&format!("Failed to get logs: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["exec", num, cmd] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Executing '{}' in container '{}'...", cmd, container.name));
                        match docker.exec_container(&container.name, cmd) {
                            Ok(output) => println!("{}", output),
                            Err(e) => ui.show_error(&format!("Failed to execute command: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["inspect", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Inspecting container '{}'...", container.name));
                        match docker.inspect_container(&container.name) {
                            Ok(info) => println!("{}", info),
                            Err(e) => ui.show_error(&format!("Failed to inspect container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["info", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Fetching info for container '{}'...", container.name));
                        match docker.get_container_info(&container.name) {
                            Ok(info) => println!("{}", info),
                            Err(e) => ui.show_error(&format!("Failed to get container info: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["top", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Getting processes for container '{}'...", container.name));
                        match docker.get_container_processes(&container.name) {
                            Ok(processes) => ui.display_container_processes(&processes),
                            Err(e) => ui.show_error(&format!("Failed to get container processes: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["attach", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Attaching to container '{}'...", container.name));
                        match docker.attach_container(&container.name) {
                            Ok(_) => ui.show_success(&format!("Attached to container '{}'", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to attach to container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["commit", num, repo] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Committing container '{}' to '{}'...", container.name, repo));
                        match docker.commit_container(&container.name, repo, None) {
                            Ok(_) => ui.show_success(&format!("Container '{}' committed successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to commit container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["cp", num, src, dest] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Copying from container '{}'...", container.name));
                        match docker.copy_from_container(&container.name, src, dest) {
                            Ok(_) => ui.show_success(&format!("Copied from container '{}' successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to copy from container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["diff", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Getting diff for container '{}'...", container.name));
                        match docker.diff_container(&container.name) {
                            Ok(diff) => println!("{}", diff),
                            Err(e) => ui.show_error(&format!("Failed to get container diff: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["export", num, file] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Exporting container '{}' to '{}'...", container.name, file));
                        match docker.export_container(&container.name, file) {
                            Ok(_) => ui.show_success(&format!("Container '{}' exported successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to export container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["kill", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Killing container '{}'...", container.name));
                        match docker.kill_container(&container.name, None) {
                            Ok(_) => ui.show_success(&format!("Container '{}' killed successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to kill container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["port", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Getting port mappings for container '{}'...", container.name));
                        match docker.get_container_ports(&container.name) {
                            Ok(ports) => println!("{}", ports),
                            Err(e) => ui.show_error(&format!("Failed to get container ports: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["rename", num, new_name] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Renaming container '{}' to '{}'...", container.name, new_name));
                        match docker.rename_container(&container.name, new_name) {
                            Ok(_) => ui.show_success(&format!("Container renamed successfully from '{}' to '{}'", container.name, new_name)),
                            Err(e) => ui.show_error(&format!("Failed to rename container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["update", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Updating container '{}'...", container.name));
                        match docker.update_container(&container.name, None, None, None, None) {
                            Ok(_) => ui.show_success(&format!("Container '{}' updated successfully", container.name)),
                            Err(e) => ui.show_error(&format!("Failed to update container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["wait", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= containers.len() {
                        let container = &containers[index - 1];
                        ui.show_loading(&format!("Waiting for container '{}'...", container.name));
                        match docker.wait_for_container(&container.name) {
                            Ok(exit_code) => ui.show_success(&format!("Container '{}' exited with code: {}", container.name, exit_code)),
                            Err(e) => ui.show_error(&format!("Failed to wait for container: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid container number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            _ => {
                ui.show_error("Invalid action. Use 'back' to return to main menu.");
            }
        }
    }
}

fn handle_interactive_image_menu(docker: &DockerClient, ui: &UserInterface, images: &[docker::Image]) {
    loop {
        let mut input = String::new();
        print!("Enter action (or 'back'): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input == "back" {
            break;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            ["remove", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= images.len() {
                        let image = &images[index - 1];
                        let image_name = format!("{}:{}", image.repository, image.tag);
                        if ui.confirm(&format!("Are you sure you want to remove image '{}'?", image_name)) {
                            ui.show_loading(&format!("Removing image '{}'...", image_name));
                            match docker.remove_image(&image_name) {
                                Ok(_) => ui.show_success(&format!("Image '{}' removed successfully", image_name)),
                                Err(e) => ui.show_error(&format!("Failed to remove image: {}", e)),
                            }
                        }
                    } else {
                        ui.show_error("Invalid image number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["tag", num, new_tag] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= images.len() {
                        let image = &images[index - 1];
                        let image_name = format!("{}:{}", image.repository, image.tag);
                        ui.show_loading(&format!("Tagging '{}' as '{}'...", image_name, new_tag));
                        match docker.tag_image(&image_name, new_tag) {
                            Ok(_) => ui.show_success(&format!("Image tagged successfully as '{}'", new_tag)),
                            Err(e) => ui.show_error(&format!("Failed to tag image: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid image number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["push", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= images.len() {
                        let image = &images[index - 1];
                        let image_name = format!("{}:{}", image.repository, image.tag);
                        ui.show_loading(&format!("Pushing image '{}'...", image_name));
                        match docker.push_image(&image_name) {
                            Ok(_) => ui.show_success(&format!("Image '{}' pushed successfully", image_name)),
                            Err(e) => ui.show_error(&format!("Failed to push image: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid image number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["history", num] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= images.len() {
                        let image = &images[index - 1];
                        let image_name = format!("{}:{}", image.repository, image.tag);
                        ui.show_loading(&format!("Getting history for image '{}'...", image_name));
                        match docker.get_container_history(&image_name) {
                            Ok(history) => println!("{}", history),
                            Err(e) => ui.show_error(&format!("Failed to get image history: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid image number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            ["save", num, file] => {
                if let Ok(index) = num.parse::<usize>() {
                    if index > 0 && index <= images.len() {
                        let image = &images[index - 1];
                        let image_name = format!("{}:{}", image.repository, image.tag);
                        ui.show_loading(&format!("Saving image '{}' to '{}'...", image_name, file));
                        match docker.save_image(&image_name, file) {
                            Ok(_) => ui.show_success(&format!("Image '{}' saved successfully to '{}'", image_name, file)),
                            Err(e) => ui.show_error(&format!("Failed to save image: {}", e)),
                        }
                    } else {
                        ui.show_error("Invalid image number");
                    }
                } else {
                    ui.show_error("Invalid number format");
                }
            }
            _ => {
                ui.show_error("Invalid action. Use 'back' to return to main menu.");
            }
        }
    }
}


