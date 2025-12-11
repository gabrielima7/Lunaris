#!/bin/bash
#
# Lunaris Engine Installer for Linux/macOS
#
# Usage: curl -fsSL https://lunaris.dev/install.sh | bash
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Banner
echo -e "${PURPLE}"
cat << 'EOF'

    ██╗     ██╗   ██╗███╗   ██╗ █████╗ ██████╗ ██╗███████╗
    ██║     ██║   ██║████╗  ██║██╔══██╗██╔══██╗██║██╔════╝
    ██║     ██║   ██║██╔██╗ ██║███████║██████╔╝██║███████╗
    ██║     ██║   ██║██║╚██╗██║██╔══██║██╔══██╗██║╚════██║
    ███████╗╚██████╔╝██║ ╚████║██║  ██║██║  ██║██║███████║
    ╚══════╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚══════╝

                    🌙 Game Engine v1.0.0

EOF
echo -e "${NC}"

# Configuration
VERSION="${LUNARIS_VERSION:-1.0.0}"
INSTALL_DIR="${LUNARIS_INSTALL_DIR:-$HOME/.lunaris}"
REPO="gabrielima7/Lunaris"

# Detect OS and Architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux*)     PLATFORM="linux" ;;
        Darwin*)    PLATFORM="macos" ;;
        MINGW*|MSYS*|CYGWIN*)    PLATFORM="windows" ;;
        *)          echo -e "${RED}Unsupported OS: $OS${NC}"; exit 1 ;;
    esac
    
    case "$ARCH" in
        x86_64|amd64)    ARCH="x64" ;;
        arm64|aarch64)    ARCH="arm64" ;;
        *)               echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
    esac
    
    echo -e "${CYAN}Detected: $PLATFORM-$ARCH${NC}"
}

# Check dependencies
check_deps() {
    echo -e "${YELLOW}Checking dependencies...${NC}"
    
    # Check for curl or wget
    if command -v curl &> /dev/null; then
        DOWNLOADER="curl -fsSL"
    elif command -v wget &> /dev/null; then
        DOWNLOADER="wget -qO-"
    else
        echo -e "${RED}Error: curl or wget required${NC}"
        exit 1
    fi
    
    # Check for Rust
    if ! command -v rustc &> /dev/null; then
        echo -e "${YELLOW}Rust not found. Installing...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    echo -e "${GREEN}✓ Dependencies satisfied${NC}"
}

# Download and install
install() {
    echo -e "${YELLOW}Installing Lunaris Engine...${NC}"
    
    # Create install directory
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$INSTALL_DIR/bin"
    mkdir -p "$INSTALL_DIR/lib"
    mkdir -p "$INSTALL_DIR/docs"
    
    # Download release
    RELEASE_URL="https://github.com/$REPO/releases/download/v$VERSION/lunaris-$PLATFORM-$ARCH"
    
    if [ "$PLATFORM" = "windows" ]; then
        ARCHIVE="$RELEASE_URL.zip"
        $DOWNLOADER "$ARCHIVE" > /tmp/lunaris.zip
        unzip -q /tmp/lunaris.zip -d "$INSTALL_DIR"
        rm /tmp/lunaris.zip
    else
        ARCHIVE="$RELEASE_URL.tar.gz"
        $DOWNLOADER "$ARCHIVE" | tar -xzf - -C "$INSTALL_DIR"
    fi
    
    echo -e "${GREEN}✓ Downloaded Lunaris $VERSION${NC}"
}

# Build from source (fallback)
build_from_source() {
    echo -e "${YELLOW}Building from source...${NC}"
    
    # Clone repository
    if [ -d "/tmp/lunaris-build" ]; then
        rm -rf /tmp/lunaris-build
    fi
    
    git clone --depth 1 "https://github.com/$REPO.git" /tmp/lunaris-build
    cd /tmp/lunaris-build
    
    # Build
    cargo build --release
    
    # Install
    mkdir -p "$INSTALL_DIR/bin"
    cp target/release/lunaris-* "$INSTALL_DIR/bin/" 2>/dev/null || true
    cp -r docs "$INSTALL_DIR/"
    
    # Cleanup
    cd -
    rm -rf /tmp/lunaris-build
    
    echo -e "${GREEN}✓ Built Lunaris from source${NC}"
}

# Setup PATH
setup_path() {
    echo -e "${YELLOW}Setting up PATH...${NC}"
    
    SHELL_RC=""
    case "$SHELL" in
        */bash)  SHELL_RC="$HOME/.bashrc" ;;
        */zsh)   SHELL_RC="$HOME/.zshrc" ;;
        */fish)  SHELL_RC="$HOME/.config/fish/config.fish" ;;
    esac
    
    if [ -n "$SHELL_RC" ] && [ -f "$SHELL_RC" ]; then
        if ! grep -q "LUNARIS_HOME" "$SHELL_RC"; then
            echo "" >> "$SHELL_RC"
            echo "# Lunaris Engine" >> "$SHELL_RC"
            echo "export LUNARIS_HOME=\"$INSTALL_DIR\"" >> "$SHELL_RC"
            echo "export PATH=\"\$LUNARIS_HOME/bin:\$PATH\"" >> "$SHELL_RC"
        fi
    fi
    
    export LUNARIS_HOME="$INSTALL_DIR"
    export PATH="$INSTALL_DIR/bin:$PATH"
    
    echo -e "${GREEN}✓ PATH configured${NC}"
}

# Verify installation
verify() {
    echo -e "${YELLOW}Verifying installation...${NC}"
    
    if [ -d "$INSTALL_DIR" ]; then
        echo -e "${GREEN}✓ Lunaris installed at $INSTALL_DIR${NC}"
    else
        echo -e "${RED}✗ Installation failed${NC}"
        exit 1
    fi
}

# Main
main() {
    echo -e "${BLUE}Starting Lunaris Engine installation...${NC}"
    echo ""
    
    detect_platform
    check_deps
    
    # Try download first, fallback to build
    if ! install 2>/dev/null; then
        echo -e "${YELLOW}Download failed, building from source...${NC}"
        build_from_source
    fi
    
    setup_path
    verify
    
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║       🌙 Lunaris Engine installed successfully!       ║${NC}"
    echo -e "${GREEN}╠════════════════════════════════════════════════════════╣${NC}"
    echo -e "${GREEN}║  Location: $INSTALL_DIR${NC}"
    echo -e "${GREEN}║                                                        ║${NC}"
    echo -e "${GREEN}║  To get started:                                       ║${NC}"
    echo -e "${GREEN}║    $ source ~/.bashrc   # or restart terminal          ║${NC}"
    echo -e "${GREEN}║    $ lunaris-editor     # Launch editor                ║${NC}"
    echo -e "${GREEN}║                                                        ║${NC}"
    echo -e "${GREEN}║  Documentation: https://docs.lunaris.dev               ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

main "$@"
