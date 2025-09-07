#!/bin/bash

# Simple performance benchmarking script
# Usage: ./benchmark_performance.sh [video_file] [iterations]

VIDEO_FILE=${1:-"video/goal.mp4"}
ITERATIONS=${2:-5}

echo "=== Performance Benchmarking for Cloning Optimizations ==="
echo "Video: $VIDEO_FILE"
echo "Iterations: $ITERATIONS"
echo ""

# Function to run performance test
run_performance_test() {
    local branch=$1
    local results_file=$2
    
    echo "Testing branch: $branch"
    
    # Checkout the branch
    git checkout $branch
    
    # Build in release mode
    cargo build --release
    
    # Run multiple iterations
    echo "Running $ITERATIONS iterations..."
    local total_time=0
    local times=()
    
    for i in $(seq 1 $ITERATIONS); do
        echo "  Iteration $i/$ITERATIONS"
        
        # Time the execution
        local start_time=$(date +%s.%N)
        
        ./target/release/land2port \
            --source "$VIDEO_FILE" \
            --object "ball" \
            --headless \
            --smooth-duration 0.5 \
            --smooth-percentage 0.1 \
            --object-prob-threshold 0.3 \
            --object-area-threshold 0.01 \
            --output-filepath "perf_test_${branch}_${i}.mp4" \
            > /dev/null 2>&1
        
        local end_time=$(date +%s.%N)
        local duration=$(echo "$end_time - $start_time" | bc)
        
        times+=($duration)
        total_time=$(echo "$total_time + $duration" | bc)
        
        echo "    Time: ${duration}s"
    done
    
    # Calculate statistics
    local avg_time=$(echo "scale=3; $total_time / $ITERATIONS" | bc)
    
    # Calculate min/max
    local min_time=${times[0]}
    local max_time=${times[0]}
    for time in "${times[@]}"; do
        if (( $(echo "$time < $min_time" | bc -l) )); then
            min_time=$time
        fi
        if (( $(echo "$time > $max_time" | bc -l) )); then
            max_time=$time
        fi
    done
    
    # Save results
    echo "Branch: $branch" > "$results_file"
    echo "Average time: ${avg_time}s" >> "$results_file"
    echo "Min time: ${min_time}s" >> "$results_file"
    echo "Max time: ${max_time}s" >> "$results_file"
    echo "Total time: ${total_time}s" >> "$results_file"
    echo "Iterations: $ITERATIONS" >> "$results_file"
    echo "Times: ${times[*]}" >> "$results_file"
    
    echo "  Average: ${avg_time}s"
    echo "  Min: ${min_time}s"
    echo "  Max: ${max_time}s"
    echo "---"
}

# Create results directory
mkdir -p benchmark_results
cd benchmark_results

# Test original branch
run_performance_test "main" "performance_main.txt"

# Test optimized branch
run_performance_test "performance/avoid-cloning-optimization" "performance_optimized.txt"

# Compare results
echo "=== PERFORMANCE COMPARISON ==="
echo ""
echo "Main branch results:"
cat performance_main.txt
echo ""
echo "Optimized branch results:"
cat performance_optimized.txt

echo ""
echo "=== IMPROVEMENT ANALYSIS ==="

# Extract average times for comparison
main_avg=$(grep "Average time" performance_main.txt | cut -d' ' -f3 | sed 's/s//')
opt_avg=$(grep "Average time" performance_optimized.txt | cut -d' ' -f3 | sed 's/s//')

if [ -n "$main_avg" ] && [ -n "$opt_avg" ]; then
    improvement=$(echo "scale=2; ($main_avg - $opt_avg) / $main_avg * 100" | bc)
    echo "Performance improvement: ${improvement}%"
    
    if (( $(echo "$improvement > 0" | bc -l) )); then
        echo "✅ Optimization improved performance by ${improvement}%"
    else
        echo "❌ Optimization decreased performance by ${improvement}%"
    fi
fi

cd ..
