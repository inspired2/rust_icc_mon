mod args;
mod counter;
mod custom_err;
mod iccp;
mod image_meta;
mod my_image;
mod process;
mod static_iecsrgb;

use args::*;
use custom_err::*;
use iccp::*;
use image_meta::*;
use my_image::*;
use process::*;
use std::env;
use std::path::Path;
pub use static_iecsrgb::*;

pub static JPEG_QUALITY: u8 = 90;
pub static MANAGEABLE_FILE_EXTENSIONS: [&str; 4] = ["jpg", "tiff", "jpeg", "webp"];
pub static mut ALLOW_ADOBE_RGB: bool = false;

fn main() -> Result<(), CustomErr> {
    let args = ArgsInput::new(env::args());
    args.process()?;
    Ok(())
}
