# ASCII Video Maker

This is a command-line tool that extracts frames from a video file and converts them into an ASCII art video. The resulting video is created by overlaying ASCII characters on the frames of the original video.

## Prerequisites

Before using this tool, make sure you have the following dependencies installed:

- [FFmpeg](https://ffmpeg.org/): Required for frame extraction and video creation.

## Usage

```shell
ascii_video_maker <input_video> <frame_rate>
```

- `input_video`: Path to the input video file.
- `frame_rate`: Frame rate of the output video.
- Note: A higher frame rate will result in a longer runtime for processing the video frames.

## How It Works

1. The tool uses FFmpeg to extract frames from the input video file at the specified frame rate.
2. The extracted frames are saved as PNG images in the `./images/` folder.
3. Each image is processed to convert it into ASCII art using grayscale intensity values of pixels.
4. The processed images are saved in the `./img/` folder.
5. Finally, FFmpeg is used again to combine the processed images and the original video, creating an ASCII art video.
6. The resulting video is saved as `ascii_video_<input_video>` in the current directory.

## Installation

1. Clone the repository:

```shell
git clone https://github.com/example/ascii-video-maker.git
```

2. Change into the project directory:

```shell
cd ascii-video-maker
```

3. Build the project:

```shell
cargo build --release
```

4. The executable file will be available in the `target/release/` directory.

## Examples

```shell
# Extract frames from input.mp4 at 24 frames per second and create the ASCII video
./ascii_video_maker input.mp4 24
```

## Credits

This tool utilizes the following libraries:

- [image](https://crates.io/crates/image): For image processing and manipulation.
- [imageproc](https://crates.io/crates/imageproc): For drawing text on images.
- [rayon](https://crates.io/crates/rayon): For parallel processing of images.
- [rusttype](https://crates.io/crates/rusttype): For loading and rendering fonts.

The default font used for ASCII conversion is **Inconsolata-ExtraBold.ttf**.

## License

This project is licensed under the [MIT License](LICENSE).
