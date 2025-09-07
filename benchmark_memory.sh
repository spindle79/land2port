#!/bin/bash

# Memory benchmarking script for cloning optimizations
# Usage: ./benchmark_memory.sh [video_file] [duration_seconds]

VIDEO_FILE=${1:-"video/goal.mp4"}
DURATION=${2:-30}  # Process only first 30 seconds for consistent benchmarking

echo "=== Memory Benchmarking for Cloning Optimizations ==="
echo "Video: $VIDEO_FILE"
echo "Duration: ${DURATION}s"
echo ""

# Function to run benchmark with memory tracking
run_benchmark() {
    local branch=$1
    local output_file=$2
    
    echo "Testing branch: $branch"
    
    # Checkout the branch
    git checkout $branch
    
    # Build in release mode for accurate performance testing
    cargo build --release
    
    # Run with memory profiling
    echo "Running benchmark..."
    /usr/bin/time -v ./target/release/land2port \
        --source "$VIDEO_FILE" \
        --object "ball" \
        --headless \
        --smooth-duration 0.5 \
        --smooth-percentage 0.1 \
        --object-prob-threshold 0.3 \
        --object-area-threshold 0.01 \
        --output-filepath "benchmark_output_${branch}.mp4" \
        2>&1 | tee "$output_file"
    
    echo "Benchmark completed for $branch"
    echo "---"
}

# Create results directory
mkdir -p benchmark_results
cd benchmark_results

# Test original branch (main)
run_benchmark "main" "memory_main.log"

# Test optimized branch
run_benchmark "performance/avoid-cloning-optimization" "memory_optimized.log"

# Compare results
echo "=== MEMORY COMPARISON ==="
echo "Maximum resident set size (peak memory usage):"
echo "Main branch:"
grep "Maximum resident set size" memory_main.log
echo "Optimized branch:"
grep "Maximum resident set size" memory_optimized.log

echo ""
echo "Average memory usage:"
echo "Main branch:"
grep "Average resident set size" memory_main.log
echo "Optimized branch:"
grep "Average resident set size" memory_optimized.log

echo ""
echo "Page faults:"
echo "Main branch:"
grep "Page reclaims" memory_main.log
echo "Optimized branch:"
grep "Page reclaims" memory_optimized.log

echo ""
echo "=== PERFORMANCE COMPARISON ==="
echo "Elapsed time:"
echo "Main branch:"
grep "Elapsed (wall clock) time" memory_main.log
echo "Optimized branch:"
grep "Elapsed (wall clock) time" memory_optimized.log

echo ""
echo "CPU usage:"
echo "Main branch:"
grep "Percent of CPU this job got" memory_main.log
echo "Optimized branch:"
grep "Percent of CPU this job got" memory_optimized.log

cd ..
