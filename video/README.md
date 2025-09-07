# Video Input Folder

This folder contains input videos for processing with Land2Port.

## Usage

Place your input video files in this directory and reference them using the `--source` parameter:

```bash
# Process a video from this folder
cargo run --release -- --source ./video/your_video.mp4

# Process with additional options
cargo run --release -- \
  --source ./video/your_video.mp4 \
  --object face \
  --headless \
  --add-captions
```

## Supported Formats

Land2Port supports common video formats including:
- MP4
- MOV
- AVI
- MKV
- WebM

## File Organization

You can organize your videos in subdirectories within this folder:

```
video/
├── interviews/
│   ├── interview_1.mp4
│   └── interview_2.mp4
├── sports/
│   ├── football_match.mp4
│   └── basketball_game.mp4
└── presentations/
    └── demo_video.mp4
```

Then reference them with the full path:

```bash
cargo run --release -- --source ./video/interviews/interview_1.mp4
```

## Default File

The default input file is `./video/video1.mp4`. If you want to use this as your default, simply rename your video file to `video1.mp4` and place it in this directory.

## Notes

- The `video/` folder structure is tracked by git, but video files are ignored
- Large video files should not be committed to the repository
- Only the `README.md` file in this folder is tracked by git
- Use this folder for testing and development
- For production use, specify the full path to your video files
