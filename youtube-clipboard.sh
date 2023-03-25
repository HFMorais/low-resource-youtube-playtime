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
        # Get the first instance of 720p quality video format with audio
        video_format=$( yt-dlp --list-formats $video_url | grep -v "audio only" | grep -v "video only" | grep "720" | head -n1 | awk '{print $1;}' )

        # If no 720p format was found revert to source
        [[ -z "$video_format" ]] && video_format=$( yt-dlp --list-formats $video_url | grep -v "audio only" | grep -v "video only" | grep "source" | head -n1 | awk '{print $1;}' )

        # Pipe the yt-dlp stream to mpv instead of using mpv configuration
        yt-dlp -f $video_format -q -o - $video_url | mpv -
    else
        mpv --profile=tempfix $video_url
    fi
else
    exit 1
fi
