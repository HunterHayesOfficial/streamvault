#!/bin/bash

echo "Welcome to the StreamVault installation script!"

# Check if Rust is installed
if ! command -v rustc &> /dev/null
then
    echo "Rust is not installed. Would you like to install it? (y/n)"
    read -r install_rust
    if [[ $install_rust =~ ^[Yy]$ ]]
    then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        source $HOME/.cargo/env
    else
        echo "Rust is required to run StreamVault. Please install it manually and run this script again."
        exit 1
    fi
fi

# Install yt-dlp
echo "Installing yt-dlp..."
sudo curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
sudo chmod a+rx /usr/local/bin/yt-dlp

# Install chat_downloader
echo "Installing chat_downloader..."
pip install chat-downloader

# Set up .env file
echo "Setting up .env file..."
echo "Please enter your YouTube API key:"
read -r api_key
echo "Enter the check interval in seconds (default: 60):"
read -r check_interval
check_interval=${check_interval:-60}
echo "Enter the database path (default: $HOME/streamvault.db):"
read -r db_path
db_path=${db_path:-$HOME/streamvault.db}
# ask question do you want to use as discord bot?
echo "Do you want to use StreamVault as a Discord bot? (y/n)"
read -r discord_bot
if [[ $discord_bot =~ ^[Yy]$ ]]
then
    echo "Please enter your Discord bot token:"
    read -r discord_token
fi

if [[ $discord_bot =~ ^[Yy]$ ]]
then
cat > .env << EOL
YOUTUBE_API_KEY=$api_key
CHECK_INTERVAL=$check_interval
DATABASE_PATH=$db_path
DISCORD_TOKEN=$discord_token
EOL
else
cat > .env << EOL
YOUTUBE_API_KEY=$api_key
CHECK_INTERVAL=$check_interval
DATABASE_PATH=$db_path
EOL
fi

echo ".env file created successfully!"

# Create necessary directories
mkdir -p "$(dirname "$db_path")"
mkdir -p "$HOME/streamvault"

echo "Installation complete! You can now build and run StreamVault using:"
echo "cargo build --release"
echo "cargo run --release"

echo "Thank you for installing StreamVault!"