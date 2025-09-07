#!/bin/bash

# Quick memory usage test
# Tests memory usage during a very short video processing run

echo "=== Quick Memory Test ==="
echo "Testing memory usage with 2-second video"
echo ""

# Create test video if it doesn't exist
if [ ! -f "video/test_2s.mp4" ]; then
    echo "Creating 2-second test video..."
    ffmpeg -i video/goal.mp4 -t 2 -c copy video/test_2s.mp4 2>/dev/null
fi

test_memory() {
    local branch=$1
    local test_name=$2
    
    echo "Testing $test_name on branch: $branch"
    
    # Checkout branch
    git checkout $branch > /dev/null 2>&1
    
    # Build in release mode
    cargo build --release > /dev/null 2>&1
    
    # Run with memory monitoring
    echo "  Running memory test..."
    
    # Start the process in background and monitor its memory
    ./target/release/land2port \
        --source "video/test_2s.mp4" \
        --object "ball" \
        --headless \
        --smooth-duration 0.1 \
        --smooth-percentage 0.05 \
        --object-prob-threshold 0.5 \
        --object-area-threshold 0.005 \
        --output-filepath "memory_test_${branch}.mp4" &
    
    local pid=$!
    
    # Monitor memory usage
    local max_memory=0
    local samples=0
    
    while kill -0 $pid 2>/dev/null; do
        local memory=$(ps -o rss= -p $pid 2>/dev/null | tr -d ' ')
        if [ -n "$memory" ] && [ "$memory" -gt 0 ]; then
            if [ "$memory" -gt "$max_memory" ]; then
                max_memory=$memory
            fi
            samples=$((samples + 1))
        fi
        sleep 0.1
    done
    
    wait $pid
    local exit_code=$?
    
    echo "  Max memory: ${max_memory}KB"
    echo "  Samples: $samples"
    echo "  Exit code: $exit_code"
    echo ""
    
    # Return max memory for comparison
    echo $max_memory
}

# Test both branches
main_memory=$(test_memory "main" "Original")
opt_memory=$(test_memory "performance/avoid-cloning-optimization" "Optimized")

echo "=== Memory Comparison ==="
echo "Original branch max memory:  ${main_memory}KB"
echo "Optimized branch max memory: ${opt_memory}KB"

if [ -n "$main_memory" ] && [ -n "$opt_memory" ] && [ "$main_memory" -gt 0 ] && [ "$opt_memory" -gt 0 ]; then
    local improvement=$(echo "scale=1; ($main_memory - $opt_memory) * 100 / $main_memory" | bc 2>/dev/null || echo "0")
    echo "Memory improvement: ${improvement}%"
    
    if (( $(echo "$improvement > 0" | bc -l 2>/dev/null || echo "0") )); then
        echo "✅ Memory usage reduced by ${improvement}%"
    else
        echo "❌ Memory usage increased by ${improvement}%"
    fi
fi
