#!/bin/bash

# Check if the video url is passed as argument, if it's not then use the clipboard
if [ "$#" -eq 0 ]
then
    video_url=$(xclip -selection c -o)
else
    video_url=$1
fi

regex='(https?|file)://[-[:alnum:]\+&@#/%?=~_|!:,.;]*[-[:alnum:]\+&@#/%=~_|]'
if [[ $video_url =~ $regex ]]
then 
    if [[ "$video_url" == *"shorts"* ]]; then
        mpv --profile=shorts $video_url
    elif [[ "$video_url" == *"twitch.tv"* ]]; then
        # To simplify this process, I just gave up and started using streamlink since it has better results in terms of buffering.
        # Will keep the yt-dlp only aproach for fallback
        streamlink --twitch-disable-ads --player mpv --default-stream 720p,1080p,best $video_url
    else
        mpv --profile=default $video_url
    fi
else
    exit 1
fi
