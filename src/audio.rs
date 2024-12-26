use crate::audio_combiner;
use std::fs::File;
use std::io::Write;
use std::process::{Command, exit};
use std::fs;

    pub fn merge_mp3(mp3_files: &[&str], output_path: &str) {
        // Create a temporary file with the MP3 paths to merge
    
        let temp_file = "input_files.txt";
        let mut file = File::create(temp_file).expect("Unable to create file");
        let tmp_output = "merge.mp3";
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
            .arg(tmp_output)
            .output()
            .expect("Failed to merge MP3 files");
    
        if !output.status.success() {
            eprintln!("Error merging MP3 files: {}", String::from_utf8_lossy(&output.stderr));
            exit(1);
        }
    
        let mp3s = "mp3s";
//        println!("MP3 files merged into: {}", tmp_output);
        let _ = audio_combiner::create_background_audio(&tmp_output, &mp3s, output_path);  // tmp_output can be used as is
        // fs::remove_file(tmp_output).expect("Failed to delete temp file list");
        fs::remove_file(temp_file).expect("Failed to delete temp file list");
    }
