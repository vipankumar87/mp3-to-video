use std::env;
use std::fs;
use std::process::{Command, exit};
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::Write;

fn get_audio_duration(mp3_path: &str) -> u64 {
    // Get the duration of the audio file
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(mp3_path)
        .output()
        .expect("Failed to get audio duration");

    let duration_str = String::from_utf8_lossy(&output.stdout);
    let duration = duration_str.trim().parse::<f64>().expect("Failed to parse duration");
    duration as u64 // Convert to seconds
}

fn merge_mp3(mp3_files: &[&str], output_path: &str) {
    // Create a temporary file with the MP3 paths to merge

    let temp_file = "input_files.txt";
    let mut file = File::create(temp_file).expect("Unable to create file");

    for mp3 in mp3_files {
        writeln!(file, "file '{}'", mp3).expect("Unable to write to file");
    }

    // Use FFmpeg to merge MP3 files
    let output = Command::new("ffmpeg")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(temp_file)
        .arg("-c")
        .arg("copy")
        .arg(output_path)
        .output()
        .expect("Failed to merge MP3 files");

    if !output.status.success() {
        eprintln!("Error merging MP3 files: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    println!("MP3 files merged into: {}", output_path);
    fs::remove_file(temp_file).expect("Failed to delete temp file list");
}

fn create_video(mp3_path: &str, image_folder: &str, output_video: &str, effects: &[&str], apply_all: bool) {
    // Get all images from the folder
    let image_paths: Vec<_> = fs::read_dir(image_folder)
        .expect("Failed to read image folder")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_file() {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();

    if image_paths.is_empty() {
        eprintln!("No images found in the provided folder");
        exit(1);
    }

    // Randomly select an image for each video frame
    let mut rng = rand::thread_rng();
    let random_image = image_paths.choose(&mut rng).expect("Failed to choose random image");

    // Get the duration of the audio file to set the video duration dynamically
    let audio_duration = get_audio_duration(mp3_path);

    // Define opacity overlay effect (we'll reduce opacity to 0.3 for the overlay)
    let opacity_overlay_effect = "format=yuva420p,alphaextract,fade=t=out:st=0:d=3,overlay=shortest=1:x=0:y=0:format=yuv420p";

    // If apply_all is true, apply all effects sequentially
    let effects_str = if apply_all {
        effects.join(",")
    } else {
        // Otherwise, apply a random effect from the list
        effects.choose(&mut rng).unwrap_or(&"").to_string()
    };

    // Use FFmpeg to create the video from the MP3 and the image with effects
    let output = Command::new("ffmpeg")
        .arg("-loop")
        .arg("1")  // Loop the image for the length of the video
        .arg("-framerate")
        .arg("1")  // You can adjust the framerate for more smooth transitions
        .arg("-t")
        .arg(audio_duration.to_string()) // Use dynamic audio length
        .arg("-i")
        .arg(random_image.to_str().unwrap())
        .arg("-i")
        .arg(mp3_path)
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-vf")
        .arg(if effects_str == "opacity_overlay" { opacity_overlay_effect } else { &effects_str }) // Apply opacity overlay if selected
        .arg("-shortest")
        .arg(output_video)
        .output()
        .expect("Failed to create video");

    if !output.status.success() {
        eprintln!("Error creating video: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    println!("Video created successfully: {}", output_video);
}
fn mp3_to_video(mp3_paths: &str, image_folder: &str, output_video: &str, effectsx: &str, apply_all: bool) {
    // Split the MP3 paths into individual file paths
    let mp3_files: Vec<&str> = mp3_paths.split(',').collect();

    // Handle single MP3 or multiple MP3 paths
    let mp3_path = if mp3_files.len() == 1 {
        mp3_files[0] // Use the single MP3 path directly
    } else {
        // Merge MP3 files if there are multiple
        let merged_mp3_path = "merged_audio.mp3";
        merge_mp3(&mp3_files, merged_mp3_path);
        merged_mp3_path
    };

    // Fixing this line to collect effects correctly
    let effects: Vec<&str> = effectsx.split(',').collect::<Vec<&str>>(); // Effects list

    // Create video using the chosen MP3 file
    create_video(mp3_path, image_folder, output_video, &effects, apply_all);

    println!("Video created successfully: {}", output_video);

    // Clean up the temporary file list
    if mp3_files.len() > 1 {
        fs::remove_file("merged_audio.mp3").expect("Failed to delete merged MP3 file");
    }
}

fn break_video(mp4_path: &str, segment_duration: u64, output_folder: &str) {
    let duration = get_video_duration(mp4_path);

    let total_segments = (duration / segment_duration) + 1;

    // Create output folder if it doesn't exist
    if !fs::metadata(output_folder).is_ok() {
        fs::create_dir_all(output_folder).expect("Failed to create output folder");
    }

    for i in 0..total_segments {
        let start_time = i * segment_duration;
        let segment_output = format!("{}/segment_{}.mp4", output_folder, i + 1);

        // Use FFmpeg to split the video
        let split_output = Command::new("ffmpeg")
            .arg("-i")
            .arg(mp4_path)
            .arg("-ss")
            .arg(format!("{}", start_time))  // Start time for the segment
            .arg("-t")
            .arg(format!("{}", segment_duration)) // Duration for each segment
            .arg("-c:v")
            .arg("libx264")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("-c:a")
            .arg("aac")
            .arg("-strict")
            .arg("-2")
            .arg(&segment_output)
            .output()
            .expect("Failed to split video");

        if !split_output.status.success() {
            eprintln!("Error splitting video: {}", String::from_utf8_lossy(&split_output.stderr));
            exit(1);
        }

        println!("Segment created: {}", segment_output);
    }
}

fn get_video_duration(video_path: &str) -> u64 {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(video_path)
        .output()
        .expect("Failed to get video duration");

    let duration_str = String::from_utf8_lossy(&output.stdout);
    let duration = duration_str.trim().parse::<f64>().expect("Failed to parse duration");
    duration as u64 // Convert to seconds
}

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

        mp3_to_video(mp3_paths, image_folder, output_video, &effects, apply_all);
    } else if choice == "break_video" {
        if args.len() < 5 {
            eprintln!("Usage for break_video: cargo run break_video <mp4_path> <segment_duration> <output_folder>");
            exit(1);
        }

        let mp4_path = &args[2];
        let segment_duration = args[3].parse::<u64>().unwrap_or(30); // Default to 30 seconds
        let output_folder = &args[4];

        break_video(mp4_path, segment_duration, output_folder);
    } else {
        eprintln!("Invalid choice. Use 'mp3_to_video' or 'break_video'.");
        exit(1);
    }
}
