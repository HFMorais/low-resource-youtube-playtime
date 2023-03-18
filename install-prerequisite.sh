#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "Root user is required, please run sudo ./install-prerequisite.sh" 2>&1
  exit 1
fi

username=$(id -u -n 1000)
home_dir=$(pwd)

apt-get update
apt-get install python3 python3-pip mpv -y

# I know I know... a root instalation of a pip package
python3 -m pip install -U yt-dlp

cd "$home_dir" || exit
mkdir -p "/home/$username/.config/mpv"
# Just to be sure the file exists
touch /home/$username/.config/mpv/mpv.conf

# Add mpv configurations
echo "script-opts=ytdl_hook-ytdl_path=/usr/local/bin/yt-dlp" >> /home/$username/.config/mpv/mpv.conf
echo "" >> /home/$username/.config/mpv/mpv.conf
echo "[1080p]" >> /home/$username/.config/mpv/mpv.conf
echo "ytdl-format=bestvideo[height<=?1080]+bestaudio/best" >> /home/$username/.config/mpv/mpv.conf
echo "" >> /home/$username/.config/mpv/mpv.conf
echo "[720p]" >> /home/$username/.config/mpv/mpv.conf
echo "ytdl-format=bestvideo[height<=?720]+bestaudio/best" >> /home/$username/.config/mpv/mpv.conf
echo "[480p]" >> /home/$username/.config/mpv/mpv.conf
echo "ytdl-format=bestvideo[height<=?480]+bestaudio/best" >> /home/$username/.config/mpv/mpv.conf
echo "" >> /home/$username/.config/mpv/mpv.conf
echo "[360p]" >> /home/$username/.config/mpv/mpv.conf
echo "ytdl-format=bestvideo[height<=?360]+bestaudio/best" >> /home/$username/.config/mpv/mpv.conf
