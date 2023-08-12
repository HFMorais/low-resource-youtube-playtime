#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "Root user is required, please run sudo ./install-prerequisites.sh" 2>&1
  exit 1
fi

username=$(id -u -n 1000)
home_dir=$(pwd)

apt update
apt install python3 curl python3-pip mpv xclip ffmpeg -y

# if you want to use the yt-dlp for twitch use the fallback branch
apt install streamlink -y

# Since youtube is always trying to screw people over, let's fetch a newer bin version from the source
wget -O /usr/local/bin/yt-dlp https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp
chmod +x /usr/local/bin/yt-dlp

# Install python required libs
apt install python3-tabulate python3-feedparser

cd "$home_dir" || exit

# Make sure the dir exists
mkdir -p "/home/$username/.config/mpv"

# Create a backup of the mpv configuraton file if it exists
if [ -e /home/$username/.config/mpv/mpv.conf ]; then
	mv /home/$username/.config/mpv/mpv.conf /home/$username/.config/mpv/mpv.conf.bak 
fi

# Copy the config file to the destination folder
cp dotfiles/mpv.conf "/home/$username/.config/mpv/"

chown $username:$username /home/$username/.config/mpv/mpv.conf
