# Performance Benchmarking Guide

This guide explains how to benchmark the performance improvements from the cloning optimizations.

## Quick Start

### 1. Memory Usage Benchmarking (Recommended)
```bash
# Test with default video (goal.mp4) for 30 seconds
./benchmark_memory.sh

# Test with custom video and duration
./benchmark_memory.sh "path/to/your/video.mp4" 60
```

### 2. Performance Benchmarking
```bash
# Test with default video, 5 iterations
./benchmark_performance.sh

# Test with custom video and iterations
./benchmark_performance.sh "path/to/your/video.mp4" 10
```

### 3. Rust Criterion Benchmarks
```bash
# Run micro-benchmarks
cargo bench

# View detailed HTML reports
open target/criterion/index.html
```

## What to Look For

### Memory Improvements
- **Maximum resident set size**: Should be lower in optimized version
- **Page reclaims**: Fewer page faults indicate better memory efficiency
- **Average memory usage**: Should be more stable/consistent

### Performance Improvements
- **Elapsed time**: Should be faster in optimized version
- **CPU usage**: May be slightly higher (more efficient processing)
- **Consistency**: Less variance between runs

### Expected Results
Based on the optimizations made:

1. **Memory Usage**: 10-30% reduction in peak memory usage
2. **Processing Speed**: 5-15% improvement in frame processing time
3. **Memory Stability**: More consistent memory usage patterns

## Benchmarking Best Practices

### 1. Use Release Builds
Always benchmark with `--release` builds for accurate performance measurements.

### 2. Multiple Iterations
Run multiple iterations to account for system variance:
```bash
./benchmark_performance.sh "video.mp4" 10
```

### 3. Consistent Test Data
Use the same video file for both branches to ensure fair comparison.

### 4. System State
- Close unnecessary applications
- Ensure consistent system load
- Use the same machine for both tests

### 5. Video Characteristics
Test with different video types:
- High resolution (4K) vs standard (1080p)
- Long vs short videos
- Different frame rates
- Different content types (sports, graphics, etc.)

## Interpreting Results

### Memory Metrics
```bash
# Good improvement example:
Main branch:      Maximum resident set size: 1,234,567 kbytes
Optimized branch: Maximum resident set size: 987,654 kbytes
Improvement: 20% reduction
```

### Performance Metrics
```bash
# Good improvement example:
Main branch:      Average time: 45.2s
Optimized branch: Average time: 38.7s
Improvement: 14.4% faster
```

### Statistical Significance
- Look for consistent improvements across multiple runs
- Consider the magnitude of improvement vs. measurement variance
- Focus on trends rather than single measurements

## Troubleshooting

### Common Issues

1. **No improvement detected**
   - Ensure you're testing with the right video characteristics
   - Check that both branches are built in release mode
   - Verify the optimizations are actually being used

2. **Inconsistent results**
   - Run more iterations
   - Check system load and background processes
   - Ensure consistent test conditions

3. **Build errors**
   - Make sure all dependencies are installed
   - Check that the video file exists and is accessible
   - Verify branch names are correct

### Debug Mode
For debugging, you can run with debug builds:
```bash
cargo build
./target/debug/land2port --help
```

## Advanced Benchmarking

### Custom Metrics
You can add custom metrics by modifying the benchmark scripts to track:
- Frame processing rate (FPS)
- Memory allocation patterns
- CPU cache efficiency
- I/O performance

### Profiling
For deeper analysis, use profiling tools:
```bash
# Install perf tools (Linux)
sudo apt-get install linux-tools-common

# Profile memory usage
valgrind --tool=massif ./target/release/land2port [args]

# Profile CPU usage
perf record ./target/release/land2port [args]
perf report
```

### Continuous Benchmarking
Set up automated benchmarking in CI/CD:
```yaml
# Example GitHub Actions workflow
- name: Run Performance Benchmarks
  run: |
    ./benchmark_performance.sh "test_video.mp4" 3
    # Compare results and fail if performance regresses
```
