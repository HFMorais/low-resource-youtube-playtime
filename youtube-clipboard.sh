#!/bin/bash

clipboard=$(xclip -selection c -o)

regex='(https?|ftp|file)://[-[:alnum:]\+&@#/%?=~_|!:,.;]*[-[:alnum:]\+&@#/%=~_|]'
if [[ $clipboard =~ $regex ]]
then 
    mpv --profile=720p $clipboard
else
    exit 1
fi
