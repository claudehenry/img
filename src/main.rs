use std::error::Error;

use clap::{Parser, ValueEnum};
use image::{DynamicImage, ImageBuffer, RgbImage, RgbaImage};

#[derive(ValueEnum, Clone, Debug)]
#[value(rename_all = "lowercase")]
enum FileFormat {
    Png,
    Jpg,
}

impl FileFormat {
    fn output_file(&self, name: &str) -> String {
        match self {
            Self::Png => format!("{}.png", name),
            Self::Jpg => todo!(),
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
#[value(rename_all = "lowercase")]
enum PixelFormat {
    Rgb,

    Rgba,
}

#[derive(Parser, Debug)]
struct Input {
    // todo: optional
    // todo: check if exists & warn & allow force create
    // todo: incremental nameing when default out.png, out-1.png ...
    name: String,

    /// horizontal image dimension
    width: u32,

    /// vertical image dimension, leave blank for square images
    height: Option<u32>,

    /// The desired pixel format
    #[arg(value_enum, short, long = "pixel", default_value = "rgba")]
    pixel_format: PixelFormat,

    #[arg(value_enum, short, long = "format", default_value = "png")]
    file_format: FileFormat,
}

fn main() {
    match run() {
        Ok(name) => println!("wrote: {}", name),
        Err(error) => println!("{}", error),
    }
}

fn run() -> Result<String, Box<dyn Error>> {
    let args = Input::try_parse()?;

    let (width, height) = (args.width, args.height.unwrap_or(args.width));
    let filename = args.file_format.output_file(&args.name);

    match args.pixel_format {
        PixelFormat::Rgb => DynamicImage::new_rgb8(width, height),
        PixelFormat::Rgba => DynamicImage::new_rgba8(width, height),
    }
    .save(&filename)?;

    Ok(filename)
}
