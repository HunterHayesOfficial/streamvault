use std::process::Command;

pub async fn download_chat(video_id: &str, streamer_name: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Command::new("which").arg("chat_downloader").status()?.success() {
        return Err("chat_downloader is not installed or not found in PATH".into());
    }

    let output = Command::new("chat_downloader")
        .args(&[
            &format!("https://www.youtube.com/watch?v={}", video_id),
            "--output",
            output_path,
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("chat_downloader failed with status: {}", output.status).into());
    }

    println!("@{} chat saved", streamer_name);
    Ok(())
}