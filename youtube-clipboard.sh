#!/bin/bash

clipboard=$(xclip -selection c -o)

regex='(https?|ftp|file)://[-[:alnum:]\+&@#/%?=~_|!:,.;]*[-[:alnum:]\+&@#/%=~_|]'
if [[ $clipboard =~ $regex ]]
then 
    echo "Link valid"
else
    exit 1
fi

mpv --profile=720p $clipboard