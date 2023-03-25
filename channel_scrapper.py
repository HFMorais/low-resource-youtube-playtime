#!/usr/bin/env python3

import requests
import feedparser

from tabulate import tabulate
from datetime import datetime

import urllib.request

CHANNEL_LIST_FILE = 'channel-list.txt'

CHANNEL_NAME_CACHE = dict()

MAX_ITEMS_PER_CHANNEL = 3

# Video object
class Video:
    def __init__(self, channel_id, video_url, video_title, video_date):
        self.channel_id = channel_id
        self.video_url = video_url
        self.video_title = video_title
        self.video_date = video_date

# Get the channel property based on the item html element
def fetch_channel_property(channel_source, split_value):
    first_pass = channel_source.split(split_value)[1]
    second_pass = first_pass.split('">')[0]

    return second_pass

# Parse the RSS feed of a channel
def parse_rss_feed(channel_id):
    rssFeedUrl = "https://www.youtube.com/feeds/videos.xml?channel_id="+channel_id
    NewsFeed = feedparser.parse(rssFeedUrl)

    channel_content = []

    current_index = 0

    # print(entry.keys())
    for entry in NewsFeed.entries:
        if current_index == MAX_ITEMS_PER_CHANNEL:
            break

        publish_date_raw = entry.published.replace("+00:00", "")
        publish_date_obj = datetime.strptime(publish_date_raw, '%Y-%m-%dT%H:%M:%S')
        #display_date = publish_date_obj.strftime("%d-%m-%Y %H:%M:%S")

        channel_video = Video(channel_id, entry.link, entry.title, publish_date_obj)
        channel_content.append(channel_video)

        current_index+=1

    return channel_content

# Fetch the page source in a string format
def fetch_channel_source(channel_url):
    page_source = requests.get(channel_url)
    return str(page_source.content)

def special_fetch_channel_source(channel_url):
    request = urllib.request.Request(
        channel_url,
        data=None,
        headers={
            'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/35.0.1916.47 Safari/537.36'
        }
    )

    try:
        with urllib.request.urlopen(request) as response:
            return response.read().decode("utf8")
    except:
        return None

if __name__ == "__main__":
    # Fetch channel list
    file = open(CHANNEL_LIST_FILE, 'r')

    Channels = file.readlines()

    available_content = []

    # Build the list of available videos
    for channel in Channels:
        #channel_source = fetch_channel_source(channel.strip())
        channel_source = special_fetch_channel_source(channel.strip())

        channel_id = fetch_channel_property(channel_source, '<meta itemprop="channelId" content="')
        
        # Add channel name to cache for later use
        if channel_id not in CHANNEL_NAME_CACHE:
            CHANNEL_NAME_CACHE[channel_id] = fetch_channel_property(channel_source, '<meta itemprop="name" content="')

        channel_available_content = parse_rss_feed(channel_id)
        for video in channel_available_content:
            available_content.append(video)

    # Do stuff to the found content
    date_sorted_list = sorted(available_content, key=lambda v: v.video_date, reverse=True)

    channel_dump_content = []
    for video in date_sorted_list:
        channel_name_temp = CHANNEL_NAME_CACHE[video.channel_id]
        formatted_date = video.video_date.strftime("%d-%m-%Y %H:%M:%S")

        video_information_dump = [video.video_url, video.video_title, channel_name_temp, formatted_date]
        channel_dump_content.append(video_information_dump)

    headers = ["URL", "Title", "Channel", "Date"]
    print(tabulate(channel_dump_content, headers, tablefmt="fancy_grid"))