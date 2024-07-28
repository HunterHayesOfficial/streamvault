use std::error::Error;
use std::process::Command;
use chrono::Utc;
use crate::services::database::Streamer;
use crate::api::youtube::YouTubeClient;
use crate::functions::sanitize::sanitize_filename;
use std::fs;

#[derive(Clone)]
pub struct Recorder {
    youtube_client: YouTubeClient,
}

impl Recorder {
    pub fn new(youtube_client: YouTubeClient) -> Self {
        Self { youtube_client }
    }

    pub async fn is_live(&self, streamer: &Streamer) -> Result<bool, Box<dyn Error>> {
        let live_stream = self.youtube_client.check_live_stream(streamer).await?;
        Ok(live_stream.is_some())
    }

    pub async fn record_stream(&self, streamer: &Streamer) -> Result<(), Box<dyn Error>> {
        let current_time = Utc::now().format("%Y-%m-%d").to_string();
        let home_dir = dirs::home_dir().ok_or("Unable to find home directory")?;
        let output_dir = home_dir.join("streamvault").join(&streamer.name);

        if let Some((video_id, stream_title)) = self.youtube_client.check_live_stream(streamer).await? {
            let sanitized_title = sanitize_filename(&stream_title);
            let file_name = format!("{}-{}.mp4", sanitized_title, current_time);
            let video_output_file = output_dir.join(&file_name);

            fs::create_dir_all(&output_dir)?;

            if Command::new("which").arg("yt-dlp").status()?.success() {
                let url = format!("https://www.youtube.com/watch?v={}", video_id);

                println!("@{} now recording: {}", streamer.name, file_name);

                let status = Command::new("yt-dlp")
                    .args(&["--format", "best", "--output", &video_output_file.to_string_lossy(), &url])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()?;

                if status.success() {
                    println!("@{} Finished recording: {}", streamer.name, file_name);
                    Ok(())
                } else {
                    Err(format!("yt-dlp failed with status: {}", status).into())
                }
            } else {
                Err("yt-dlp is not installed or not found in PATH".into())
            }
        } else {
            Err("No live stream found".into())
        }
    }
}