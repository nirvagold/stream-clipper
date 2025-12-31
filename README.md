# 🎬 Stream Clipper

Auto-detect highlight moments from your streaming videos and generate clips automatically.

![Stream Clipper](https://img.shields.io/badge/version-0.1.0-blue)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/license-Proprietary-red)

## ✨ Features

### Free Tier
- 🔊 **Audio Spike Detection** - Automatically detect loud/exciting moments
- 📊 **Waveform Timeline** - Visual representation of audio with highlight markers
- 🎥 **Video Clipping** - Export up to 5 clips per video
- 📺 **720p Export** - Standard definition output
- 🎬 **MP4 Format** - Universal video format

### Pro Tier ($15-25 one-time)
- 💬 **Chat Activity Detection** - Detect hype moments from Twitch/YouTube chat
- 🎯 **Combo Detection** - Find moments where audio AND chat spike together
- ♾️ **Unlimited Clips** - No limit on clips per video
- 📱 **Vertical Crop (9:16)** - Perfect for TikTok, Reels, Shorts
- 🎨 **Fade Effects** - Smooth fade in/out transitions
- 🎬 **WebM Support** - Additional format option
- 📺 **Up to 4K Export** - Full resolution support
- 🔑 **Custom Keywords** - Define your own chat trigger words
- ✨ **No Watermark** - Clean exports

## 🖥️ Screenshots

*Coming soon*

## 📥 Installation

### Windows
1. Download `Stream-Clipper_x.x.x_x64-setup.exe` from [Releases](../../releases)
2. Run the installer
3. Launch Stream Clipper from Start Menu

### macOS
1. Download `Stream-Clipper_x.x.x_x64.dmg` from [Releases](../../releases)
2. Open the DMG and drag to Applications
3. Launch from Applications folder

### Linux
1. Download `Stream-Clipper_x.x.x_amd64.AppImage` from [Releases](../../releases)
2. Make executable: `chmod +x Stream-Clipper*.AppImage`
3. Run: `./Stream-Clipper*.AppImage`

## 🚀 Quick Start

1. **Import Video** - Drag & drop your stream recording or click to browse
2. **Import Chat (Pro)** - Optionally add chat log for better detection
3. **Adjust Settings** - Fine-tune audio sensitivity
4. **Analyze** - Click "Analyze Video" to detect highlights
5. **Review Clips** - Preview and select clips to export
6. **Export** - Choose format, resolution, and export!

## 📁 Supported Formats

### Video Input
- MP4, MKV, MOV, AVI, WebM, FLV, TS

### Chat Input (Pro)
- Twitch JSON (from TwitchDownloader)
- YouTube JSON (from yt-dlp)
- Generic TXT format: `[HH:MM:SS] username: message`

### Video Output
- MP4 (H.264 + AAC)
- WebM (VP9 + Opus) - Pro only

## ⚙️ Detection Settings

| Setting | Range | Default | Description |
|---------|-------|---------|-------------|
| Audio Sensitivity | 1.0 - 4.0 | 1.5 | Lower = more highlights |
| Chat Rate Multiplier | 1.0 - 5.0 | 3.0 | Threshold for chat spikes |
| Padding Before | 0 - 10s | 3s | Context before highlight |
| Padding After | 0 - 10s | 2s | Reaction time after |

## 🔧 Development

### Prerequisites
- Node.js 18+
- Rust 1.70+
- FFmpeg (bundled in release)

### Setup
```bash
# Clone repository
git clone https://github.com/yourusername/stream-clipper.git
cd stream-clipper

# Install dependencies
npm install

# Run in development
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Structure
```
stream-clipper/
├── src/                    # Svelte frontend
│   └── lib/
│       ├── components/     # UI components
│       ├── stores/         # State management
│       ├── types/          # TypeScript types
│       └── utils/          # Utilities
├── src-tauri/              # Rust backend
│   └── src/
│       ├── audio/          # Audio analysis
│       ├── chat/           # Chat parsing
│       ├── highlight/      # Scoring logic
│       ├── video/          # FFmpeg wrapper
│       └── commands/       # Tauri IPC
└── static/                 # Static assets
```

## 📄 License

This software is proprietary. Free tier is available for personal use.
Pro license required for commercial use and advanced features.

## 🤝 Support

- 📧 Email: support@streamclipper.app
- 🐛 Issues: [GitHub Issues](../../issues)
- 💬 Discord: *Coming soon*

---

Made with ❤️ for content creators
