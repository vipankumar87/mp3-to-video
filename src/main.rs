mod audio;  // Declare audio module
mod video;  // Declare video module
mod utils;  // Declare utils module

use std::env;
use std::process::exit;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run <choice> <param1> <param2> ...");
        exit(1);
    }

    let choice = &args[1]; // choice = "mp3_to_video" or "break_video"
    
    if choice == "mp3_to_video" {
        if args.len() < 6 {
            eprintln!("Usage for mp3_to_video: cargo run mp3_to_video <mp3_paths> <image_folder> <output_video> <effects> <apply_all (true/false)>");
            exit(1);
        }

        let mp3_paths = &args[2]; // Comma-separated MP3 paths
        let image_folder = &args[3]; 
        let output_video = &args[4];
        let effects = &args[5]; // Effects list
        let apply_all = args[6].parse::<bool>().unwrap_or(true);

        video::mp3_to_video(mp3_paths, image_folder, output_video, &effects, apply_all);
    } else if choice == "break_video" {
        if args.len() < 5 {
            eprintln!("Usage for break_video: cargo run break_video <mp4_path> <segment_duration> <output_folder>");
            exit(1);
        }

        let mp4_path = &args[2];
        let segment_duration = args[3].parse::<u64>().unwrap_or(30); // Default to 30 seconds
        let output_folder = &args[4];

        video::break_video(mp4_path, segment_duration, output_folder);
    } else {
        eprintln!("Invalid choice. Use 'mp3_to_video' or 'break_video'.");
        exit(1);
    }
}
