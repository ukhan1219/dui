# DUI - Docker User Interface

An intuitive Docker management CLI tool built with Rust that provides a GUI-like experience in the terminal with advanced monitoring, visualization, and interactive features.

## Features

- 🐳 **Container Management**: Complete container lifecycle management (create, start, stop, restart, pause, unpause, remove, logs, exec, inspect)
- 🖼️ **Image Handling**: Pull, build, tag, push, and remove Docker images
- 📊 **Advanced Monitoring**: Real-time container statistics, system information, and Docker events
- 📈 **Visual Charts**: CPU, memory, network, and storage usage charts with beautiful ASCII visualizations
- 🎨 **Interactive Mode**: Full-featured interactive CLI with tab completion and command suggestions
- 🌐 **Network Management**: List and inspect Docker networks
- 💾 **Volume Management**: List and manage Docker volumes
- 🔍 **Container Inspection**: Detailed container information and size analysis
- ✅ **Smart Validation**: Comprehensive input validation and error handling
- 🚀 **User-Friendly**: Designed for both beginners and experienced users

## Installation

### Prerequisites

- Docker installed and running on your system
- Rust toolchain (1.70.0 or later)

### Build from Source

```bash
git clone https://github.com/ukhan1219/dui.git
cd dui
cargo build --release
```

The binary will be available at `target/release/dui`.

### Install via Cargo

```bash
cargo install dui-cli
```

### Install via Homebrew (Coming Soon)

```bash
brew install dui
```

### Install via Script

#### Unix/Linux/macOS
```bash
curl -fsSL https://raw.githubusercontent.com/ukhan1219/dui/main/scripts/install.sh | bash
```

#### Windows (PowerShell)
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/ukhan1219/dui/main/scripts/install.ps1").Content
```

### Manual Installation

Download the latest release from [GitHub Releases](https://github.com/ukhan1219/dui/releases) and add the binary to your PATH.

## Usage

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

## Features in Detail

### Advanced Container Management

- **Container Creation**: Create containers with port mappings, volume mounts, and environment variables
- **Container Inspection**: Detailed container information and configuration
- **Size Analysis**: Container disk usage and resource consumption
- **Exec Support**: Execute commands inside running containers
- **Comprehensive Lifecycle**: Full container lifecycle management

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
- **Error Recovery**: Graceful error handling with helpful messages

### Network & Volume Management

- **Network Listing**: View all Docker networks with details
- **Volume Management**: List and inspect Docker volumes
- **Resource Discovery**: Easy discovery of Docker resources

### User-Friendly Interface

- **Colorful Output**: Different colors for different types of information
- **Clear Tables**: Well-formatted tables for easy reading
- **Status Indicators**: Visual indicators for container states
- **Progress Feedback**: Loading indicators for long-running operations
- **ASCII Charts**: Beautiful terminal-based visualizations

### Error Handling & Safety

- **Input Validation**: Validates container and image names
- **Clear Error Messages**: Descriptive error messages with suggestions
- **Graceful Failures**: Handles Docker daemon unavailability
- **Confirmation Prompts**: Asks for confirmation before destructive operations
- **Non-destructive Defaults**: Safe defaults for all operations

## Development

### Project Structure

```
src/
├── main.rs          # Main application entry point and CLI handling
├── docker.rs        # Docker client implementation and API
├── ui.rs           # User interface and display logic
├── charts.rs       # Chart rendering and visualization
├── completion.rs   # Tab completion and interactive features
└── utils.rs        # Utility functions and validation
```

### Running Tests

```bash
cargo test
```

### Development Mode

```bash
cargo run -- containers list
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Roadmap

- [x] **Container Creation**: GUI for creating new containers
- [x] **Network Management**: View and manage Docker networks
- [x] **Volume Management**: Handle Docker volumes
- [x] **Visual Charts**: CPU, memory, and system charts
- [x] **Interactive Mode**: Full-featured interactive CLI
- [x] **Tab Completion**: Intelligent command completion
- [x] **Container Inspection**: Detailed container information
- [ ] **Docker Compose Support**: Manage multi-container applications
- [ ] **Registry Integration**: Push/pull from custom registries
- [ ] **Configuration Files**: Save and load CLI preferences
- [ ] **Plugins System**: Extensible plugin architecture
- [ ] **Web Interface**: Optional web-based GUI

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI parsing with [clap](https://github.com/clap-rs/clap)
- Colorful output with [colored](https://github.com/mackwic/colored)
- Interactive CLI with [rustyline](https://github.com/kkawakam/rustyline)
- Terminal UI with [crossterm](https://github.com/crossterm-rs/crossterm)
- Inspired by the need for better Docker tooling

## Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/ukhan1219/dui/issues) page
2. Create a new issue with detailed information
3. Join our community discussions

---

**Happy Dockering! 🐳**
