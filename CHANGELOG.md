# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.1.2] - 2025-09-04

### Fixed
- Clean up temporary files in release script to prevent cargo publish failures
- Improved release automation workflow

## [4.1.1] - 2025-09-04

### Fixed  
- Resolved cargo publish failures caused by untracked temporary files
- Fixed version consistency across release process

## [4.1.0] - 2025-09-04

### Added
- Docker daemon auto-start functionality - automatically starts Docker if not running
- Comprehensive release management system with Cargo publishing  
- Automated changelog generation and release notes
- Enhanced version management and iteration system

### Changed
- Improved Docker availability checking with daemon status verification
- Enhanced error messages with actionable suggestions
- Better user feedback during Docker initialization

### Fixed
- Docker service detection across different platforms (macOS, Linux, Windows)

## [4.1.3] - 2025-09-04

## [4.1.4] - 2025-09-04

## [4.1.5] - 2025-09-04

## [4.1.6] - 2025-09-04

## [Unreleased]

### Added

### Changed

### Fixed


### Added

### Changed

### Fixed


### Added

### Changed

### Fixed


### Added

### Changed

### Fixed


### Added

### Changed

### Fixed


### Added

### Changed

### Fixed


### Added

### Changed

### Fixed


### Added
- Docker daemon auto-start functionality - automatically starts Docker if not running
- Comprehensive release management system with Cargo publishing
- Automated changelog generation and release notes
- Enhanced version management and iteration system

### Changed
- Improved Docker availability checking with daemon status verification
- Enhanced error messages with actionable suggestions
- Better user feedback during Docker initialization

### Fixed
- Docker service detection across different platforms (macOS, Linux, Windows)

## [4.0.10] - 2024-01-15

### Added
- Complete Docker command parity with 40+ commands
- Interactive mode with tab completion and numbered menus
- Real-time progress indicators for long-running operations
- Color-coded output for better readability
- Smart command suggestions and contextual help
- Beautiful ASCII charts and visualizations
- Comprehensive error handling with actionable suggestions
- Tab completion for all commands and container/image names
- Command history with search functionality
- Interactive command builders with validation
- Batch operations and scripting support
- Real-time monitoring dashboard

### Container Management Features
- Container lifecycle management (create, start, stop, restart, pause, unpause, remove)
- Advanced container operations (attach, commit, cp, diff, export, kill, port, rename, top, update, wait)
- Container inspection and information gathering
- Container size analysis and resource consumption monitoring
- Command execution inside running containers

### Image Management Features
- Image management (pull, build, tag, push, remove, history, import, load, save)
- Image size analysis and optimization
- Multi-platform image support
- Image layer inspection and history

### Network & Volume Management
- Network listing and inspection
- Volume management and analysis
- Resource discovery and monitoring

### Visual Analytics & Charts
- CPU usage charts with real-time data
- Memory usage charts with percentage and absolute values
- Network traffic visualization
- Storage usage analytics
- Container status distribution charts
- Image size analysis charts
- System pie charts for resource distribution
- Real-time dashboard with live system overview

### System Monitoring
- System information and monitoring
- Docker events monitoring (real-time)
- Container statistics with resource usage
- Performance monitoring and alerting

### Developer Experience
- Comprehensive command-line interface with full Docker parity
- Interactive mode with intelligent suggestions
- Tab completion for commands, containers, and images
- Colored output and progress indicators
- Error handling with helpful suggestions
- Cross-platform support (macOS, Linux, Windows)
- Fast startup and memory-efficient operation
- Extensive documentation and examples

## [Previous Versions]

### [4.0.9] and Earlier
- Initial Docker management functionality
- Basic container and image operations
- Command-line interface foundation
- Core Docker API integration

---

## Release Notes Format

Each release includes:
- **Added**: New features and enhancements
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes and improvements
- **Security**: Security-related changes

## Contributing

When contributing, please:
1. Update the `[Unreleased]` section with your changes
2. Follow the established format and categories
3. Use clear, descriptive language
4. Link to relevant issues/PRs when applicable

## Release Process

1. Update version in `Cargo.toml`
2. Move unreleased changes to new version section
3. Update release date
4. Create git tag with version
5. Publish to crates.io
6. Update Homebrew formula
7. Create GitHub release with notes
