use scraper::{Html, Selector};
use reqwest::blocking::Client;
use regex::Regex;
use log::{info, error};

use crate::VideoEntry;

/**
 * Fetches the YouTube channel ID from a channel URL.
 * 
 * @param channel_url: The URL of the YouTube channel.
 * @return: Option containing the channel ID.
 */
pub fn fetch_youtube_channel_id(channel_url: &str) -> Option<String> {
    // Send a GET request to the channel URL
    let response = reqwest::blocking::get(channel_url).unwrap();

    // Check if the request was successful
    if !response.status().is_success() {
        error!("Failed to fetch channel page: HTTP {}", response.status());
        return None;
    }

    // Parse the HTML content
    let body = response.text().unwrap();
    let document = Html::parse_document(&body);

    // Try to find the channel ID in the meta tags
    let selector = Selector::parse("meta[itemprop='channelId']").unwrap();
    if let Some(element) = document.select(&selector).next() {
        if let Some(channel_id) = element.value().attr("content") {
            info!("Found channel ID: {}", channel_id);
            return Some(channel_id.to_string());
        }
    }

    // If not found in meta tags, try to find it in the page source
    let re = Regex::new(r#""channelId"\s*:\s*"([^"]+)""#).unwrap();
    if let Some(captures) = re.captures(&body) {
        if let Some(channel_id) = captures.get(1) {
            info!("Found channel ID: {}", channel_id.as_str());
            return Some(channel_id.as_str().to_string());
        }
    }

    error!("Could not find channel ID in the page source");
    None
}

pub fn fetch_last_10_videos_from_channel() -> Vec<VideoEntry> {
    let channel_id = "UCdBK94H6oZT2Q7l0-b0xmMg";
    let url = format!("https://www.youtube.com/feeds/videos.xml?channel_id={}", channel_id);

    let mut entries = Vec::new();

    // Create an HTTP client
    let client = Client::new();

    let response = client.get(&url).send().unwrap();

    if !response.status().is_success() {
        error!("Failed to fetch RSS feed: HTTP {}", response.status());
        return entries;
    }

    let content = response.text().unwrap();

    let video_id_regex = Regex::new(r"<yt:videoId>(.*?)</yt:videoId>").unwrap();
    let video_title_regex = Regex::new(r"<media:title>(.*?)</media:title>").unwrap();
    
    let mut found_video_id = false;
    let mut found_video_title = false;

    let mut video_id = "";
    let mut video_title = "";

    for line in content.lines() {
        if let Some(video_id_capture) = video_id_regex.captures(line) {
            video_id = video_id_capture.get(1).unwrap().as_str();
            found_video_id = true;
        }
        
        if let Some(video_title_capture) = video_title_regex.captures(line) {
            video_title = video_title_capture.get(1).unwrap().as_str();
            found_video_title = true;
        }
        
        if found_video_id && found_video_title {
            found_video_id = false;
            found_video_title = false;

            let video_entry = VideoEntry { video_id: video_id.to_string(), title: video_title.to_string() };
            entries.push(video_entry);

            info!("Found video: {} - {}", video_id, video_title);
            continue;
        }

    }

    return entries;
}