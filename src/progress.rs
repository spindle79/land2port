use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;

/// Progress tracker for video processing operations
pub struct VideoProgressTracker {
    progress_bar: ProgressBar,
    start_time: Instant,
    total_frames: Option<u64>,
    frame_rate: f64,
    processed_frames: u64,
}

impl VideoProgressTracker {
    /// Creates a new progress tracker with known total frames
    pub fn new(total_frames: u64, frame_rate: f64, operation_name: &str) -> Self {
        let progress_bar = ProgressBar::new(total_frames);
        
        // Set up the progress bar style with time and frame information
        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} frames ({percent}%) | {msg}")
            .unwrap()
            .progress_chars("#>-");
        
        progress_bar.set_style(style);
        progress_bar.set_message(format!("Processing {}", operation_name));
        
        Self {
            progress_bar,
            start_time: Instant::now(),
            total_frames: Some(total_frames),
            frame_rate,
            processed_frames: 0,
        }
    }

    /// Creates a new progress tracker without known total frames (will estimate)
    pub fn new_unknown_total(frame_rate: f64, operation_name: &str) -> Self {
        let progress_bar = ProgressBar::new_spinner();
        
        // Set up the progress bar style for unknown total
        let style = ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {pos} frames | {msg}")
            .unwrap();
        
        progress_bar.set_style(style);
        progress_bar.set_message(format!("Processing {}", operation_name));
        
        Self {
            progress_bar,
            start_time: Instant::now(),
            total_frames: None,
            frame_rate,
            processed_frames: 0,
        }
    }

    /// Updates the progress by one frame
    pub fn update_frame(&mut self) {
        self.processed_frames += 1;
        self.progress_bar.inc(1);
        
        // Update message with current time and ETA
        let current_time = self.get_current_time();
        let eta = self.get_eta();
        let msg = format!("{} | ETA: {}", current_time, eta);
        self.progress_bar.set_message(msg);
    }

    /// Updates the progress by a specific number of frames
    pub fn update_frames(&mut self, frames: u64) {
        self.processed_frames += frames;
        self.progress_bar.inc(frames);
        
        // Update message with current time and ETA
        let current_time = self.get_current_time();
        let eta = self.get_eta();
        let msg = format!("{} | ETA: {}", current_time, eta);
        self.progress_bar.set_message(msg);
    }

    /// Gets the current time position in the video (h:mm:ss format)
    fn get_current_time(&self) -> String {
        let current_seconds = (self.processed_frames as f64) / self.frame_rate;
        format_duration(current_seconds)
    }

    /// Gets the estimated time remaining (ETA)
    fn get_eta(&self) -> String {
        if self.processed_frames == 0 {
            return "Calculating...".to_string();
        }

        let elapsed = self.start_time.elapsed();
        let frames_per_second = self.processed_frames as f64 / elapsed.as_secs_f64();
        
        if frames_per_second > 0.0 {
            if let Some(total_frames) = self.total_frames {
                let remaining_frames = total_frames - self.processed_frames;
                let remaining_seconds = remaining_frames as f64 / frames_per_second;
                format_duration(remaining_seconds)
            } else {
                // For unknown total, show processing rate
                format!("{:.1} fps", frames_per_second)
            }
        } else {
            "Calculating...".to_string()
        }
    }

    /// Finishes the progress bar
    pub fn finish(&self) {
        let total_time = self.start_time.elapsed();
        let processing_time = format_duration(total_time.as_secs_f64());
        let avg_fps = self.processed_frames as f64 / total_time.as_secs_f64();
        
        let message = if let Some(total_frames) = self.total_frames {
            let total_video_time = format_duration((total_frames as f64) / self.frame_rate);
            format!(
                "Completed! Video: {} | Processing: {} | Avg FPS: {:.1}",
                total_video_time,
                processing_time,
                avg_fps
            )
        } else {
            let processed_video_time = format_duration((self.processed_frames as f64) / self.frame_rate);
            format!(
                "Completed! Processed: {} | Processing: {} | Avg FPS: {:.1}",
                processed_video_time,
                processing_time,
                avg_fps
            )
        };
        
        self.progress_bar.finish_with_message(message);
    }

    /// Gets the total number of frames
    pub fn total_frames(&self) -> Option<u64> {
        self.total_frames
    }

    /// Gets the current number of processed frames
    pub fn processed_frames(&self) -> u64 {
        self.processed_frames
    }

    /// Gets the frame rate
    pub fn frame_rate(&self) -> f64 {
        self.frame_rate
    }
}

/// Formats a duration in seconds to h:mm:ss format
fn format_duration(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0.0), "0:00");
        assert_eq!(format_duration(30.0), "0:30");
        assert_eq!(format_duration(90.0), "1:30");
        assert_eq!(format_duration(3661.0), "1:01:01");
        assert_eq!(format_duration(7200.0), "2:00:00");
    }

    #[test]
    fn test_progress_tracker_creation() {
        let tracker = VideoProgressTracker::new(1000, 30.0, "test video");
        assert_eq!(tracker.total_frames(), 1000);
        assert_eq!(tracker.frame_rate(), 30.0);
        assert_eq!(tracker.processed_frames(), 0);
    }
}
