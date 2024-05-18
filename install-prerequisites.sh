#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "Root user is required, please run sudo ./install-prerequisites.sh" 2>&1
  exit 1
fi

# Function to install prerequisites on Debian-based systems
install_rust_debian() {
    sudo apt update
    sudo apt install -y ffmpeg mpv yt-dlp
}

# Function to install prerequisites on Arch-based systems
install_rust_arch() {
    sudo pacman -Syu --noconfirm
    sudo pacman -S --noconfirm ffmpeg mpv yt-dlp
}

# Function to install prerequisites on Fedora-based systems
install_rust_fedora() {
    sudo dnf check-update
    sudo dnf install -y ffmpeg mpv yt-dlp
}

# Function to install prerequisites on openSUSE-based systems
install_rust_opensuse() {
    sudo zypper refresh
    sudo zypper install -y ffmpeg mpv yt-dlp
}

# Detect the distribution and install Rust
if [ -f /etc/debian_version ]; then
    echo "Detected Debian-based system."
    install_rust_debian
elif [ -f /etc/arch-release ]; then
    echo "Detected Arch-based system."
    install_rust_arch
elif [ -f /etc/fedora-release ]; then
    echo "Detected Fedora-based system."
    install_rust_fedora
elif [ -f /etc/os-release ]; then
    . /etc/os-release
    if [ "$ID" == "opensuse" ] || [ "$ID_LIKE" == "suse" ]; then
        echo "Detected openSUSE-based system."
        install_rust_opensuse
    else
        echo "Unsupported distribution: $ID"
        exit 1
    fi
else
    echo "Unsupported Linux distribution."
    exit 1
fi

curl -o /usr/local/bin https://github.com/HFMorais/low-resource-youtube-playtime/releases/download/v0.9-beta/lryp
chmod +x /usr/local/bin/lryp

echo "Installation completed successfully."