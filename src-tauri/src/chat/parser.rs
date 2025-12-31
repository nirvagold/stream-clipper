// Chat file parsing - supports Twitch JSON, YouTube JSON, Generic TXT

use super::types::{ChatFormat, ChatInfo, ChatMessage};
use regex::Regex;
use serde_json::Value;
use std::fs;

/// Detect chat format from file content
pub fn detect_format(path: &str) -> Result<ChatFormat, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Try to parse as JSON first
    if let Ok(json) = serde_json::from_str::<Value>(&content) {
        // Check for Twitch format (has "comments" array)
        if json.get("comments").is_some() {
            return Ok(ChatFormat::TwitchJson);
        }
        // Check for YouTube format (array of objects with "replayChatItemAction")
        if json.is_array() {
            if let Some(first) = json.as_array().and_then(|a| a.first()) {
                if first.get("replayChatItemAction").is_some() {
                    return Ok(ChatFormat::YouTubeJson);
                }
            }
        }
    }

    // Check for generic TXT format: [HH:MM:SS] username: message
    let txt_pattern = Regex::new(r"^\[?\d{1,2}:\d{2}:\d{2}\]?\s+\w+:").unwrap();
    if content.lines().take(10).any(|line| txt_pattern.is_match(line)) {
        return Ok(ChatFormat::GenericTxt);
    }

    Ok(ChatFormat::Unknown)
}

/// Parse chat file and return messages
pub fn parse_chat(path: &str) -> Result<(Vec<ChatMessage>, ChatFormat), String> {
    let format = detect_format(path)?;
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let messages = match format {
        ChatFormat::TwitchJson => parse_twitch_json(&content)?,
        ChatFormat::YouTubeJson => parse_youtube_json(&content)?,
        ChatFormat::GenericTxt => parse_generic_txt(&content)?,
        ChatFormat::Unknown => return Err("Unknown chat format".to_string()),
    };

    Ok((messages, format))
}

/// Get chat file info
pub fn get_chat_info(path: &str) -> Result<ChatInfo, String> {
    let (messages, format) = parse_chat(path)?;

    if messages.is_empty() {
        return Ok(ChatInfo {
            path: path.to_string(),
            format,
            total_messages: 0,
            duration_secs: 0.0,
            avg_rate_per_min: 0.0,
        });
    }

    let first_ts = messages.first().map(|m| m.timestamp_secs).unwrap_or(0.0);
    let last_ts = messages.last().map(|m| m.timestamp_secs).unwrap_or(0.0);
    let duration = last_ts - first_ts;
    let avg_rate = if duration > 0.0 {
        (messages.len() as f32 / duration as f32) * 60.0
    } else {
        0.0
    };

    Ok(ChatInfo {
        path: path.to_string(),
        format,
        total_messages: messages.len() as u32,
        duration_secs: duration,
        avg_rate_per_min: avg_rate,
    })
}

/// Parse Twitch JSON format (from TwitchDownloader)
fn parse_twitch_json(content: &str) -> Result<Vec<ChatMessage>, String> {
    let json: Value =
        serde_json::from_str(content).map_err(|e| format!("Invalid JSON: {}", e))?;

    let comments = json
        .get("comments")
        .and_then(|c| c.as_array())
        .ok_or("No comments array found")?;

    let messages: Vec<ChatMessage> = comments
        .iter()
        .filter_map(|comment| {
            let offset = comment.get("content_offset_seconds")?.as_f64()?;
            let commenter = comment.get("commenter")?;
            let username = commenter
                .get("display_name")
                .or_else(|| commenter.get("name"))
                .and_then(|n| n.as_str())?
                .to_string();
            let message_obj = comment.get("message")?;
            let body = message_obj.get("body").and_then(|b| b.as_str())?.to_string();

            // Check for emotes
            let has_emote = message_obj
                .get("fragments")
                .and_then(|f| f.as_array())
                .map(|frags| frags.iter().any(|f| f.get("emoticon").is_some()))
                .unwrap_or(false);

            Some(ChatMessage {
                timestamp_secs: offset,
                username,
                message: body,
                has_emote,
            })
        })
        .collect();

    Ok(messages)
}

/// Parse YouTube JSON format (from yt-dlp)
fn parse_youtube_json(content: &str) -> Result<Vec<ChatMessage>, String> {
    let json: Value =
        serde_json::from_str(content).map_err(|e| format!("Invalid JSON: {}", e))?;

    let items = json.as_array().ok_or("Expected JSON array")?;

    let messages: Vec<ChatMessage> = items
        .iter()
        .filter_map(|item| {
            let action = item.get("replayChatItemAction")?;
            let video_offset = action
                .get("videoOffsetTimeMsec")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<f64>().ok())?
                / 1000.0;

            let actions = action.get("actions")?.as_array()?;
            let chat_item = actions.first()?.get("addChatItemAction")?.get("item")?;

            // Try different message types
            let renderer = chat_item
                .get("liveChatTextMessageRenderer")
                .or_else(|| chat_item.get("liveChatPaidMessageRenderer"))?;

            let author = renderer
                .get("authorName")
                .and_then(|a| a.get("simpleText"))
                .and_then(|t| t.as_str())?
                .to_string();

            let message_runs = renderer.get("message")?.get("runs")?.as_array()?;
            let message: String = message_runs
                .iter()
                .filter_map(|r| r.get("text").and_then(|t| t.as_str()))
                .collect::<Vec<_>>()
                .join("");

            let has_emote = message_runs
                .iter()
                .any(|r| r.get("emoji").is_some());

            Some(ChatMessage {
                timestamp_secs: video_offset,
                username: author,
                message,
                has_emote,
            })
        })
        .collect();

    Ok(messages)
}

/// Parse generic TXT format: [HH:MM:SS] username: message
fn parse_generic_txt(content: &str) -> Result<Vec<ChatMessage>, String> {
    let pattern = Regex::new(r"^\[?(\d{1,2}):(\d{2}):(\d{2})\]?\s+(\w+):\s*(.*)$").unwrap();

    let messages: Vec<ChatMessage> = content
        .lines()
        .filter_map(|line| {
            let caps = pattern.captures(line)?;
            let hours: f64 = caps.get(1)?.as_str().parse().ok()?;
            let minutes: f64 = caps.get(2)?.as_str().parse().ok()?;
            let seconds: f64 = caps.get(3)?.as_str().parse().ok()?;
            let username = caps.get(4)?.as_str().to_string();
            let message = caps.get(5)?.as_str().to_string();

            let timestamp = hours * 3600.0 + minutes * 60.0 + seconds;

            Some(ChatMessage {
                timestamp_secs: timestamp,
                username,
                message,
                has_emote: false, // Can't detect emotes in plain text
            })
        })
        .collect();

    Ok(messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_generic_txt() {
        let content = "[00:01:30] user1: Hello world\n[00:02:00] user2: POG";
        let messages = parse_generic_txt(content).unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].timestamp_secs, 90.0);
        assert_eq!(messages[0].username, "user1");
    }
}
