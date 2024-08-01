#!/bin/sh

# Function to install prerequisites on Debian-based systems
install_rust_debian() {
    sudo apt update
    sudo apt install -y ffmpeg mpv

    # Download yt-dlp binary from the official website since the debian package is outdated
    YT_DLP_URL="https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp"
    YT_DLP_BIN="/usr/local/bin/yt-dlp"

    sudo curl -L $YT_DLP_URL -o $YT_DLP_BIN
    sudo chmod +x $YT_DLP_BIN
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
curl -s https://api.github.com/repos/HFMorais/low-resource-youtube-playtime/releases/latest \
| grep "browser_download_url.*lryp" \
| cut -d : -f 2,3 \
| tr -d \" \
| wget -O /usr/local/bin/lryp -qi -

chmod +x /usr/local/bin/lryp

# Add mpv configuration
username=$(id -u -n 1000)
home_dir=$(pwd)

# Make sure the dir and file exists
mkdir -p "/home/$username/.config/mpv"
touch /home/$username/.config/mpv/mpv.conf
chown $username:$username /home/$username/.config/mpv
chown $username:$username /home/$username/.config/mpv/*

# Create a backup of the mpv configuraton file if it exists
if [ -e /home/$username/.config/mpv/mpv.conf ]; then
	mv /home/$username/.config/mpv/mpv.conf /home/$username/.config/mpv/mpv.conf.bak 
fi

echo "script-opts=ytdl_hook-ytdl_path=$(which yt-dlp)" >> /home/$username/.config/mpv/mpv.conf

echo "Installation completed successfully."
