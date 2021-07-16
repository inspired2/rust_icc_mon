mod image_meta;
mod counter;
mod process;
mod args;

use process::*;
use image_meta::*;
use std::path::Path;
use std::env;
use args::*;


pub static MANAGEABLE_FILE_EXTENSIONS: [&str; 4] = ["jpg", "tiff", "jpeg", "webp"];

fn main() -> Result<(), CustomErr> {
    let args = ArgsInput::new(env::args());
    args.process()?;
    Ok(())
}