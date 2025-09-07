use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use land2port::video_processor::{VideoProcessor, SimpleSmoothingVideoProcessor, BallVideoProcessor};
use land2port::cli::Args;
use land2port::crop::{CropResult, CropArea};
use usls::{Image, Hbb};
use std::path::Path;

// Mock data for benchmarking
fn create_mock_image() -> Image {
    // Create a mock image for testing
    // This would need to be adapted based on your Image struct
    todo!("Implement mock image creation based on your Image struct")
}

fn create_mock_crop_result() -> CropResult {
    CropResult::Single(CropArea::new(100.0, 100.0, 200.0, 200.0))
}

fn create_mock_objects() -> Vec<Hbb> {
    vec![
        Hbb::from_cxcywh(150.0, 150.0, 50.0, 50.0),
        Hbb::from_cxcywh(200.0, 200.0, 60.0, 60.0),
    ]
}

fn benchmark_crop_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_processing");
    
    // Test with different numbers of objects
    for object_count in [1, 3, 5, 10].iter() {
        let objects = create_mock_objects();
        let objects_slice: Vec<&Hbb> = objects.iter().take(*object_count).collect();
        
        group.bench_with_input(
            BenchmarkId::new("calculate_crop_area", object_count),
            &objects_slice,
            |b, objects| {
                b.iter(|| {
                    let result = land2port::crop::calculate_crop_area(
                        black_box(false),
                        black_box(false),
                        black_box(1920.0),
                        black_box(1080.0),
                        black_box(objects),
                    );
                    black_box(result)
                })
            },
        );
    }
    
    group.finish();
}

fn benchmark_image_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("image_processing");
    
    // Benchmark image cloning vs reference usage
    group.bench_function("image_clone", |b| {
        let image = create_mock_image();
        b.iter(|| {
            let cloned = black_box(image.clone());
            black_box(cloned)
        })
    });
    
    group.bench_function("image_reference", |b| {
        let image = create_mock_image();
        b.iter(|| {
            let reference = black_box(&image);
            black_box(reference)
        })
    });
    
    group.finish();
}

fn benchmark_crop_result_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_result_handling");
    
    // Benchmark crop result cloning
    group.bench_function("crop_result_clone", |b| {
        let crop_result = create_mock_crop_result();
        b.iter(|| {
            let cloned = black_box(crop_result.clone());
            black_box(cloned)
        })
    });
    
    // Benchmark crop result reference usage
    group.bench_function("crop_result_reference", |b| {
        let crop_result = create_mock_crop_result();
        b.iter(|| {
            let reference = black_box(&crop_result);
            black_box(reference)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_crop_processing,
    benchmark_image_processing,
    benchmark_crop_result_handling
);
criterion_main!(benches);
