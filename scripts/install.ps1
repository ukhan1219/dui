# DUI CLI Installer for Windows
# An intuitive Docker management CLI built in Rust

param(
    [string]$Version = "latest"
)

# Configuration
$Repo = "ukhan1219/dui"
$BinaryName = "dui.exe"

# Colors for output
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

# Get latest version if not specified
if ($Version -eq "latest") {
    $Version = (Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest").tag_name
}

Write-ColorOutput Green "🐳 Installing DUI CLI..."
Write-ColorOutput Yellow "Version: $Version"
Write-ColorOutput Yellow "Platform: Windows"
Write-Host ""

# Check if Docker is available
try {
    $null = Get-Command docker -ErrorAction Stop
    Write-ColorOutput Green "✅ Docker is available"
} catch {
    Write-ColorOutput Red "❌ Docker is not installed or not in PATH"
    Write-ColorOutput Yellow "Please install Docker Desktop first: https://docs.docker.com/desktop/install/windows/"
    exit 1
}

# Check if Docker is running
try {
    $null = docker info 2>$null
    Write-ColorOutput Green "✅ Docker is running"
} catch {
    Write-ColorOutput Red "❌ Docker is not running"
    Write-ColorOutput Yellow "Please start Docker Desktop and try again"
    exit 1
}

# Create installation directory
$InstallDir = "$env:USERPROFILE\.local\bin"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Download URL
$DownloadUrl = "https://github.com/$Repo/releases/download/$Version/dui-windows-x86_64.exe"

# Download binary
Write-ColorOutput Blue "📥 Downloading DUI CLI..."
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile "$InstallDir\$BinaryName"
    Write-ColorOutput Green "✅ Download completed"
} catch {
    Write-ColorOutput Red "❌ Download failed"
    Write-ColorOutput Red $_.Exception.Message
    exit 1
}

# Add to PATH if not already there
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    Write-ColorOutput Blue "📝 Adding to PATH..."
    $NewPath = "$CurrentPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-ColorOutput Green "✅ Added to PATH"
    Write-ColorOutput Yellow "Please restart your terminal or run: refreshenv"
}

# Test installation
Write-ColorOutput Blue "🧪 Testing installation..."
try {
    & "$InstallDir\$BinaryName" --version | Out-Null
    Write-ColorOutput Green "✅ Installation successful!"
    Write-Host ""
    Write-ColorOutput Blue "🎉 DUI CLI is now installed!"
    Write-ColorOutput Yellow "Usage:"
    Write-Host "  dui --help"
    Write-Host "  dui containers list"
    Write-Host "  dui interactive"
    Write-Host ""
    Write-ColorOutput Blue "📚 Documentation: https://github.com/$Repo"
} catch {
    Write-ColorOutput Red "❌ Installation test failed"
    exit 1
} 