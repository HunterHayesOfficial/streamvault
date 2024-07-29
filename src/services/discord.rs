use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use crate::services::database::Database;
use crate::api::youtube::YouTubeClient;

struct Handler {
    database: Arc<Database>,
    youtube_client: Arc<YouTubeClient>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "subscribers" => {
                    let streamers = self.database.get_streamers().unwrap_or_else(|_| vec![]);
                    if streamers.is_empty() {
                        "No subscribers found.".to_string()
                    } else {
                        streamers.iter()
                            .map(|s| format!("[{}](https://youtube.com/@{}) - {}", s.name, s.name, s.channel_id))
                            .collect::<Vec<_>>()
                            .join("\n")
                    }
                },
                "add_subscriber" => {
                    let options = &command.data.options;
                    if let Some(name_option) = options.get(0) {
                        let name = name_option.value.as_ref().unwrap().as_str().unwrap();
                        match self.youtube_client.get_channel_id_by_name(name).await {
                            Ok(channel_id) => {
                                match self.database.add_streamer(name, &channel_id) {
                                    Ok(_) => format!("Successfully added subscriber: {} ({}). Their live streams will now be recorded.", name, channel_id),
                                    Err(e) => format!("Failed to add subscriber to database: {}", e),
                                }
                            },
                            Err(e) => format!("Failed to get channel ID: {}", e),
                        }
                    } else {
                        "Invalid command usage. Please provide the YouTube channel name.".to_string()
                    }
                },
                "remove_subscriber" => {
                    let options = &command.data.options;
                    if let Some(name_option) = options.get(0) {
                        let name = name_option.value.as_ref().unwrap().as_str().unwrap();
                        match self.youtube_client.get_channel_id_by_name(name).await {
                            Ok(channel_id) => {
                                match self.database.remove_streamer(&channel_id) {
                                    Ok(true) => format!("Successfully removed subscriber: {} ({}). Their live streams will no longer be recorded.", name, channel_id),
                                    Ok(false) => format!("No subscriber could be removed with the name: {}. Please check the name and try again.", name),
                                    Err(e) => format!("Failed to remove subscriber from database: {}", e),
                                }
                            },
                            Err(e) => format!("Failed to get channel ID: {}", e),
                        }
                    } else {
                        "Invalid command usage. Please provide the YouTube channel name.".to_string()
                    }
                },
                _ => "Not implemented".to_string(),
            };

            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            }).await {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("subscribers").description("List all subscribed YouTube channels")
                })
                .create_application_command(|command| {
                    command.name("add_subscriber")
                        .description("Add a new YouTube channel to watch for live streams to record")
                        .create_option(|option| {
                            option
                                .name("name")
                                .description("The name of the YouTube channel")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command.name("remove_subscriber")
                        .description("Remove a YouTube channel from the watch list")
                        .create_option(|option| {
                            option
                                .name("name")
                                .description("The name of the YouTube channel to remove")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                })
        })
        .await;

        match commands {
            Ok(_) => println!("Successfully created global slash commands"),
            Err(why) => println!("Failed to create global slash commands: {:?}", why),
        }
    }
}

pub async fn run_discord_bot(database: Arc<Database>, youtube_client: Arc<YouTubeClient>) {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { database, youtube_client })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}