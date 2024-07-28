# StreamVault

StreamVault is a Rust application that records YouTube live streams and downloads chat logs for specified channels.

## Features

- Monitors YouTube channels for live streams
- Records live streams automatically
- Downloads chat logs for each recorded stream
- Supports multiple channels simultaneously
- Configurable check interval for live stream detection

## Prerequisites

- Rust (latest stable version)
- yt-dlp
- chat_downloader
- YouTube Data API key

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/streamvault.git
   cd streamvault
   ```

2. Run the installation script:
   ```
   ./install.sh
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Configuration

StreamVault uses a `.env` file for configuration:

```
YOUTUBE_API_KEY=your_youtube_api_key_here
CHECK_INTERVAL=60
DATABASE_PATH=/path/to/your/database/streamvault.db
```

## Usage

1. Run StreamVault:
   ```
   cargo run --release
   ```

2. Follow the prompts to add YouTube channels for monitoring.

3. StreamVault will monitor the added channels and record when they go live.

4. Recorded videos and chat logs are saved in `~/streamvault/[channel_name]/` directory.

## Future Improvements

- Discord integration
- Web interface for watching archived streams with chat replay
- Content management system
- Multi-platform support
- Custom archive path

## Contributing

If you wish to contribute to this project, feel free to submit a pull request.

## License

This project is open source and available under the [MIT License](LICENSE).