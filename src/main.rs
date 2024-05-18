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

    // Now you can use `clipboard_contents` as a variable in your main function
    // println!("Clipboard contents: {}", clipboard_contents);

    // Example usage: print the length of the clipboard contents
    // let length = clipboard_contents.len();
    // println!("Length of clipboard contents: {}", length);

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

    /*
     * So at this point, we have a list of good video formats, now lets try to select by preference:
     * 720p first, 1080p next, and if it fails, fetch the last one, since it will be the "best one".
     */

    // let result = find_stream_id_by_quality(good_options, "720");
    // match result {
    //     Some(x) => println!("yay? {x}"),
    //     None => println!("nay")
    // }


    if let Some(stream_id) = find_stream_id_by_quality(good_options, "720") {
        print!("Found stream: {}", stream_id);
    }



    /*
    if clipboard_contents.contains("shorts") {

    } else if clipboard_contents.contains("twitch.tv") {

    } else {
        
    }
    */ 

    // yt-dlp -f <format_id> <url>
    // https://www.youtube.com/watch?v=h5dsmj8LPNk
    // mpv --ytdl-format=300 'https://www.youtube.com/watch?v=Bbrm1ldCBKY'
    // mpv --ytdl-format=720p 'https://www.twitch.tv/laylacodesit'
}

fn find_stream_id_by_quality(available_options: Vec<&str>, quality: &str) -> Option<String> {
    for option in available_options {
        if option.contains(quality) {
            return Some(option.to_string());
        }
    }
    None
}