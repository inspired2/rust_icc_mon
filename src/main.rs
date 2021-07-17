mod args;
mod counter;
mod image_meta;
mod process;
mod iccp;
mod my_image;


use iccp::*;
use args::*;
use image_meta::*;
use process::*;
use std::env;
use std::path::Path;
use my_image::*;

pub type CustomErr = Box<dyn std::error::Error + Send + Sync>;

pub static mut ALLOW_ADOBE_RGB: bool = false;
pub static MANAGEABLE_FILE_EXTENSIONS: [&str; 4] = ["jpg", "tiff", "jpeg", "webp"];

fn main() -> Result<(), CustomErr> {
    let args = ArgsInput::new(env::args());
    args.process()?;
    Ok(())
}
