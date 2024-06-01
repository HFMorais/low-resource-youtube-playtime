extern crate regex;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::process::Command;
use regex::Regex;
use std::env;

fn main() {

    let mut video_url: String = String::new();
    let mut video_quality: String = "720".to_string();
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" => {
                println!("Usage: command [-h] [-q quality] <URL>");
                println!("-h: Print this help message.");
                println!("-q: Specify the video quality (e.g., 720, 1080, 360). If the specified quality is not available, 720 and upwards will be used.");
                println!("<URL>: URL of the video to be played.");
                return;
            },
            "-q" if i + 1 < args.len() => {
                if args[i + 1].parse::<u32>().is_err() {
                    eprintln!("Error: The format argument must be a number.");
                    return;
                }
                video_quality = args[i + 1].clone();
                i += 1; // Skip next argument since it's part of -f
            },
            _ if video_url.is_empty() => {
                video_url = args[i].clone();
                println!("Found argument {}", video_url);
            },
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
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
                eprintln!("Error getting clipboard contents: {}", e);
                return;
            },
        };
    }
    
    let regex_expression =  r"(https?|file)://[-[:alnum:]\+&@#/%?=~_|!:,.;]*[-[:alnum:]\+&@#/%=~_|]";
    let re = Regex::new(regex_expression).unwrap();

    // Validates if the clipboard is a valid url
    if !re.is_match(video_url.as_str()) {
        println!("The string is not a valid url.");
        return;
    }

    let output = Command::new("yt-dlp")
        .args(&["--list-formats", video_url.as_str()])
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", stderr);
        return;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut good_options: Vec<&str> = Vec::new();

    let stream_options: Vec<&str> = stdout.lines().collect();
    for option in stream_options {
        if !(option.contains("mp4") || option.contains("webm")) {
            continue;
        } else if option.contains("audio only") {
            continue;
        } else if option.contains("video only") {
            continue;
        }

        good_options.push(option);
    }

    if good_options.is_empty() {
        println!("No good video formats were found");
        return;
    }

    let stream_id: String = fetch_available_stream_id(&good_options, &video_quality);
    let format_parameter: String = format!("{}{}", "--ytdl-format=", stream_id);

    Command::new("mpv")
        .args(&[format_parameter, video_url])
        .output()
        .expect("Failed to execute command");
}

fn fetch_available_stream_id(available_options: &Vec<&str>, video_quality: &str) -> String {

    
    if let Some(stream_id) = find_stream_id_by_quality(&available_options, video_quality) {
        println!("Found quality: {} with stream_id: {}", video_quality, stream_id);
        return stream_id;
    }
    if let Some(stream_id) = find_stream_id_by_quality(&available_options, "720") {
        println!("Found quality: 720 with stream_id: {}", stream_id);
        return stream_id;
    }
    if let Some(stream_id) = find_stream_id_by_quality(&available_options, "1080") {
        println!("Found quality: 1080 with stream_id: {}", stream_id);
        return stream_id;
    }

    let last_item = &available_options.last().unwrap();
    return last_item.split_whitespace().next().unwrap().to_string();
}


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
