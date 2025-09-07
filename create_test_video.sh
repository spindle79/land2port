#!/bin/bash

# Create a short test video for quick benchmarking
# This extracts just the first 10 seconds of your existing video

echo "Creating short test video for benchmarking..."

# Extract first 10 seconds of the goal video
ffmpeg -i video/goal.mp4 -t 10 -c copy video/test_10s.mp4

# Extract first 5 seconds for even faster testing
ffmpeg -i video/goal.mp4 -t 5 -c copy video/test_5s.mp4

# Extract first 2 seconds for micro-benchmarks
ffmpeg -i video/goal.mp4 -t 2 -c copy video/test_2s.mp4

echo "Created test videos:"
echo "  - video/test_10s.mp4 (10 seconds)"
echo "  - video/test_5s.mp4 (5 seconds)" 
echo "  - video/test_2s.mp4 (2 seconds)"
echo ""
echo "Use these for quick benchmarking:"
echo "  ./benchmark_memory.sh video/test_5s.mp4 5"
echo "  ./benchmark_performance.sh video/test_5s.mp4 3"
