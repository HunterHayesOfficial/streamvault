use std::error::Error;
use std::env;
use reqwest::Client;
use serde::Deserialize;
use crate::services::database::Streamer;

#[derive(Clone)]
pub struct YouTubeClient {
    pub client: Client,
    pub api_key: String,
}

impl YouTubeClient {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        dotenv::dotenv().ok();
        let api_key = env::var("YOUTUBE_API_KEY")?;
        let client = Client::new();
        Ok(Self { client, api_key })
    }

    pub async fn check_live_stream(&self, streamer: &Streamer) -> Result<Option<(String, String)>, Box<dyn Error>> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=id,snippet&channelId={}&eventType=live&type=video&key={}",
            streamer.channel_id, self.api_key
        );

        let response = self.client.get(&url).send().await?.json::<LiveStreamResponse>().await?;

        if let Some(item) = response.items.into_iter().next() {
            if item.snippet.live_broadcast_content == "live" {
                let video_id = item.id.video_id.unwrap_or_default();
                let title = item.snippet.title;
                return Ok(Some((video_id, title)));
            }
        }

        Ok(None)
    }

    pub async fn get_channel_id_by_name(&self, channel_name: &str) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=id,snippet&maxResults=1&q={}&type=channel&key={}",
            channel_name, self.api_key
        );

        let response = self.client.get(&url).send().await?.json::<ChannelSearchResponse>().await?;

        if let Some(item) = response.items.into_iter().next() {
            return Ok(item.id.channel_id);
        }

        Err(format!("No channel found with name: {}", channel_name).into())
    }
}

#[derive(Deserialize, Debug)]
struct LiveStreamResponse {
    items: Vec<LiveStreamItem>,
}

#[derive(Deserialize, Debug)]
struct LiveStreamItem {
    id: LiveStreamItemId,
    snippet: LiveStreamSnippet,
}

#[derive(Deserialize, Debug)]
struct LiveStreamItemId {
    #[serde(rename = "videoId")]
    video_id: Option<String>,
}

#[derive(Deserialize, Debug)]
struct LiveStreamSnippet {
    title: String,
    #[serde(rename = "liveBroadcastContent")]
    live_broadcast_content: String,
}

#[derive(Deserialize, Debug)]
struct ChannelSearchResponse {
    items: Vec<ChannelSearchItem>,
}

#[derive(Deserialize, Debug)]
struct ChannelSearchItem {
    id: ChannelId,
}

#[derive(Deserialize, Debug)]
struct ChannelId {
    #[serde(rename = "channelId")]
    channel_id: String,
}