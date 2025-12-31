# Changelog

All notable changes to Stream Clipper will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-12-31

### Added

#### Core Features
- 🎵 **Audio Analysis** - RMS-based spike detection for finding loud moments
- 🎤 **Voice Activity Detection** - WebRTC VAD integration for speech detection
- 💬 **Chat Analysis** (Pro) - Parse Twitch JSON, YouTube JSON, and generic TXT formats
- 🎯 **Highlight Scoring** - Weighted scoring with combo detection for overlapping audio/chat
- ✂️ **Video Clipping** - FFmpeg-powered clip extraction with customizable padding

#### User Interface
- 📁 **Drag & Drop Import** - Easy video and chat file import
- 📊 **Timeline Visualization** - Waveform display with highlight markers
- 🎬 **Clip Preview** - Preview clips before exporting
- ⚙️ **Detection Settings** - Adjustable sensitivity and parameters
- 📤 **Export Panel** - Configure output format, resolution, and effects

#### Export Options
- 📹 **MP4 Export** - H.264 video with AAC audio
- 🎞️ **WebM Export** (Pro) - VP9 video with Opus audio
- 📐 **Vertical Crop** (Pro) - 9:16 aspect ratio for TikTok/Reels/Shorts
- ✨ **Fade Effects** (Pro) - Smooth fade in/out transitions
- 💧 **Watermark** - Automatic watermark for free tier

#### License System
- 🔐 **Free Tier** - 5 clips max, 720p, watermark, audio-only detection
- ⭐ **Pro Tier** - Unlimited clips, 4K, no watermark, all features

### Technical
- Built with Tauri 2.0 + Rust backend
- Svelte 5 frontend with reactive stores
- FFmpeg bundled for video processing
- Cross-platform support (Windows ready, macOS/Linux coming)

---

## Upcoming

### [0.2.0] - Planned
- macOS and Linux builds
- Batch processing
- Custom export presets
- Keyboard shortcuts

### [0.3.0] - Future
- Auto-caption with Whisper
- Direct upload to social media
- Game-specific detection
- Smart vertical crop with face detection
