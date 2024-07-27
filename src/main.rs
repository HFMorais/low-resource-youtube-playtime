extern crate regex;
extern crate log;
extern crate env_logger;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::process::Command;
use regex::Regex;
use std::env;
use log::{info, warn, error};

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
            "-h" => {
                println!("Usage: command [-h] [-q quality] <URL>");
                println!("-h: Print this help message.");
                println!("-v (--version): Print the version of the program.");
                println!("-q: Specify the video quality (e.g., 720, 1080, 360). If the specified quality is not available, 720 and upwards will be used.");
                println!("--simulate: Simulate the command without playing the video (debug only).");
                println!("<URL>: URL of the video to be played.");
                return;
            },
            "-v" | "--version" => {
                println!("v1.1");
                return;
            },
            "--simulate" => {
                info!("Simulating lryp...");
                simulate = true;
            },
            "-q" if i + 1 < args.len() => {
                if args[i + 1].parse::<u32>().is_err() {
                    error!("Error: The format argument must be a number.");
                    return;
                }
                video_quality = args[i + 1].clone();
                i += 1; // Skip next argument since it's part of -f
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

        info!("Found video url from clipboard: {}", video_url);
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