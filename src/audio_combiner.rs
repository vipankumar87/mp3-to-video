use std::process::Command;
use std::fs;
use rand::seq::SliceRandom;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_mp3_duration(file_path: &str) -> Result<f64, String> {
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", file_path,
            "-hide_banner",
            "-f", "null",
            "-"
        ])
        .output()
        .expect("Failed to execute ffmpeg");

    let stderr = String::from_utf8_lossy(&output.stderr);
    if let Some(duration_line) = stderr.lines().find(|line| line.contains("Duration")) {
        let parts: Vec<&str> = duration_line.split_whitespace().collect();
        if let Some(duration_str) = parts.get(1) {
            let hms: Vec<&str> = duration_str.trim_end_matches(',').split(':').collect();
            if hms.len() == 3 {
                let hours: f64 = hms[0].parse().unwrap_or(0.0);
                let minutes: f64 = hms[1].parse().unwrap_or(0.0);
                let seconds: f64 = hms[2].parse().unwrap_or(0.0);
                return Ok(hours * 3600.0 + minutes * 60.0 + seconds);
            }
        }
    }

    Err("Failed to extract duration.".to_string())
}

pub fn create_background_audio(main_mp3: &str, background_dir: &str, output_file: &str) -> Result<(), String> {
    let main_duration = get_mp3_duration(main_mp3)?;

    let paths = match fs::read_dir(background_dir) {
        Ok(paths) => paths,
        Err(_) => return Err("Failed to read background directory.".to_string()),
    };

    let mut background_files: Vec<String> = vec![];
    for path in paths {
        if let Ok(entry) = path {
            let path_str = entry.path().to_string_lossy().to_string();
            if path_str.ends_with(".mp3") {
                background_files.push(path_str);
            }
        }
    }

    if background_files.is_empty() {
        return Err("No background MP3 files found.".to_string());
    }

    // Randomly shuffle and pick files based on the Unix timestamp (odd/even logic)
    let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let filter_even = unix_time % 2 == 0;

    let selected_files: Vec<&String> = background_files
        .iter()
        .filter(|file| {
            let even_index = background_files.iter().position(|f| f == *file).unwrap_or(0) % 2 == 0;
            filter_even == even_index
        })
        .collect();

    let mut combined_duration = 0.0;
    let mut selected_audio: Vec<&String> = vec![];

    for file in selected_files {
        if combined_duration < main_duration {
            let file_duration = get_mp3_duration(file).unwrap_or(0.0);
            selected_audio.push(file);
            combined_duration += file_duration;
        } else {
            break;
        }
    }

    if combined_duration > main_duration {
        // Logic to trim the last file can be added here if needed.
    }

    // Use ffmpeg to concatenate the selected audio files and create the output
    let concat_list = selected_audio.join("|");
    Command::new("ffmpeg")
        .args(&[
            "-i", &concat_list,
            "-filter_complex", "concat=n=1:v=0:a=1",
            "-y", output_file,
        ])
        .status()
        .expect("Failed to execute ffmpeg");

    Ok(())
}
