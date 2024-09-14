use scraper::{Html, Selector};
use reqwest::blocking::Client;
use regex::Regex;
use log::{info, error};
use chrono::Utc;
use rusqlite::Connection;

use crate::core::database_handler;
use crate::data_structures::Channel;
use crate::data_structures::VideoEntry;

pub fn fetch_channel_information(channel_url: &str) -> Option<Channel> {
    // Send a GET request to the channel URL
    let response = reqwest::blocking::get(channel_url).unwrap();

    // Check if the request was successful
    if !response.status().is_success() {
        error!("Failed to fetch channel page: HTTP {}", response.status());
        return None;
    }

    // Parse the HTML content
    let body = response.text().unwrap();
    //let document = Html::parse_document(&body);

    // Fetch the channel title
    let channel_title_option = fetch_youtube_channel_title(body);
    if channel_title_option.is_none() {
        error!("No channel title was found");
        return None;
    }

    let channel_id_option = fetch_youtube_channel_id(channel_url);
    if channel_id_option.is_none() {
        error!("No channel id was found");
        return None;
    }
    let channel = Channel {
        id: 0,
        name: channel_title_option.unwrap().replace(" - YouTube", ""),
        url: channel_url.to_string(),
        channel_id: channel_id_option
    };


    return Some(channel)
}

fn fetch_youtube_channel_title(html_body: String) -> Option<String> {
    // Regex to match the content within <title> tags
    let title_re = Regex::new(r#"<title>([^<]+)</title>"#).unwrap();

    if let Some(captures) = title_re.captures(&html_body) {
        if let Some(title) = captures.get(1) {
            info!("Found channel Title: {}", title.as_str());
            return Some(title.as_str().to_string());
        }
    }

    error!("No channel title found");
    None
}

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
    let re = Regex::new(r#"<link\s+rel="alternate"\s+type="application/rss\+xml"\s+title="RSS"\s+href="([^"]+)">"#).unwrap();
    if let Some(captures) = re.captures(&body) {
        if let Some(url) = captures.get(1) {
            let channel_id_re = Regex::new(r#"channel_id=([^&]+)"#).unwrap();

            if let Some(id_captures) = channel_id_re.captures(url.as_str()) {
                if let Some(channel_id) = id_captures.get(1) {
                    info!("Found channel ID: {}", channel_id.as_str());
                    return Some(channel_id.as_str().to_string());
                }
            }
        }
    }

    error!("Could not find channel ID in the page source");
    None
}

pub fn fetch_last_10_videos_from_channel(conn: &Connection, channel: &Channel) -> Vec<VideoEntry> {
    let url = format!("https://www.youtube.com/feeds/videos.xml?channel_id={}", channel.channel_id.as_ref().unwrap());

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

            let current_datetime = Utc::now();

            let mut video_entry = VideoEntry {
                id: -1,
                video_id: video_id.to_string(), 
                title: video_title.to_string(),
                channel_id: channel.id,
                added_date: current_datetime.timestamp(),
                seen: false
            };

            let video_add_result = database_handler::save_video_info(conn, &video_entry);
            if video_add_result.is_ok() {
                video_entry.id = video_add_result.unwrap();
            }

            entries.push(video_entry);

            info!("Found video: {} - {}", video_id, video_title);
            continue;
        }

    }

    return entries;
}