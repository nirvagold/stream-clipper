// Formatting utilities

/**
 * Format seconds to duration string (e.g., "1:23:45" or "23:45")
 */
export function formatDuration(secs: number): string {
  const totalSecs = Math.floor(secs);
  const hours = Math.floor(totalSecs / 3600);
  const minutes = Math.floor((totalSecs % 3600) / 60);
  const seconds = totalSecs % 60;

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  }
  return `${minutes}:${seconds.toString().padStart(2, '0')}`;
}

/**
 * Format seconds to timestamp string (e.g., "01h23m45s")
 */
export function formatTimestamp(secs: number): string {
  const totalSecs = Math.floor(secs);
  const hours = Math.floor(totalSecs / 3600);
  const minutes = Math.floor((totalSecs % 3600) / 60);
  const seconds = totalSecs % 60;

  if (hours > 0) {
    return `${hours.toString().padStart(2, '0')}h${minutes.toString().padStart(2, '0')}m${seconds.toString().padStart(2, '0')}s`;
  }
  return `${minutes.toString().padStart(2, '0')}m${seconds.toString().padStart(2, '0')}s`;
}

/**
 * Format bytes to human readable size
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${units[i]}`;
}

/**
 * Format number with commas
 */
export function formatNumber(num: number): string {
  return num.toLocaleString();
}

/**
 * Format score (0-100) with color class
 */
export function getScoreClass(score: number): string {
  if (score >= 80) return 'score-high';
  if (score >= 50) return 'score-medium';
  return 'score-low';
}

/**
 * Get highlight type color
 */
export function getHighlightColor(type: 'Audio' | 'Chat' | 'Combo'): string {
  switch (type) {
    case 'Audio': return '#ef4444'; // red
    case 'Chat': return '#eab308';  // yellow
    case 'Combo': return '#a855f7'; // purple
    default: return '#6b7280';      // gray
  }
}

/**
 * Get highlight type emoji
 */
export function getHighlightEmoji(type: 'Audio' | 'Chat' | 'Combo'): string {
  switch (type) {
    case 'Audio': return '🔴';
    case 'Chat': return '🟡';
    case 'Combo': return '🟣';
    default: return '⚪';
  }
}
