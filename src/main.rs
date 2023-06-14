use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rayon::prelude::*;
use rusttype::{Font as RusttypeFont, Scale};
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{exit, Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ascii_video_maker <input_video> <frame_rate>");
        return;
    }

    let input_file = &args[1];
    let frame_rate = &args[2];

    let output_folder = "./images/";

    if let Err(err) = fs::create_dir_all(output_folder) {
        eprintln!("Failed to create output folder: {}", err);
        exit(1);
    }

    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file)
        .arg("-r")
        .arg(frame_rate)
        .arg(format!("{}%04d.png", output_folder))
        .output();

    match output {
        Ok(_) => {
            println!("Frames extracted successfully!");
            process_images(output_folder);

            let input_folder = "./img/";
            let output_file = format!("ascii_video_{}", input_file);

            let mut ffmpeg_command = Command::new("ffmpeg")
                .arg("-framerate")
                .arg(frame_rate)
                .arg("-i")
                .arg(format!("{}%04d.png", input_folder))
                .arg("-i")
                .arg(input_file)
                .arg("-c:v")
                .arg("libx264")
                .arg("-filter_complex")
                .arg("scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2")
                .arg("-r")
                .arg(frame_rate)
                .arg("-pix_fmt")
                .arg("yuv420p")
                .arg("-c:a")
                .arg("copy")
                .arg("-threads")
                .arg("6")
                .arg(output_file)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to execute FFmpeg command");

            if let Some(stdout) = ffmpeg_command.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        println!("{}", line);
                    }
                }
            }

            let output = ffmpeg_command.wait();

            match output {
                Ok(_) => {
                    println!("Video created successfully!");
                    remove_directory(input_folder, "./img folder deleted");
                    remove_directory(output_folder, "./images folder deleted");
                }
                Err(err) => {
                    eprintln!("Failed to execute FFmpeg command: {}", err);
                    exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to execute FFmpeg command: {}", err);
            exit(1);
        }
    }
}

fn process_images(folder_path: &str) {
    let folder = std::path::Path::new(folder_path);
    let entries = fs::read_dir(folder)
        .expect("Failed to read folder")
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    let font_data: &[u8] = include_bytes!("Inconsolata-ExtraBold.ttf");
    let font = RusttypeFont::try_from_bytes(font_data).expect("Error loading font");

    // adding progress report
    println!("Total images: {}", entries.len() as u64);

    entries.par_iter().for_each(|entry| {
        if entry.is_file() {
            if let Ok(image) = image::open(entry) {
                let image = image.resize(240, 135, image::imageops::FilterType::Lanczos3);
                let processed_image = get_image(image, &font);

                let file_name = entry.file_name().unwrap();
                let output_folder = std::path::Path::new("./img/");
                let output_path = output_folder.join(format!("{}", file_name.to_string_lossy()));

                if let Err(err) = fs::create_dir_all(output_folder) {
                    eprintln!("Failed to create output folder: {}", err);
                    exit(1);
                }

                if let Err(err) = processed_image.save(output_path) {
                    eprintln!("Can't save the processed image: {}", err);
                }
            }
        }
    });
}

fn get_image(img: DynamicImage, font: &RusttypeFont) -> DynamicImage {
    let (width, height) = img.dimensions();
    let canvas_width = width * 20;
    let canvas_height = height * 20;
    let background_color = Rgba([0, 0, 0, 255]);
    let mut image =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_pixel(canvas_width, canvas_height, background_color);

    for (x, y, pixel) in img.pixels() {
        let mut intent = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
        if pixel[3] == 0 {
            intent = 0;
        }
        let char = get_str_ascii(intent);
        let img_x = x as i32 * 20;
        let img_y = y as i32 * 20;
        draw_text_mut(
            &mut image,
            Rgba([pixel[0], pixel[1], pixel[2], 255]),
            img_x,
            img_y,
            Scale { x: 36.0, y: 28.0 },
            &font,
            &char.to_string(),
        );
    }

    DynamicImage::ImageRgba8(image)
}

fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 3;
    let ascii: [&str; 86] = [
        " ", "\"", "'", "*", "+", ",", "-", ".", ":", ";", "0", "1", "2", "3", "4", "5", "6", "7",
        "8", "9", "=", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
        "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G",
        "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y",
        "Z", "#", "(", ")", "$", "%", "&", "{", "|", "}", "~", "@", "?", ">",
    ];
    ascii[index as usize]
}

fn remove_directory(directory_path: &str, success_message: &str) {
    match fs::remove_dir_all(directory_path) {
        Ok(_) => println!("{}", success_message),
        Err(e) => println!("Couldn't delete the {} folder: {}", directory_path, e),
    }
}
