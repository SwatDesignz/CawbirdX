# CawbirdX

<div align="center">

![CawbirdX Logo](https://img.shields.io/badge/CawbirdX-v0.1.0-blue)
![GTK4](https://img.shields.io/badge/GTK4-0.7-orange)
![Rust](https://img.shields.io/badge/Rust-1.94-orange)
![License](https://img.shields.io/badge/License-MIT-green)

*A lightweight, modern GTK4 Twitter client for Linux*

</div>

## 🛠️ Build Status

[![Type checking: Ty](https://img.shields.io/badge/type%20checking-ty-ffcc00.svg?style=for-the-badge)](https://pypi.org/project/ty/)
[![Code style: Ruff](https://img.shields.io/badge/code%20formatting-ruff-f5a623.svg?style=for-the-badge)](https://github.com/astral-sh/ruff)
[![Logging: Loguru](https://img.shields.io/badge/logging-loguru-4ecdc4.svg?style=for-the-badge)](https://github.com/Delgan/loguru)

## 📸 Screenshots

![Timeline](https://via.placeholder.com/800x400?text=Timeline+View)
![Compose](https://via.placeholder.com/800x400?text=Compose+Tweet)
![Settings](https://via.placeholder.com/800x400?text=Settings)

## ✨ Features

- **Timeline Viewing** - Browse your Twitter timeline with real-time updates
- **Tweet Composition** - Compose and post new tweets with character count
- **Search** - Search tweets by keywords and hashtags
- **User Profiles** - View user profiles with follower/following counts
- **Secure Storage** - Credentials stored securely in system keyring
- **Local Caching** - Cached tweets for improved performance and offline viewing
- **Modern UI** - Clean, modern interface built with GTK4 and libadwaita
- **Dark Mode** - Follows system theme automatically

## 🚀 Installation

### Prerequisites

#### System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y \
    libgtk-4-dev \
    libadwaita-1-dev \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    pkg-config \
    build-essential \
    libssl-dev
```

**Fedora/RHEL:**
```bash
sudo dnf install -y \
    gtk4-devel \
    libadwaita-devel \
    gstreamer1-devel \
    gstreamer1-plugins-base-devel \
    pkg-config \
    openssl-devel
```

**Arch Linux:**
```bash
sudo pacman -S --needed \
    gtk4 \
    libadwaita \
    gstreamer \
    pkg-config \
    base-devel
```

#### Rust Toolchain

Install Rust using rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/SwatDesignz/CawbirdX.git
cd CawbirdX

# Build in release mode
cargo build --release

# The binary will be at target/release/cawbirdx
```

### Installing

```bash
# Install to /usr/local/bin
sudo install -m 755 target/release/cawbirdx /usr/local/bin/cawbirdx

# Or run directly from the build directory
./target/release/cawbirdx
```

## 🔧 Configuration

### RapidAPI Credentials

CawbirdX uses the RapidAPI Twitter API. You'll need to:

1. Sign up at [RapidAPI](https://rapidapi.com/)
2. Subscribe to a Twitter API (e.g., Twitter241)
3. Get your API Key and Host

On first run, you'll be prompted to enter:
- **API Key**: Your RapidAPI key (e.g., `RapidAPI key`)
- **API Host**: The RapidAPI host (e.g., `twitter241.p.rapidapi.com`)

Credentials are stored securely in your system keyring.

### Environment Variables

You can also set credentials via environment variables:

```bash
export RAPIDAPI_KEY="your_api_key"
export RAPIDAPI_HOST="twitter241.p.rapidapi.com"
cawbirdx
```

## 📖 Usage

### Basic Usage

```bash
# Run the application
cawbirdx

# Or with cargo
cargo run --release
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Q` | Quit application |
| `Ctrl+R` | Refresh timeline |
| `Ctrl+F` | Focus search bar |
| `Ctrl+N` | New tweet |

### Features

#### Timeline
- View your home timeline
- Refresh with the refresh button
- Scroll to load more tweets

#### Compose Tweets
- Click the compose button (document icon)
- Type your tweet (280 character limit)
- Character count shown in real-time
- Click "Post" to send

#### Search
- Enter search terms in the search bar
- Click the search button or press Enter
- Results appear in the timeline

#### Settings
- Click the settings button (gear icon)
- View application information
- Clear stored credentials

## 🛠️ Development

### Project Structure

```
cawbirdx/
├── src/
│   ├── api/           # Twitter API client
│   ├── auth/          # Credential storage
│   ├── state/         # Local cache
│   ├── ui/            # UI components
│   ├── resources/     # GTK resources
│   ├── app.rs         # Main application
│   └── main.rs        # Entry point
├── Cargo.toml         # Rust dependencies
├── build.rs           # Build script
└── README.md          # This file
```

### Dependencies

**Core Dependencies:**
- `gtk4` (0.7) - GTK4 bindings
- `libadwaita` (0.5) - libadwaita bindings
- `tokio` (1.35) - Async runtime
- `reqwest` (0.11) - HTTP client
- `serde` (1.0) - Serialization
- `keyring` (1.2) - Secure credential storage
- `heed` (0.20) - LMDB database for caching

**Development Dependencies:**
- `glib-build-tools` (0.18) - GTK resource compilation

### Building for Development

```bash
# Debug build with logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Check code
cargo clippy
```

### Creating a Release

```bash
# Build optimized release
cargo build --release

# The binary will be at target/release/cawbirdx
# Size is optimized with LTO and stripping
```

## 🐛 Troubleshooting

### Application won't start

**GTK4 not found:**
```bash
# Install GTK4 development libraries
sudo apt-get install libgtk-4-dev libadwaita-1-dev
```

**Resources not loading:**
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Credentials not saving

**Keyring issues:**
```bash
# Check if keyring is working
# On Linux, ensure you have a keyring backend installed
# (gnome-keyring, kwallet, etc.)
```

### API errors

**Invalid credentials:**
- Check your RapidAPI key and host
- Ensure your RapidAPI subscription is active
- Verify you have API quota remaining

**Rate limiting:**
- RapidAPI has rate limits per subscription
- Wait before making more requests

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📧 Contact

- **Repository:** https://github.com/SwatDesignz/CawbirdX
- **Issues:** https://github.com/SwatDesignz/CawbirdX/issues

## 🙏 Acknowledgments

- GTK4 and libadwaita teams for the excellent GUI framework
- RapidAPI for providing Twitter API access
- The Rust community for amazing tooling

## 📊 Roadmap

- [ ] Direct Twitter API support (OAuth)
- [ ] Image/media upload support
- [ ] Thread viewing
- [ ] Lists support
- [ ] Notifications
- [ ] Direct messages
- [ ] Multiple account support
- [ ] Custom themes
- [ ] Keyboard navigation improvements

---

<div align="center">

Made with ❤️ using Rust and GTK4

</div>
