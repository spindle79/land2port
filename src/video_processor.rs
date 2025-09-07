use crate::cli::Args;
use crate::config;
use crate::crop;
use crate::progress::VideoProgressTracker;
use crate::video_processor_utils;
use anyhow::Result;
use ndarray::Axis;
use usls::{
    Annotator, Config, DType, DataLoader, Style, Viewer, perf,
    models::{Clip, YOLO},
};

/// Base trait for video processors that handle cropping with different smoothing strategies
pub trait VideoProcessor {
    /// Processes a video with cropping and smoothing
    fn process_video(&mut self, args: &Args, processed_video: &str) -> Result<()> {
        let config = config::build_config(&args)?;
        let mut model = YOLO::new(config.commit()?)?;

        let clip_config = Config::mobileclip_s0()
            .with_dtype_all(DType::Fp16)
            .with_device_all(args.device.parse()?)
            .commit()?;
        let mut clip_model = Clip::new(clip_config)?;
        let texts = vec![
            "a realistic image",
            "a photographic image",
            "an image of a person",
            "an image of multiple people",
            "an image of a text document",
            "an image of graphics",
            "an image of figures",
            "an image of diagrams",
        ];
        let feats_text = clip_model.encode_texts(&texts)?.norm(1)?;

        // build dataloader
        let data_loader = DataLoader::new(&args.source)?
            .with_batch(model.batch() as _)
            .build()?;

        // Convert smooth_duration from seconds to frames
        let frame_rate = data_loader.frame_rate();
        let smooth_duration_frames = if args.smooth_duration > 0.0 {
            (args.smooth_duration * frame_rate as f32).round() as usize
        } else {
            0
        };

        // Create progress tracker (we'll estimate total as we go)
        println!("Video info: {:.1} FPS", frame_rate);
        
        // Create progress tracker
        let mut progress_tracker = VideoProgressTracker::new_unknown_total(
            frame_rate as f64,
            &format!("{} detection", args.object)
        );

        let mut viewer = Viewer::default()
            .with_window_scale(0.5)
            .with_fps(frame_rate as usize)
            .with_saveout(processed_video.to_string());

        // build annotator
        let annotator = Annotator::default()
            .with_obb_style(Style::obb().with_draw_fill(true))
            .with_hbb_style(
                Style::hbb()
                    .with_draw_fill(true)
                    .with_palette(&usls::Color::palette_coco_80()),
            );

        // Common video processing logic
        for images in data_loader {
            if viewer.is_window_exist() && !viewer.is_window_open() {
                break;
            }

            // Handle key events and delay
            if let Some(key) = viewer.wait_key(1) {
                if key == usls::Key::Escape {
                    break;
                }
            }

            let detections = model.forward(&images)?;

            for (image, detection) in images.iter().zip(detections.iter()) {
                // Update progress for each frame
                progress_tracker.update_frame();
                // Calculate crop areas based on the detection results first
                let objects = video_processor_utils::extract_objects_above_threshold(
                    detection,
                    &args.object,
                    args.object_prob_threshold,
                    args.object_area_threshold,
                    image.width() as f32,
                    image.height() as f32,
                );

                let is_graphic = if objects.len() == 0 && args.keep_graphic {
                    let feats_image = clip_model.encode_images(&[image.clone()])?.norm(1)?;

                    // use image to query texts
                    let matrix = (feats_image * 100.).dot2(&feats_text)?.softmax(1)?;
                    let mut id = 0;
                    let mut score = 0.0;
                    for (_i, row) in matrix.axis_iter(Axis(0)).enumerate() {
                        if let Some((item_id, &item_score)) = row
                            .iter()
                            .enumerate()
                            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                        {
                            id = item_id;
                            score = item_score;
                            video_processor_utils::debug_println(format_args!("({}) <=> ({})", item_score * 100.0, &texts[item_id]));
                        }
                    }
                    id > 3 && score > args.graphic_threshold
                } else {
                    false
                };

                let latest_crop = crop::calculate_crop_area(
                    args.use_stack_crop,
                    is_graphic,
                    image.width() as f32,
                    image.height() as f32,
                    &objects,
                )?;

                // Print debug information
                self.print_debug_info(&objects, &latest_crop, is_graphic);

                // Create img only when needed (avoid unnecessary clone)
                if smooth_duration_frames > 0 {
                    let img = if !args.headless {
                        annotator.annotate(image, detection)?
                    } else {
                        image.clone()
                    };
                    self.process_frame_with_smoothing(
                        &img,
                        &latest_crop,
                        &objects,
                        args,
                        &mut viewer,
                        smooth_duration_frames,
                    )?;
                } else {
                    let img = if !args.headless {
                        annotator.annotate(image, detection)?
                    } else {
                        image.clone()
                    };
                    video_processor_utils::process_and_display_crop(
                        &img,
                        &latest_crop,
                        &mut viewer,
                        args.headless,
                    )?;
                }
            }
        }
        self.finalize_processing(args, &mut viewer)?;
        viewer.finalize_video()?;

        // Finish progress tracking
        progress_tracker.finish();

        perf(false);

        Ok(())
    }

    /// Processes a single frame with smoothing logic (to be implemented by concrete processors)
    fn process_frame_with_smoothing(
        &mut self,
        img: &usls::Image,
        latest_crop: &crop::CropResult,
        objects: &[&usls::Hbb],
        args: &Args,
        viewer: &mut Viewer,
        smooth_duration_frames: usize,
    ) -> Result<()>;

    /// Finalizes processing by handling any remaining frames in history (to be implemented by concrete processors)
    fn finalize_processing(&mut self, _args: &Args, _viewer: &mut Viewer) -> Result<()> {
        // Default implementation does nothing
        Ok(())
    }

    /// Prints debug information (can be overridden by concrete processors)
    fn print_debug_info(
        &self,
        objects: &[&usls::Hbb],
        latest_crop: &crop::CropResult,
        is_graphic: bool,
    ) {
        video_processor_utils::print_default_debug_info(objects, latest_crop, is_graphic);
    }
}
