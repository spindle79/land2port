#!/bin/bash

# Quick benchmark script for fast testing
# Tests just the core optimizations without full video processing

echo "=== Quick Benchmark: Cloning Optimizations ==="
echo "This tests the core optimizations without full video processing"
echo ""

# Function to test memory usage of a single frame
test_single_frame() {
    local branch=$1
    local test_name=$2
    
    echo "Testing $test_name on branch: $branch"
    
    # Checkout branch
    git checkout $branch > /dev/null 2>&1
    
    # Build in release mode
    cargo build --release > /dev/null 2>&1
    
    # Create a minimal test that exercises the cloning optimizations
    # We'll create a simple test that processes just a few frames
    
    # Test with very short video (2 seconds) and minimal processing
    local start_time=$(date +%s.%N)
    
    # Use minimal settings for fastest processing
    timeout 30s ./target/release/land2port \
        --source "video/test_2s.mp4" \
        --object "ball" \
        --headless \
        --smooth-duration 0.1 \
        --smooth-percentage 0.05 \
        --object-prob-threshold 0.5 \
        --object-area-threshold 0.005 \
        --output-filepath "quick_test_${branch}.mp4" \
        2>/dev/null
    
    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc 2>/dev/null || echo "0")
    
    echo "  Time: ${duration}s"
    echo "  Status: $([ $? -eq 0 ] && echo "SUCCESS" || echo "TIMEOUT/ERROR")"
    echo ""
}

# Create test videos if they don't exist
if [ ! -f "video/test_2s.mp4" ]; then
    echo "Creating test videos..."
    ./create_test_video.sh > /dev/null 2>&1
fi

# Test both branches
test_single_frame "main" "Original"
test_single_frame "performance/avoid-cloning-optimization" "Optimized"

echo "=== Quick Comparison ==="
echo "If the optimized version is faster, you should see:"
echo "  - Lower processing time"
echo "  - More consistent results"
echo "  - Less memory usage (check with: ps aux | grep land2port)"
echo ""
echo "For detailed memory analysis, run:"
echo "  ./benchmark_memory.sh video/test_5s.mp4 5"
