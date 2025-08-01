# Docker GUI CLI

An intuitive Docker management CLI tool built with Rust that provides a GUI-like experience in the terminal.

## Features

- 🐳 **Container Management**: List, start, stop, remove containers with ease
- 🖼️ **Image Handling**: Pull, list, and remove Docker images
- 📊 **Resource Monitoring**: View container statistics and system information
- 🎨 **Beautiful Interface**: Colorful, well-formatted output with clear feedback
- 🔄 **Interactive Mode**: Command-line interface for continuous operations
- ✅ **Error Handling**: Comprehensive error messages and input validation
- 🚀 **User-Friendly**: Designed for both beginners and experienced users

## Installation

### Prerequisites

- Docker installed and running on your system
- Rust toolchain (1.70.0 or later)

### Build from Source

``` bash
git clone https://github.com/yourusername/docker-gui-cli.git
cd docker-gui-cli
cargo build --release
```

The binary will be available at `target/release/docker-cli`.

### Install via Cargo

``` bash
cargo install docker-gui-cli
```

## Usage

### Basic Commands

#### Container Management

``` bash
# List all containers
docker-cli containers list

# Start a container
docker-cli containers start my-container

# Stop a container
docker-cli containers stop my-container

# Remove a container
docker-cli containers remove my-container

# View container logs
docker-cli containers logs my-container
```

#### Image Management

``` bash
# List all images
docker-cli images list

# Pull an image
docker-cli images pull nginx:latest

# Remove an image
docker-cli images remove nginx:latest
```

#### Resource Monitoring

``` bash
# View container statistics
docker-cli monitor stats

# View system information
docker-cli monitor system

# Monitor Docker events (real-time)
docker-cli monitor events
```

#### Interactive Mode

``` bash
# Launch interactive mode
docker-cli interactive
```

In interactive mode, you can use simplified commands:
- `containers` - List containers
- `images` - List images
- `stats` - Show statistics
- `help` - Show help
- `exit` - Exit interactive mode

### Examples

``` bash
# Quick container overview
docker-cli containers list

# Pull and run a new container
docker-cli images pull hello-world
docker run hello-world
docker-cli containers list

# Monitor resource usage
docker-cli monitor stats

# Interactive session
docker-cli interactive
> containers
> stats
> exit
```

## Features in Detail

### User-Friendly Interface

- **Colorful Output**: Different colors for different types of information
- **Clear Tables**: Well-formatted tables for easy reading
- **Status Indicators**: Visual indicators for container states
- **Progress Feedback**: Loading indicators for long-running operations

### Error Handling

- **Input Validation**: Validates container and image names
- **Clear Error Messages**: Descriptive error messages with suggestions
- **Graceful Failures**: Handles Docker daemon unavailability

### Safety Features

- **Confirmation Prompts**: Asks for confirmation before destructive operations
- **Non-destructive Defaults**: Safe defaults for all operations
- **Detailed Logging**: Comprehensive logging for troubleshooting

## Development

### Project Structure

```
src/
├── main.rs          # Main application entry point
├── docker.rs        # Docker client implementation
├── ui.rs           # User interface and display logic
└── utils.rs        # Utility functions and validation
```

### Running Tests

``` bash
cargo test
```

### Development Mode

``` bash
cargo run -- containers list
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Roadmap

- [ ] **Container Creation**: GUI for creating new containers
- [ ] **Docker Compose Support**: Manage multi-container applications
- [ ] **Network Management**: View and manage Docker networks
- [ ] **Volume Management**: Handle Docker volumes
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
- Inspired by the need for better Docker tooling

## Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/yourusername/docker-gui-cli/issues) page
2. Create a new issue with detailed information
3. Join our community discussions

---

**Happy Dockering! 🐳**
