# CawbirdX

Lightweight GTK4 Twitter client for Linux.

## Features

- View Twitter timeline
- Compose tweets
- Search tweets
- Secure credential storage using system keyring
- Local caching for improved performance

## Building

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install libgtk-4-dev libadwaita-1-dev libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev pkg-config

# Build
cargo build --release
```

## Running

```bash
cargo run --release
```

## Configuration

On first run, you'll be prompted to enter your RapidAPI credentials:

- **API Key**: Your RapidAPI key
- **API Host**: The RapidAPI host (e.g., `twitter241.p.rapidapi.com`)

## Credentials

Get your credentials from [RapidAPI](https://rapidapi.com/).

## License

MIT
# CawbirdX
