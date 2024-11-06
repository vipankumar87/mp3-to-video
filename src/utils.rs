use std::process::Command;
pub fn get_audio_duration(mp3_path: &str) -> u64 {
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
pub fn get_video_duration(video_path: &str) -> u64 {
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