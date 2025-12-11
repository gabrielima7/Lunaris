# Lunaris Engine Installer for Windows
#
# Usage: iwr -useb https://lunaris.dev/install.ps1 | iex
#

$ErrorActionPreference = "Stop"

# Configuration
$Version = if ($env:LUNARIS_VERSION) { $env:LUNARIS_VERSION } else { "1.0.0" }
$InstallDir = if ($env:LUNARIS_INSTALL_DIR) { $env:LUNARIS_INSTALL_DIR } else { "$env:USERPROFILE\.lunaris" }
$Repo = "gabrielima7/Lunaris"

# Banner
Write-Host @"

    ██╗     ██╗   ██╗███╗   ██╗ █████╗ ██████╗ ██╗███████╗
    ██║     ██║   ██║████╗  ██║██╔══██╗██╔══██╗██║██╔════╝
    ██║     ██║   ██║██╔██╗ ██║███████║██████╔╝██║███████╗
    ██║     ██║   ██║██║╚██╗██║██╔══██║██╔══██╗██║╚════██║
    ███████╗╚██████╔╝██║ ╚████║██║  ██║██║  ██║██║███████║
    ╚══════╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚══════╝

                    🌙 Game Engine v$Version

"@ -ForegroundColor Magenta

function Test-Admin {
    $currentUser = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    return $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Install-Rust {
    Write-Host "Checking for Rust..." -ForegroundColor Yellow
    
    if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
        Write-Host "Installing Rust..." -ForegroundColor Yellow
        $rustupInit = "$env:TEMP\rustup-init.exe"
        Invoke-WebRequest -Uri "https://win.rustup.rs" -OutFile $rustupInit
        Start-Process -FilePath $rustupInit -ArgumentList "-y" -Wait
        $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
    }
    
    Write-Host "✓ Rust installed" -ForegroundColor Green
}

function Install-Lunaris {
    Write-Host "Installing Lunaris Engine..." -ForegroundColor Yellow
    
    # Create directories
    New-Item -ItemType Directory -Force -Path "$InstallDir\bin" | Out-Null
    New-Item -ItemType Directory -Force -Path "$InstallDir\lib" | Out-Null
    New-Item -ItemType Directory -Force -Path "$InstallDir\docs" | Out-Null
    
    # Download release
    $ReleaseUrl = "https://github.com/$Repo/releases/download/v$Version/lunaris-windows-x64.zip"
    $ZipPath = "$env:TEMP\lunaris.zip"
    
    try {
        Write-Host "Downloading from $ReleaseUrl..." -ForegroundColor Cyan
        Invoke-WebRequest -Uri $ReleaseUrl -OutFile $ZipPath
        Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force
        Remove-Item $ZipPath
        Write-Host "✓ Downloaded Lunaris $Version" -ForegroundColor Green
    }
    catch {
        Write-Host "Download failed, building from source..." -ForegroundColor Yellow
        Build-FromSource
    }
}

function Build-FromSource {
    Write-Host "Building from source..." -ForegroundColor Yellow
    
    $BuildDir = "$env:TEMP\lunaris-build"
    
    if (Test-Path $BuildDir) {
        Remove-Item -Recurse -Force $BuildDir
    }
    
    git clone --depth 1 "https://github.com/$Repo.git" $BuildDir
    Push-Location $BuildDir
    
    cargo build --release
    
    Copy-Item "target\release\*.exe" "$InstallDir\bin\" -Force -ErrorAction SilentlyContinue
    Copy-Item -Recurse "docs" "$InstallDir\" -Force
    
    Pop-Location
    Remove-Item -Recurse -Force $BuildDir
    
    Write-Host "✓ Built Lunaris from source" -ForegroundColor Green
}

function Set-EnvPath {
    Write-Host "Setting up environment..." -ForegroundColor Yellow
    
    # Set environment variables
    [Environment]::SetEnvironmentVariable("LUNARIS_HOME", $InstallDir, [EnvironmentVariableTarget]::User)
    
    $currentPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
    $binPath = "$InstallDir\bin"
    
    if ($currentPath -notlike "*$binPath*") {
        [Environment]::SetEnvironmentVariable("Path", "$binPath;$currentPath", [EnvironmentVariableTarget]::User)
    }
    
    # Update current session
    $env:LUNARIS_HOME = $InstallDir
    $env:Path = "$binPath;$env:Path"
    
    Write-Host "✓ Environment configured" -ForegroundColor Green
}

function Create-Shortcut {
    Write-Host "Creating shortcuts..." -ForegroundColor Yellow
    
    $WshShell = New-Object -ComObject WScript.Shell
    
    # Desktop shortcut
    $Shortcut = $WshShell.CreateShortcut("$env:USERPROFILE\Desktop\Lunaris Editor.lnk")
    $Shortcut.TargetPath = "$InstallDir\bin\lunaris-editor.exe"
    $Shortcut.WorkingDirectory = $InstallDir
    $Shortcut.Description = "Lunaris Game Engine Editor"
    $Shortcut.Save()
    
    # Start menu shortcut
    $StartMenu = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Lunaris"
    New-Item -ItemType Directory -Force -Path $StartMenu | Out-Null
    
    $Shortcut = $WshShell.CreateShortcut("$StartMenu\Lunaris Editor.lnk")
    $Shortcut.TargetPath = "$InstallDir\bin\lunaris-editor.exe"
    $Shortcut.WorkingDirectory = $InstallDir
    $Shortcut.Save()
    
    Write-Host "✓ Shortcuts created" -ForegroundColor Green
}

function Verify-Installation {
    Write-Host "Verifying installation..." -ForegroundColor Yellow
    
    if (Test-Path $InstallDir) {
        Write-Host "✓ Lunaris installed at $InstallDir" -ForegroundColor Green
        return $true
    }
    
    Write-Host "✗ Installation failed" -ForegroundColor Red
    return $false
}

# Main
Write-Host "Starting Lunaris Engine installation..." -ForegroundColor Blue
Write-Host ""

Install-Rust
Install-Lunaris
Set-EnvPath
Create-Shortcut

if (Verify-Installation) {
    Write-Host @"

╔════════════════════════════════════════════════════════╗
║       🌙 Lunaris Engine installed successfully!       ║
╠════════════════════════════════════════════════════════╣
║  Location: $InstallDir
║                                                        
║  To get started:                                       
║    1. Restart PowerShell/Terminal                      
║    2. Run: lunaris-editor                              
║                                                        
║  Or use the desktop shortcut!                          
║                                                        
║  Documentation: https://docs.lunaris.dev               
╚════════════════════════════════════════════════════════╝

"@ -ForegroundColor Green
}
