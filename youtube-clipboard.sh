#!/bin/bash

clipboard=$(xclip -selection c -o)

regex='(https?|file)://[-[:alnum:]\+&@#/%?=~_|!:,.;]*[-[:alnum:]\+&@#/%=~_|]'
if [[ $clipboard =~ $regex ]]
then 
    if [[ "$clipboard" == *"shorts"* ]]; then
        mpv --profile=shorts $clipboard
    elif [[ "$clipboard" == *"twitch.tv"* ]]; then
        # Get the first instance of 720p quality video format with audio
        video_format=$( yt-dlp --list-formats $clipboard | grep -v "audio only" | grep -v "video only" | grep "720" | head -n1 | awk '{print $1;}' )

        # If no 720p format was found revert to source
        [[ -z "$video_format" ]] && video_format=$( yt-dlp --list-formats $clipboard | grep -v "audio only" | grep -v "video only" | grep "source" | head -n1 | awk '{print $1;}' )

        # Pipe the yt-dlp stream to mpv instead of using mpv configuration
        yt-dlp -f $video_format -q -o - $clipboard | mpv -
    else
        mpv --profile=tempfix $clipboard
    fi
else
    exit 1
fi
