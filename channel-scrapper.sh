#!/bin/bash

# This would be 100x easier in python, but I must be sick in the head

scrap_youtube_id () {
    # For now fetch only the first one, it has what we need
    channel_id_raw_input=$( curl -s $1 | grep -oP '{.*?}' | grep "\"browse_id\"" | head -1 )
    # For now python3 is required, until I change this thing to parse "json"
    echo $channel_id_raw_input | python3 -c "import sys, json; print(json.load(sys.stdin)['value'])" 
}

scrap_video_url () {
    xml_raw_input=$( curl -s "https://www.youtube.com/feeds/videos.xml?channel_id=$1" )

    # Titles
    echo "$( echo $xml_raw_input | xmllint --xpath '//*[local-name() = "title"]' - | grep "media:title" | sed -e 's/<[^>]*>//g' )"

    # Urls
    echo "$( echo $xml_raw_input | xmllint --xpath '//*[local-name() = "link"]' - | grep "alternate" | grep -v "channel" | sed -r 's/.*href="([^"]+).*/\1/g' )"

}

# Load array of channels from channel-list.txt 
channel_array=()
while IFS= read -r line; do
    channel_array+=("$line")
done <channel-list.txt

# Do stuff for each of the channels
for channel in "${channel_array[@]}"
do
    channel_name=$( echo "$channel" | cut -d "@" -f 2 ) 
    channel_id=$( scrap_youtube_id $channel )
        
    echo $channel_name : $channel_id

    echo "$( scrap_video_url $channel_id )"

    echo ""
done
