extern crate regex;
extern crate log;
extern crate env_logger;
use scraper::{Html, Selector};
use reqwest::blocking::Client;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::process::Command;
use regex::Regex;
use std::env;
use log::{info, warn, error};

#[derive(Debug)]
struct VideoEntry {
    video_id: String,
    title: String,
}

fn main() {
     // Set the default log level to info if not set
     if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let mut video_url: String = String::new();
    let mut video_quality: String = "720".to_string();
    let mut simulate: bool = false;

    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                println!("Usage: command [-h] [-q quality] <URL>");
                println!("-h (--help): Print this help message.");
                println!("-v (--version): Print the version of the program.");
                println!("-q (--quality): Specify the video quality (e.g., 720, 1080, 360). If the specified quality is not available, 720 and upwards will be used.");
                println!("--simulate: Simulate the command without playing the video (debug only).");
                println!("<URL>: URL of the video to be played.");
                return;
            },
            "-v" | "--version" => {
                println!("v1.2");
                return;
            },
            "--simulate" => {
                info!("Simulating lryp...");
                simulate = true;
            },
            "-s" | "--scrapping" => {
                let channel_url = args[i + 1].clone();
                //let channel_id = fetch_youtube_channel_id("https://www.youtube.com/@LinusTechTips");
                let channel_id = fetch_youtube_channel_id(&channel_url);
                let video_entries = fetch_last_10_videos_from_channel();
                return;
            },
            "-q" | "--quality" if i + 1 < args.len() => {
                if args[i + 1].parse::<u32>().is_err() {
                    error!("Error: The format argument must be a number.");
                    return;
                }
                video_quality = args[i + 1].clone();
                info!("Searching for video with {} quality", video_quality);
                i += 1;
            },
            _ if video_url.is_empty() => {
                video_url = args[i].clone();
                info!("Using video url passed as argument {}", video_url);
            },
            _ => {
                error!("Unknown argument: {}", args[i]);
                return;
            }
        }
        i += 1;
    }

    if video_url.is_empty() {
        // Create a new clipboard context
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    
        // Get the text from the clipboard
        video_url = match ctx.get_contents() {
            Ok(contents) => contents,
            Err(e) => {
                error!("Error getting clipboard contents: {}", e);
                return;
            },
        };

        info!("Using content from clipboard as video url: {}", video_url);
    }
    
    let regex_expression =  r"(https?|file)://[-[:alnum:]\+&@#/%?=~_|!:,.;]*[-[:alnum:]\+&@#/%=~_|]";
    let re = Regex::new(regex_expression).unwrap();

    // Validates if the clipboard is a valid url
    if !re.is_match(video_url.as_str()) {
        error!("URL parameter string is not a valid url.");
        return;
    }

    let output = Command::new("yt-dlp")
        .args(&["--list-formats", video_url.as_str()])
        .output()
        .expect("Failed to execute yt-dlp --list-formats command");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Error: {}", stderr);
        return;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut combo_options: Vec<&str> = Vec::new();
    let mut audio_only_options: Vec<&str> = Vec::new();
    let mut video_only_options: Vec<&str> = Vec::new();

    let stream_options: Vec<&str> = stdout.lines().collect();
    for option in stream_options {
        if !(option.contains("mp4") || option.contains("webm") || option.contains("m4a")) {
            continue;
        } else if option.contains("audio only") {
            audio_only_options.push(option);
        } else if option.contains("video only") {
            video_only_options.push(option);
        } else {
            combo_options.push(option);
        }
    }

    // No video and audio options were found, so just error out
    if combo_options.is_empty() && audio_only_options.is_empty() && video_only_options.is_empty() {
        error!("No valid formats were found");
        return;
    }

    let mut stream_id: String = String::new();

    // First lets try to fetch a stream id that has video and audio combined for the video quality specified
    info!("Trying to find a combo stream id...");
    let combo_stream_id: Option<String> = fetch_available_video_stream_id(&combo_options, &video_quality, false);
    if let Some(id) = combo_stream_id {
        info!("Found combo stream id: {}", id);
        stream_id = id;
    }

    // Ok we didn't find a combo stream id, so lets try to fetch a video stream id and audio stream id to combine later
    if stream_id.is_empty() {
        info!("Trying to find a video and audio stream id...");
        let video_stream_id: Option<String> = fetch_available_video_stream_id(&video_only_options, &video_quality, false);
        let audio_stream_id: Option<String> = fetch_available_audio_stream_id(&audio_only_options);
        if let (Some(video_id), Some(audio_id)) = (video_stream_id, audio_stream_id) {
            info!("Found video and audio streams, video id: {}, audio id: {}", video_id, audio_id);
            stream_id = format!("{}+{}", video_id, audio_id);
        }
    }
    
    // Shit... we still didn't find a stream id, so lets try to find any stream id that can produce anything
    if stream_id.is_empty() {
        info!("Trying to find a last resort stream id...");
        let last_resort_id: Option<String> = fetch_available_video_stream_id(&video_only_options, &video_quality, true);
        if let Some(id) = last_resort_id {
            info!("Found last resort stream id: {}", id);
            stream_id = id;
        }
    }

    // There is no video id (simple or combo), so error out
    if stream_id.is_empty() {
        warn!("Unable to fetch a valid stream id.");
        return;
    }

    let format_parameter: String = format!("{}{}", "--ytdl-format=", stream_id);

    if simulate {
        info!("Simulating command: mpv {} {}", format_parameter, video_url);
        return;
    }

    Command::new("mpv")
        .args(&[format_parameter, video_url])
        .output()
        .expect("Failed to execute command");
}

/**
 * Fetches the available video stream id from the available options.
 * 
 * @param available_options: The available options to search through.
 * @param video_quality: The video quality to search for.
 * @param allow_last_resort: Whether to allow the last resort video option, any format will suffice.
 * @return: The available video stream id.
 */
fn fetch_available_video_stream_id(available_options: &Vec<&str>, video_quality: &str, allow_last_resort: bool) -> Option<String> {

    if let Some(stream_id) = find_stream_id_by_quality(&available_options, video_quality) {
        info!("Found video quality: {} with stream_id: {}", video_quality, stream_id);
        return Some(stream_id);
    }

    if allow_last_resort {
        if let Some(stream_id) = find_stream_id_by_quality(&available_options, "720") {
            info!("Found video quality: 720 with stream_id: {}", stream_id);
            return Some(stream_id);
        }
        if let Some(stream_id) = find_stream_id_by_quality(&available_options, "1080") {
            info!("Found video quality: 1080 with stream_id: {}", stream_id);
            return Some(stream_id);
        }

        let last_item = &available_options.last().unwrap();
        return Some(last_item.split_whitespace().next().unwrap().to_string());
    }

    None
}

/**
 * Fetches the available audio stream id from the available options.
 * For now fetches the best audio quality available. At a later time, may support selecting audio quality.
 * 
 * @param available_options: The available options to search through.
 * @return: The available audio stream id.
 */
fn fetch_available_audio_stream_id(available_options: &Vec<&str>) -> Option<String> {
    let last_item = &available_options.last().unwrap();
    return Some(last_item.split_whitespace().next().unwrap().to_string());
}

/**
 * Finds the stream id by quality from the available options.
 * 
 * @param available_options: The available options to search through.
 * @param quality: The quality to search for.
 * @return: The available stream id.
 */
fn find_stream_id_by_quality(available_options: &Vec<&str>, quality: &str) -> Option<String> {
    for option in available_options {
        if option.contains(quality) {
            if let Some(stream_id) = option.split_whitespace().next() {
                return Some(stream_id.to_string());
            }
        }
    }
    None
}

/**
 * Fetches the YouTube channel ID from a channel URL.
 * 
 * @param channel_url: The URL of the YouTube channel.
 * @return: Option containing the channel ID.
 */
fn fetch_youtube_channel_id(channel_url: &str) -> Option<String> {
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


fn fetch_last_10_videos_from_channel() -> Vec<VideoEntry> {
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

