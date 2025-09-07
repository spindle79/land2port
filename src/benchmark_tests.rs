// Quick unit-style benchmarks for testing cloning optimizations
// These run in seconds, not minutes

#[cfg(test)]
mod benchmark_tests {
    use std::time::Instant;
    use crate::crop::{CropResult, CropArea};
    use usls::Hbb;

    // Mock data for testing
    fn create_test_crop_result() -> CropResult {
        CropResult::Single(CropArea::new(100.0, 100.0, 200.0, 200.0))
    }

    fn create_test_objects() -> Vec<Hbb> {
        vec![
            Hbb::from_cxcywh(150.0, 150.0, 50.0, 50.0),
            Hbb::from_cxcywh(200.0, 200.0, 60.0, 60.0),
            Hbb::from_cxcywh(250.0, 250.0, 70.0, 70.0),
        ]
    }

    #[test]
    fn benchmark_crop_cloning() {
        let crop_result = create_test_crop_result();
        let iterations = 10000;

        // Test cloning performance
        let start = Instant::now();
        for _ in 0..iterations {
            let _cloned = crop_result.clone();
        }
        let clone_duration = start.elapsed();

        // Test reference usage
        let start = Instant::now();
        for _ in 0..iterations {
            let _reference = &crop_result;
        }
        let reference_duration = start.elapsed();

        println!("Crop cloning benchmark ({} iterations):", iterations);
        println!("  Clone time: {:?}", clone_duration);
        println!("  Reference time: {:?}", reference_duration);
        println!("  Speedup: {:.2}x", 
            clone_duration.as_nanos() as f64 / reference_duration.as_nanos() as f64);

        // Reference should be significantly faster
        assert!(reference_duration < clone_duration);
    }

    #[test]
    fn benchmark_crop_calculation() {
        let objects = create_test_objects();
        let objects_slice: Vec<&Hbb> = objects.iter().collect();
        let iterations = 1000;

        let start = Instant::now();
        for _ in 0..iterations {
            let _result = crate::crop::calculate_crop_area(
                false, false, 1920.0, 1080.0, &objects_slice
            );
        }
        let duration = start.elapsed();

        println!("Crop calculation benchmark ({} iterations):", iterations);
        println!("  Total time: {:?}", duration);
        println!("  Average per calculation: {:?}", duration / iterations);

        // Should complete in reasonable time
        assert!(duration.as_millis() < 1000); // Less than 1 second
    }

    #[test]
    fn benchmark_object_processing() {
        let objects = create_test_objects();
        let iterations = 10000;

        // Test with cloning
        let start = Instant::now();
        for _ in 0..iterations {
            let cloned_objects: Vec<Hbb> = objects.iter().map(|o| o.clone()).collect();
            let _sum: f32 = cloned_objects.iter().map(|o| o.width() + o.height()).sum();
        }
        let clone_duration = start.elapsed();

        // Test with references
        let start = Instant::now();
        for _ in 0..iterations {
            let _sum: f32 = objects.iter().map(|o| o.width() + o.height()).sum();
        }
        let reference_duration = start.elapsed();

        println!("Object processing benchmark ({} iterations):", iterations);
        println!("  Clone time: {:?}", clone_duration);
        println!("  Reference time: {:?}", reference_duration);
        println!("  Speedup: {:.2}x", 
            clone_duration.as_nanos() as f64 / reference_duration.as_nanos() as f64);

        // Reference should be faster
        assert!(reference_duration < clone_duration);
    }
}
