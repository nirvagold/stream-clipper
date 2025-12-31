<div align="center">

# 🎬 Stream Clipper

**Auto-detect highlight moments from streaming videos and generate clips automatically**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](#installation)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri-FFC131.svg)](https://tauri.app)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Svelte](https://img.shields.io/badge/svelte-5-FF3E00.svg)](https://svelte.dev)

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Tech Stack](#-tech-stack) • [Contributing](#-contributing) • [License](#-license)

</div>

---

## 📖 About

Stream Clipper is a desktop application that automatically detects highlight moments in streaming videos (Twitch, YouTube, etc.) using audio analysis and chat activity detection. Perfect for content creators, streamers, and video editors who want to quickly find and export the best moments from long streams.

### Why Stream Clipper?

- ⏱️ **Save Hours of Editing** - No more scrubbing through hours of footage
- 🎯 **Smart Detection** - Uses audio spikes and chat activity to find highlights
- 🚀 **Fast Processing** - Analyzes videos 10-50x faster than realtime
- 💻 **Offline & Private** - All processing happens locally on your machine
- 🎨 **Modern UI** - Clean, intuitive interface built with Svelte

---

## ✨ Features

### Core Features

| Feature | Free | Pro |
|---------|:----:|:---:|
| Audio spike detection | ✅ | ✅ |
| Voice Activity Detection (VAD) | ✅ | ✅ |
| Timeline visualization | ✅ | ✅ |
| Waveform display | ✅ | ✅ |
| Clip preview | ✅ | ✅ |
| Max clips per video | 5 | Unlimited |
| Export resolution | 720p | Up to 4K |
| Watermark | Yes | No |
| Chat activity detection | ❌ | ✅ |
| Custom keywords | ❌ | ✅ |
| Vertical crop (9:16) | ❌ | ✅ |
| Fade effects | ❌ | ✅ |
| WebM format | ❌ | ✅ |

### Detection Methods

#### 🎵 Audio Analysis
- **RMS Volume Detection** - Detects loud moments (cheering, reactions)
- **Voice Activity Detection** - Uses WebRTC VAD to detect speech patterns
- **Configurable Sensitivity** - Adjust threshold to find more or fewer highlights

#### 💬 Chat Analysis (Pro)
- **Twitch JSON** - Parse chat logs from TwitchDownloader
- **YouTube JSON** - Parse chat logs from yt-dlp
- **Generic TXT** - Support for `[HH:MM:SS] user: message` format
- **Keyword Detection** - Custom keywords like "POG", "CLIP IT", "OMG"
- **Activity Spikes** - Detect moments when chat goes crazy

#### 🎯 Highlight Scoring
- **Combo Detection** - Audio + Chat overlap = higher score
- **Weighted Scoring** - Customize importance of audio vs chat
- **Smart Merging** - Combine adjacent highlights automatically

---

## 📥 Installation

### Windows

1. Download the latest release from [Releases](https://github.com/nirvagold/stream-clipper/releases)
2. Run `Stream Clipper_x.x.x_x64-setup.exe` (NSIS installer) or `Stream Clipper_x.x.x_x64_en-US.msi`
3. Follow the installation wizard
4. Launch Stream Clipper from Start Menu

### macOS

Coming soon! Build from source for now.

### Linux

Coming soon! Build from source for now.

### Build from Source

#### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

#### Steps

```bash
# Clone the repository
git clone https://github.com/nirvagold/stream-clipper.git
cd stream-clipper

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

---

## 🚀 Usage

### Quick Start

1. **Import Video** - Drag & drop or click to browse for your video file
2. **Import Chat** (Optional, Pro) - Add chat log for better detection
3. **Adjust Settings** - Configure audio sensitivity and other options
4. **Analyze** - Click "Analyze Video" to detect highlights
5. **Review** - Preview detected clips in the timeline
6. **Export** - Select clips and export to your desired format

### Supported Formats

#### Video Input
- MP4, MKV, MOV, AVI, WebM, FLV, TS

#### Chat Input (Pro)
- Twitch JSON (from TwitchDownloader)
- YouTube JSON (from yt-dlp)
- Generic TXT (`[HH:MM:SS] username: message`)

#### Export Output
- MP4 (H.264 + AAC)
- WebM (VP9 + Opus) - Pro only

### Tips for Best Results

1. **Lower sensitivity** = More highlights detected
2. **Use chat logs** when available for better accuracy
3. **Adjust padding** to include context before/after highlights
4. **Preview clips** before exporting to verify quality

---

## 🛠️ Tech Stack

Stream Clipper is built with modern, performant technologies:

| Layer | Technology |
|-------|------------|
| **Framework** | [Tauri 2.0](https://tauri.app) - Secure, lightweight desktop apps |
| **Backend** | [Rust](https://www.rust-lang.org) - Fast, memory-safe processing |
| **Frontend** | [Svelte 5](https://svelte.dev) - Reactive UI with minimal overhead |
| **Audio** | [hound](https://crates.io/crates/hound) + [webrtc-vad](https://crates.io/crates/webrtc-vad) |
| **Video** | [FFmpeg](https://ffmpeg.org) - Industry-standard video processing |
| **Styling** | CSS with CSS Variables |

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    STREAM CLIPPER APP                       │
├─────────────────────────────────────────────────────────────┤
│  FRONTEND (Svelte + TypeScript)                             │
│  - Reactive UI Components                                   │
│  - State Management (Svelte stores)                         │
│  - Tauri IPC calls                                          │
├─────────────────────────────────────────────────────────────┤
│  BACKEND (Rust + Tauri)                                     │
│  - Audio Analysis (RMS + VAD)                               │
│  - Chat Parsing (Twitch/YouTube/TXT)                        │
│  - Highlight Scoring & Merging                              │
│  - Video Clipping (FFmpeg wrapper)                          │
│  - License Validation                                       │
├─────────────────────────────────────────────────────────────┤
│  EXTERNAL                                                   │
│  - FFmpeg (bundled)                                         │
└─────────────────────────────────────────────────────────────┘
```

---

## 🤝 Contributing

We welcome contributions from the community! Here's how you can help:

### Ways to Contribute

- 🐛 **Report Bugs** - Open an issue with detailed reproduction steps
- 💡 **Suggest Features** - Share your ideas in discussions
- 📝 **Improve Docs** - Help make documentation clearer
- 🔧 **Submit PRs** - Fix bugs or implement features

### Development Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/stream-clipper.git
   ```
3. Create a feature branch:
   ```bash
   git checkout -b feature/amazing-feature
   ```
4. Make your changes
5. Run tests and linting:
   ```bash
   npm run check
   cd src-tauri && cargo clippy
   ```
6. Commit your changes:
   ```bash
   git commit -m "feat: add amazing feature"
   ```
7. Push to your fork:
   ```bash
   git push origin feature/amazing-feature
   ```
8. Open a Pull Request

### Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

### Code Style

- **Rust**: Follow `rustfmt` and `clippy` recommendations
- **TypeScript/Svelte**: Follow ESLint and Prettier configurations
- **Commits**: Use conventional commit messages

---

## 📋 Roadmap

### v0.2.0 (Planned)
- [ ] macOS and Linux builds
- [ ] Batch processing multiple videos
- [ ] Custom export presets
- [ ] Keyboard shortcuts

### v0.3.0 (Future)
- [ ] Auto-caption with Whisper
- [ ] Direct upload to YouTube/TikTok
- [ ] Game-specific detection (kill feed OCR)
- [ ] Smart vertical crop with face detection

---

## ❓ FAQ

<details>
<summary><b>What video formats are supported?</b></summary>

Stream Clipper supports most common video formats including MP4, MKV, MOV, AVI, WebM, FLV, and TS. Any format that FFmpeg can decode should work.
</details>

<details>
<summary><b>How does the audio detection work?</b></summary>

The app extracts audio from your video, splits it into small chunks, and calculates the volume (RMS) of each chunk. It then identifies moments where the volume exceeds a threshold based on the baseline. Voice Activity Detection (VAD) is also used to detect speech patterns.
</details>

<details>
<summary><b>Where can I get chat logs?</b></summary>

- **Twitch**: Use [TwitchDownloader](https://github.com/lay295/TwitchDownloader) to download chat as JSON
- **YouTube**: Use [yt-dlp](https://github.com/yt-dlp/yt-dlp) with `--write-subs` flag
</details>

<details>
<summary><b>Is my data sent anywhere?</b></summary>

No! All processing happens locally on your machine. Stream Clipper does not send any data to external servers. Your videos and chat logs never leave your computer.
</details>

<details>
<summary><b>How do I get Pro features?</b></summary>

Pro features can be unlocked with a license key. Contact us for purchasing options.
</details>

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Tauri](https://tauri.app) - For the amazing desktop framework
- [Svelte](https://svelte.dev) - For the reactive UI framework
- [FFmpeg](https://ffmpeg.org) - For video processing capabilities
- [WebRTC VAD](https://webrtc.org) - For voice activity detection

---

<div align="center">

**Made with ❤️ for content creators**

[⬆ Back to Top](#-stream-clipper)

</div>
