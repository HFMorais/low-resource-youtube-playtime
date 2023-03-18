#!/bin/bash

clipboard=$(xclip -selection c -o)

regex='(https?|ftp|file)://[-[:alnum:]\+&@#/%?=~_|!:,.;]*[-[:alnum:]\+&@#/%=~_|]'
if [[ $clipboard =~ $regex ]]
then 
    if [[ "$clipboard" == *"shorts"* ]]; then
        mpv --profile=shorts $clipboard
    else
        mpv --profile=tempfix $clipboard
    fi
else
    exit 1
fi
