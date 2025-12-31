# Changelog

All notable changes to Stream Clipper will be documented in this file.

## [0.1.0] - 2024-12-30

### Added
- Initial release
- Audio spike detection with configurable sensitivity
- Chat activity detection (Pro) for Twitch, YouTube, and generic TXT formats
- Combo detection when audio and chat spike together
- Waveform timeline visualization with highlight markers
- Video clipping with FFmpeg
- Clip preview in theater mode
- Clip timing editor (start/end/duration)
- Export settings: format, resolution, vertical crop, fade effects
- Free tier: 5 clips, 720p, watermark, audio-only
- Pro tier: unlimited clips, 4K, no watermark, chat detection
- License activation system
- Settings persistence
- Cross-platform support (Windows, macOS, Linux)

### Technical
- Tauri 2.0 + Rust backend
- Svelte 5 + TypeScript frontend
- FFmpeg bundled for video processing
- Percentile-based threshold for better spike detection
