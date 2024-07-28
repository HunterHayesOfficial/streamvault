mod services;
mod api;
mod functions;

use std::error::Error;
use std::env;
use std::io::{self, Write};
use dotenv::dotenv;
use services::database::Database;
use services::recorder::Recorder;
use services::chat;
use api::youtube::YouTubeClient;
use chrono::Utc;
use functions::sanitize::sanitize_filename;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "streamvault.db".to_string());
    let database = Database::init(&db_path)?;

    let check_interval = env::var("CHECK_INTERVAL")
        .unwrap_or_else(|_| "60".to_string())
        .parse::<u64>()?;

    let youtube_client = YouTubeClient::new().await?;
    let recorder = Recorder::new(youtube_client.clone());

    println!("Do you want to add a YouTuber to the database? (y/n)");
    let mut input = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        println!("Enter the YouTube channel name:");
        let mut channel_name = String::new();
        io::stdin().read_line(&mut channel_name)?;
        channel_name = channel_name.trim().to_string();

        match youtube_client.get_channel_id_by_name(&channel_name).await {
            Ok(channel_id) => {
                database.add_streamer(&channel_name, &channel_id)?;
                println!("Added {} with channel ID {} to the database", channel_name, channel_id);
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(e.into());
            }
        }
    }

    loop {
        let streamers = database.get_streamers()?;

        for streamer in streamers {
            if recorder.is_live(&streamer).await? {
                let recorder_clone = recorder.clone();
                let video_streamer_clone = streamer.clone();
                tokio::spawn(async move {
                    if let Err(e) = recorder_clone.record_stream(&video_streamer_clone).await {
                        eprintln!("Error recording stream for {}: {}", video_streamer_clone.name, e);
                    }
                });

                let chat_streamer_clone = streamer.clone();
                let youtube_client_clone = youtube_client.clone();
                tokio::spawn(async move {
                    if let Some((video_id, stream_title)) = youtube_client_clone.check_live_stream(&chat_streamer_clone).await.unwrap() {
                        let home_dir = dirs::home_dir().unwrap();
                        let sanitized_title = sanitize_filename(&stream_title);
                        let current_time = Utc::now().format("%Y-%m-%d").to_string();
                        let file_name = format!("{}-{}", sanitized_title, current_time);
                        let chat_output_file = home_dir.join("streamvault")
                            .join(&chat_streamer_clone.name)
                            .join(format!("{}-chat.json", file_name));
                        
                        if let Err(e) = chat::download_chat(&video_id, &chat_streamer_clone.name, &chat_output_file.to_string_lossy()).await {
                            eprintln!("Error downloading chat for {}: {}", chat_streamer_clone.name, e);
                        } else {
                            println!("Chat downloaded successfully for {}: {}", chat_streamer_clone.name, file_name);
                        }
                    }
                });
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(check_interval)).await;
    }
}