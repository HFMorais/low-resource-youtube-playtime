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

# Download binary file and add it to bin folder
curl -o /usr/local/bin https://github.com/HFMorais/low-resource-youtube-playtime/releases/download/v0.9-beta/lryp
chmod +x /usr/local/bin/lryp

# Add mpv configuration
username=$(id -u -n 1000)
home_dir=$(pwd)

# Make sure the dir and file exists
mkdir -p "/home/$username/.config/mpv"
touch /home/$username/.config/mpv/mpv.conf

# Create a backup of the mpv configuraton file if it exists
if [ -e /home/$username/.config/mpv/mpv.conf ]; then
	mv /home/$username/.config/mpv/mpv.conf /home/$username/.config/mpv/mpv.conf.bak 
fi

echo "$(which yt-dlp)" >> /home/$username/.config/mpv/mpv.conf

echo "Installation completed successfully."