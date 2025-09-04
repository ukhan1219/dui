# DUI - Docker User Interface

> An intuitive Docker management CLI built in Rust with **full Docker command parity** and enhanced user experience

[![Crates.io](https://img.shields.io/crates/v/dui-cli)](https://crates.io/crates/dui-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

DUI provides a beautiful, intuitive interface for Docker management with **100% Docker CLI command coverage** and enhanced user experience features like tab completion, interactive menus, and real-time visualizations.

## ✨ Features

### 🚀 Intelligent Docker Management
- **Auto-Start Docker Daemon** - Automatically detects and starts Docker if not running
- **Cross-Platform Support** - Works on macOS, Linux, and Windows
- **Smart Error Handling** - Provides helpful suggestions when Docker issues occur
- **Graceful Startup** - Waits for Docker daemon to be ready before proceeding

### 🐳 Complete Docker Command Parity
- **All 40+ Docker commands** supported with intuitive interfaces
- Container lifecycle management (create, start, stop, restart, pause, unpause, remove)
- Advanced container operations (attach, commit, cp, diff, export, kill, port, rename, top, update, wait)
- Image management (pull, build, tag, push, remove, history, import, load, save)
- Network and volume management
- System monitoring and events

### 🎨 Enhanced User Experience
- **Interactive mode** with numbered menus and tab completion
- **Real-time progress indicators** for long-running operations
- **Color-coded output** for better readability
- **Smart command suggestions** and contextual help
- **Beautiful ASCII charts** and visualizations
- **Comprehensive error handling** with actionable suggestions

### 🔧 Developer-Friendly Features
- **Tab completion** for all commands and container/image names
- **Command history** with search functionality
- **Interactive command builders** with validation
- **Batch operations** and scripting support
- **Real-time monitoring dashboard**

## 🚀 Quick Start

### Installation

#### Using Homebrew (macOS/Linux)
```bash
brew install ukhan1219/dui/dui
```

#### Using Cargo
```bash
cargo install dui-cli
```

#### Manual Installation
Download the latest release from [GitHub Releases](https://github.com/ukhan1219/dui/releases) and add the binary to your PATH.

#### From Source
```bash
git clone https://github.com/ukhan1219/dui.git
cd dui
cargo build --release
cargo install --path .
```

## 📖 Usage

### Basic Commands

#### Container Management

```bash
# List all containers
dui containers list

# Create a new container
dui containers create my-container nginx:latest -p 8080:80 -v /host/path:/container/path -e ENV_VAR=value

# Start a container
dui containers start my-container

# Stop a container
dui containers stop my-container

# Restart a container
dui containers restart my-container

# Pause/Unpause a container
dui containers pause my-container
dui containers unpause my-container

# Remove a container
dui containers remove my-container

# View container logs
dui containers logs my-container

# Execute command in container
dui containers exec my-container /bin/bash

# Inspect container details
dui containers inspect my-container

# Get container information
dui containers info my-container

# Get container size
dui containers size my-container

# Attach to a running container
dui containers attach my-container

# Create image from container changes
dui containers commit my-container my-repo:latest

# Copy files between container and host
dui containers cp my-container /data /backup

# Show container filesystem changes
dui containers diff my-container

# Export container filesystem
dui containers export my-container backup.tar

# Kill a running container
dui containers kill my-container

# List port mappings
dui containers port my-container

# Rename a container
dui containers rename old-name new-name

# Show container processes
dui containers top my-container

# Update container configuration
dui containers update my-container

# Wait for container to stop
dui containers wait my-container
```

#### Image Management

```bash
# List all images
dui images list

# Pull an image
dui images pull nginx:latest

# Build an image
dui images build /path/to/dockerfile my-image:tag

# Tag an image
dui images tag source-image:tag new-image:tag

# Push an image
dui images push my-image:tag

# Remove an image
dui images remove nginx:latest

# Show image history
dui images history nginx:latest

# Import image from tarball
dui images import backup.tar my-repo:latest

# Load image from tar archive
dui images load nginx.tar

# Save image to tar archive
dui images save nginx:latest nginx.tar
```

#### Network & Volume Management

```bash
# List Docker networks
dui networks

# List Docker volumes
dui volumes
```

#### Resource Monitoring

```bash
# View container statistics
dui monitor stats

# View system information
dui monitor system

# Monitor Docker events (real-time)
dui monitor events

# View real-time dashboard
dui monitor dashboard

# Display various charts
dui monitor charts
```

#### Visual Charts & Analytics

```bash
# CPU usage chart
dui charts cpu

# Memory usage chart
dui charts memory

# Network traffic chart
dui charts network

# Storage usage chart
dui charts storage

# Container status chart
dui charts status

# Image size chart
dui charts images

# System pie chart
dui charts pie

# Real-time dashboard
dui charts dashboard
```

#### Interactive Mode

```bash
# Launch interactive mode with tab completion
dui interactive
```

In interactive mode, you can use simplified commands with full tab completion:
- `containers` - List containers with interactive menu
- `images` - List images with interactive menu
- `networks` - List networks
- `volumes` - List volumes
- `stats` - Show statistics
- `system` - Show system information
- `events` - Monitor Docker events
- `dashboard` - Show real-time dashboard
- `charts` - Display all charts
- `cpu-chart` - CPU usage chart
- `memory-chart` - Memory usage chart
- `pie-chart` - System pie chart
- `help` - Show help
- `exit` - Exit interactive mode

### Examples

```bash
# Quick container overview
dui containers list

# Create and run a new web server
dui containers create web-server nginx:latest -p 8080:80
dui containers start web-server

# Advanced container operations
dui containers commit my-app my-repo:latest
dui containers cp my-app /data /backup
dui containers export my-app backup.tar
dui containers top my-app

# Image operations
dui images history nginx:latest
dui images save nginx:latest nginx.tar
dui images load nginx.tar

# Monitor resource usage with charts
dui charts cpu
dui charts memory

# Interactive session with tab completion
dui interactive
> containers
> start 1
> stats
> charts
> exit
```

## 🎯 Features in Detail

### Advanced Container Management

- **Container Creation**: Create containers with port mappings, volume mounts, and environment variables
- **Container Inspection**: Detailed container information and configuration
- **Size Analysis**: Container disk usage and resource consumption
- **Exec Support**: Execute commands inside running containers
- **Comprehensive Lifecycle**: Full container lifecycle management
- **Advanced Operations**: Attach, commit, copy, diff, export, kill, port mapping, rename, process monitoring, update, wait

### Visual Analytics & Charts

- **CPU Usage Charts**: Real-time CPU consumption with color-coded bars
- **Memory Usage Charts**: Memory consumption with percentage and absolute values
- **Network Traffic Charts**: Network I/O visualization
- **Storage Usage Charts**: Disk usage analytics
- **Status Charts**: Container state distribution
- **Image Size Charts**: Docker image size analysis
- **System Pie Charts**: Overall system resource distribution
- **Real-time Dashboard**: Live system overview

### Interactive Mode Features

- **Tab Completion**: Intelligent command and container/image name completion
- **Interactive Menus**: Numbered lists for easy container/image selection
- **Command History**: Persistent command history
- **Syntax Highlighting**: Color-coded command input
- **Smart Suggestions**: Context-aware command suggestions
- **Real-time Feedback**: Immediate feedback for all operations

### Network & Volume Management

- **Network Listing**: View all Docker networks with details
- **Volume Management**: List and inspect Docker volumes
- **Resource Discovery**: Easy discovery of Docker resources

## 🔧 Technical Architecture

### Core Components

- **CLI Layer** (`main.rs`): Command parsing and routing with comprehensive argument handling
- **Docker Integration** (`docker.rs`): Complete Docker API wrapper with 40+ command implementations
- **User Interface** (`ui.rs`): Enhanced UI with color-coded output and interactive menus
- **Tab Completion** (`completion.rs`): Intelligent command completion using rustyline
- **Visual Charts** (`charts.rs`): Real-time chart rendering with ASCII art
- **Utilities** (`utils.rs`): Shared helper functions and validation

### Dependencies

- **clap**: Command-line argument parsing
- **colored**: Terminal color support
- **rustyline**: Interactive line editing with tab completion
- **crossterm**: Cross-platform terminal manipulation
- **tui**: Terminal UI components
- **serde**: Serialization/deserialization
- **tokio**: Async runtime (optional)

### Build Configuration

- **Rust Version**: 1.70.0+
- **Optimization**: LTO enabled for release builds
- **Panic Strategy**: Abort on panic for smaller binaries
- **Code Generation**: Single codegen unit for better optimization

## 🚀 Performance & Reliability

- **Fast Startup**: Optimized binary with minimal dependencies
- **Memory Efficient**: Low memory footprint for long-running sessions
- **Error Handling**: Comprehensive error handling with user-friendly messages
- **Validation**: Input validation for all commands and parameters
- **Cross-Platform**: Works on macOS, Linux, and Windows

## 📚 Documentation

- **Updated README** with all features and examples
- **Comprehensive command reference**
- **Interactive mode guide**
- **Setup and deployment documentation** (see `SETUP.md`)
- **Contributing guidelines**

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** and add tests
4. **Commit your changes**: `git commit -m 'Add amazing feature'`
5. **Push to the branch**: `git push origin feature/amazing-feature`
6. **Open a Pull Request**

### Development Setup

```bash
# Clone the repository
git clone https://github.com/ukhan1219/dui.git
cd dui

# Build the project
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://rust-lang.org/)
- Uses [clap](https://github.com/clap-rs/clap) for command-line argument parsing
- Uses [rustyline](https://github.com/kkawakam/rustyline) for interactive line editing
- Uses [colored](https://github.com/mackwic/colored) for terminal colors
- Uses [tui-rs](https://github.com/fdehau/tui-rs) for terminal UI components

---

**DUI** - Making Docker management beautiful and intuitive! 🐳✨
