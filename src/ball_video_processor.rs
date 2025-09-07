use crate::cli::Args;
use crate::crop;
use crate::image::CutDetector;
use crate::video_processor_utils;
use crate::video_processor::VideoProcessor;
use crate::video_processor_utils::predict_current_hbb;
use anyhow::Result;
use usls::{Viewer, Hbb};

/// Video processor that handles cropping with ball-specific logic
pub struct BallVideoProcessor {
    previous_crop: Option<crop::CropResult>,
    most_recent_image: Option<usls::Image>,
    hbb_three_frames_ago: Option<Hbb>,
    hbb_two_frames_ago: Option<Hbb>,
    hbb_last_frame: Option<Hbb>,
    cut_detector: CutDetector,
}

impl BallVideoProcessor {
    /// Creates a new ball video processor
    pub fn new(args: &Args) -> Self {
        Self {
            previous_crop: None,
            most_recent_image: None,
            hbb_three_frames_ago: None,
            hbb_two_frames_ago: None,
            hbb_last_frame: None,
            cut_detector: CutDetector::new(args.cut_similarity, args.cut_start),
        }
    }
}

impl VideoProcessor for BallVideoProcessor {
    /// Processes a single frame with ball-specific smoothing logic
    fn process_frame_with_smoothing(
        &mut self,
        img: &usls::Image,
        latest_crop: &crop::CropResult,
        objects: &[&usls::Hbb],
        args: &Args,
        viewer: &mut Viewer,
        _smooth_duration_frames: usize,
    ) -> Result<()> {
        let current_ball_count = objects.len();
        
        // Determine if there was a cut
        let is_cut = if let Some(ref most_recent) = self.most_recent_image {
            self.cut_detector.is_cut(most_recent, img)?
        } else {
            true
        };

        // Update most_recent_image for next frame (need to clone for storage)
        self.most_recent_image = Some(img.clone());

        // Apply the ball-specific algorithm
        let (crop_result, needs_storage) = if is_cut {
            // If there was a cut, use latest_crop
            video_processor_utils::debug_println(format_args!("Cut detected, using latest ball crop"));
            self.hbb_three_frames_ago = None;
            self.hbb_two_frames_ago = None;
            self.hbb_last_frame = None;
            (latest_crop.clone(), true)
        } else {
            // If no cut, check ball count
            if current_ball_count > 0 {
                if current_ball_count > 1 {
                    // Multiple balls detected - find the highest confidence ball
                    let highest_confidence_ball = objects
                        .iter()
                        .max_by(|a, b| {
                            let conf_a = a.confidence().unwrap_or(0.0);
                            let conf_b = b.confidence().unwrap_or(0.0);
                            conf_a.partial_cmp(&conf_b).unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .unwrap();

                    video_processor_utils::debug_println(format_args!(
                        "No cut, multiple balls detected ({}), using highest confidence ball (confidence: {:.3})",
                        current_ball_count,
                        highest_confidence_ball.confidence().unwrap_or(0.0)
                    ));

                    // Create a new crop from just the highest confidence ball
                    let single_ball_crop = crop::calculate_crop_area(
                        false, // Don't use stack crop for single ball
                        false, // Not graphic mode for ball processing
                        img.width() as f32,
                        img.height() as f32,
                        &[highest_confidence_ball],
                    )?;

                    self.hbb_three_frames_ago = self.hbb_two_frames_ago.take();
                    self.hbb_two_frames_ago = self.hbb_last_frame.take();
                    self.hbb_last_frame = Some(Hbb::from_cxcywh(
                        highest_confidence_ball.cx(),
                        highest_confidence_ball.cy(),
                        highest_confidence_ball.width(),
                        highest_confidence_ball.height(),
                    ));

                    (single_ball_crop, true)
                } else {
                    // Single ball detected, use latest_crop
                    video_processor_utils::debug_println(format_args!("No cut, single ball detected, using latest ball crop"));
                    self.hbb_three_frames_ago = self.hbb_two_frames_ago.take();
                    self.hbb_two_frames_ago = self.hbb_last_frame.take();
                    self.hbb_last_frame = Some(objects[0].clone());
                    (latest_crop.clone(), true)
                }
            } else {
                // If no balls detected, try to predict position or use previous crop
                if let (Some(three_frames_ago), Some(two_frames_ago), Some(last_frame)) = (&self.hbb_three_frames_ago, &self.hbb_two_frames_ago, &self.hbb_last_frame) {
                    let current_hbb = predict_current_hbb(three_frames_ago, two_frames_ago, last_frame, img.width() as f32, img.height() as f32);
                    let current_crop = crop::calculate_crop_area(
                        false, // Don't use stack crop for single ball
                        false, // Not graphic mode for ball processing
                        img.width() as f32,
                        img.height() as f32,
                        &[&current_hbb],
                    )?;
                    self.hbb_three_frames_ago = self.hbb_two_frames_ago.take();
                    self.hbb_two_frames_ago = self.hbb_last_frame.take();
                    self.hbb_last_frame = Some(current_hbb);
                    (current_crop, true)
                } else {
                    // Not enough history for prediction, use previous crop
                    self.hbb_three_frames_ago = self.hbb_two_frames_ago.take();
                    self.hbb_two_frames_ago = self.hbb_last_frame.take();
                    self.hbb_last_frame = None;
                    if let Some(prev_crop) = &self.previous_crop {
                        video_processor_utils::debug_println(format_args!("No cut, no balls detected, insufficient history, using previous ball crop"));
                        (prev_crop.clone(), false) // Don't need to store, already stored
                    } else {
                        video_processor_utils::debug_println(format_args!("No cut, no balls detected, insufficient history, no previous crop, using latest crop"));
                        (latest_crop.clone(), true)
                    }
                }
            }
        };

        // Update previous crop only if needed (avoid double clone)
        if needs_storage {
            self.previous_crop = Some(crop_result.clone());
        }

        // Process and display the chosen crop
        video_processor_utils::process_and_display_crop(img, &crop_result, viewer, args.headless)?;
        Ok(())
    }

    /// Override debug info to include ball-specific information
    fn print_debug_info(&self, objects: &[&usls::Hbb], latest_crop: &crop::CropResult, is_graphic: bool) {
        video_processor_utils::print_default_debug_info(objects, latest_crop, is_graphic);
        video_processor_utils::debug_println(format_args!("previous_crop: {:?}", self.previous_crop));
        video_processor_utils::debug_println(format_args!("hbb_three_frames_ago: {:?}", self.hbb_three_frames_ago));
        video_processor_utils::debug_println(format_args!("hbb_two_frames_ago: {:?}", self.hbb_two_frames_ago));
        video_processor_utils::debug_println(format_args!("hbb_last_frame: {:?}", self.hbb_last_frame));
    }
} 