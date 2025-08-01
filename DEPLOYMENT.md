# DUI CLI Deployment Guide

This guide explains the complete deployment process and why we have multiple scripts for different purposes.

## 🎯 Overview

We have multiple scripts because we're building a professional-grade deployment pipeline that handles:
- **Cross-platform builds** (Linux, macOS, Windows, ARM64)
- **Package manager distribution** (Homebrew, Cargo)
- **Installation automation** (one-liner installs)
- **Release management** (GitHub Actions)
- **Quality assurance** (testing, validation)

## 📁 Script Organization

### Core Deployment Scripts
```
scripts/
├── release.sh          # Creates git tags and triggers releases
├── trigger-release.sh  # Manual workflow trigger with options
├── monitor-release.sh  # Monitors build progress automatically
├── get-sha256.sh      # Gets SHA256 hashes for Homebrew
├── check-actions.sh    # Checks GitHub Actions status
└── debug-workflow.sh   # Debugs failed workflows
```

### Installation Scripts
```
scripts/
├── install.sh          # Unix/Linux/macOS installation
└── install.ps1         # Windows PowerShell installation
```

### Configuration Files
```
.github/workflows/release.yml  # GitHub Actions workflow
Formula/dui.rb                 # Homebrew formula
```

## 🚀 Step-by-Step Deployment Process

### Step 1: Verify Current State
```bash
# Check current version
cat Cargo.toml | grep version

# Check git status
git status

# Check if we're on main branch
git branch
```

**Why**: Ensures we're starting from a clean state with the correct version.

### Step 2: Make Scripts Executable
```bash
chmod +x scripts/*.sh
```

**Why**: Scripts need execute permissions to run. This is a one-time setup.

### Step 3: Test Local Build
```bash
cargo build --release
```

**Why**: Ensures the code compiles before triggering expensive GitHub Actions builds.

### Step 4: Commit All Changes
```bash
git add .
git commit -m "feat: complete Docker management CLI with deployment automation

- Add comprehensive container management features
- Add visual charts and analytics
- Add interactive mode with tab completion
- Add network and volume management
- Add deployment automation and installation scripts
- Add GitHub Actions workflow for cross-platform builds
- Add Homebrew formula for package manager distribution
- Add installation scripts for easy deployment"
```

**Why**: All changes must be committed before creating a release tag.

### Step 5: Create Release Tag
```bash
./scripts/release.sh
```

**What it does**:
- Reads version from `Cargo.toml`
- Checks if tag already exists
- Creates git tag with release message
- Pushes tag to GitHub
- Triggers GitHub Actions workflow

**Why**: Tags trigger the automated build and release process.

### Step 6: Monitor Build Progress
```bash
./scripts/monitor-release.sh
```

**What it does**:
- Monitors GitHub Actions workflow
- Shows build progress in real-time
- Automatically gets SHA256 hashes when complete
- Provides next steps

**Why**: Builds take 5-10 minutes, so we need to monitor progress.

### Step 7: Get SHA256 Hashes
```bash
./scripts/get-sha256.sh
```

**What it does**:
- Downloads release binaries
- Calculates SHA256 hashes
- Provides Homebrew formula updates

**Why**: Homebrew requires SHA256 hashes for security verification.

### Step 8: Update Homebrew Formula
```bash
# Edit Formula/dui.rb with the new SHA256 hashes
# Then test locally
brew install --build-from-source Formula/dui.rb
```

**Why**: Homebrew needs exact SHA256 hashes to verify binary integrity.

## 🔧 Why So Many Scripts?

### 1. **Separation of Concerns**
Each script has a specific purpose:
- `release.sh`: Tag creation and triggering
- `monitor-release.sh`: Progress monitoring
- `get-sha256.sh`: Hash calculation
- `debug-workflow.sh`: Troubleshooting

### 2. **Cross-Platform Support**
- `install.sh`: Unix/Linux/macOS
- `install.ps1`: Windows PowerShell
- Different platforms need different installation methods

### 3. **Error Handling**
- `check-actions.sh`: Diagnoses workflow issues
- `debug-workflow.sh`: Provides detailed error analysis
- `trigger-release.sh`: Manual recovery options

### 4. **Automation vs Manual**
- Automated scripts for routine tasks
- Manual scripts for debugging and recovery
- Different user skill levels need different tools

### 5. **Professional Deployment**
- GitHub Actions for CI/CD
- Homebrew for package management
- Installation scripts for user convenience
- Monitoring for reliability

## 📋 Complete Deployment Command Sequence

```bash
# 1. Verify state
git status
cat Cargo.toml | grep version

# 2. Make scripts executable
chmod +x scripts/*.sh

# 3. Test build
cargo build --release

# 4. Commit changes
git add .
git commit -m "feat: complete Docker management CLI with deployment automation"

# 5. Create release
./scripts/release.sh

# 6. Monitor progress
./scripts/monitor-release.sh

# 7. Get SHA256 hashes (after completion)
./scripts/get-sha256.sh

# 8. Update Homebrew formula with hashes
# Edit Formula/dui.rb with new hashes

# 9. Test Homebrew installation
brew install --build-from-source Formula/dui.rb
```

## 🎯 Expected Results

After running these commands, you'll have:
- ✅ GitHub release with binaries for all platforms
- ✅ SHA256 hashes for Homebrew formula
- ✅ Installation scripts for users
- ✅ Professional deployment pipeline

## 🔍 Troubleshooting

If anything fails:
```bash
# Check workflow status
./scripts/check-actions.sh

# Debug failures
./scripts/debug-workflow.sh

# Manual trigger
./scripts/trigger-release.sh
```

## 📚 Next Steps After Deployment

1. **Submit to Homebrew**: `brew tap ukhan1219/dui`
2. **Update documentation**: Add installation instructions
3. **Share with community**: Announce the release
4. **Monitor usage**: Track downloads and feedback

This comprehensive setup ensures your CLI can be installed by users through multiple methods and handles edge cases professionally. 