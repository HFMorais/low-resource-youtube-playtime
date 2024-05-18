extern crate regex;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::process::Command;
use regex::Regex;

fn main() {
    // Create a new clipboard context
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    // Get the text from the clipboard
    let clipboard_contents: String = match ctx.get_contents() {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error getting clipboard contents: {}", e);
            return;
        },
    };

    let regex_expression =  r"(https?|file)://[-[:alnum:]\+&@#/%?=~_|!:,.;]*[-[:alnum:]\+&@#/%=~_|]";
    let re = Regex::new(regex_expression).unwrap();

    // Validates if the clipboard is a valid url
    if !re.is_match(clipboard_contents.as_str()) {
        println!("The string is not a valid url.");
        return;
    }

    // Add character to the beginning
    let mut video_url = format!("{}{}", '\'', clipboard_contents);
    
    // Add character to the end (alternative syntax)
    video_url.push('\'');

    let output = Command::new("yt-dlp")
        .args(&["--list-formats", clipboard_contents.as_str()])
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

    let stream_id: String = fetch_available_stream_id(&good_options);
    let format_parameter: String = format!("{}{}", "--ytdl-format=", stream_id);

    Command::new("mpv")
        .args(&[format_parameter, clipboard_contents])
        .output()
        .expect("Failed to execute command");
}

fn fetch_available_stream_id(available_options: &Vec<&str>) -> String {
    if let Some(stream_id) = find_stream_id_by_quality(&available_options, "720") {
        return stream_id;
    }
    if let Some(stream_id) = find_stream_id_by_quality(&available_options, "1080") {
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