// extern crate lettre;
// extern crate lettre_email;
#![feature(backtrace)]
extern crate lettre;

mod args;
mod counter;
mod custom_err;
mod iccp;
mod image_meta;
mod my_image;
mod process;
mod static_iecsrgb;
mod mailer;

use args::*;
use custom_err::*;
use iccp::*;
use image_meta::*;
use my_image::*;
use process::*;
use std::env;
use std::path::Path;
pub use static_iecsrgb::*;
// pub use lettre;
// pub use lettre_email;

//use mailer;

pub static JPEG_QUALITY: u8 = 90;
pub static MANAGEABLE_FILE_EXTENSIONS: [&str; 4] = ["jpg", "tiff", "jpeg", "webp"];
pub static mut ALLOW_ADOBE_RGB: bool = false;
pub static EMAIL_TO: &str = "astrafotovl@yandex.ru";
pub static EMAIL_FROM: &str = "inspired2@yandex.ru";


fn main() -> Result<(), CustomErr> {
    let args = ArgsInput::new(env::args());
    if let Err(e) = args.clone().process() {
        let kind = e.source();
        let trace = match kind {
            Some(e) => Some(e.backtrace().unwrap()),
            None => None
        };
        println!("an error occured; sending report to {:?}", EMAIL_TO);
        mailer::send_email(format!{"an error occured while processing image\n
                                    arguments: {:?}. Backtrace: {:?}", args ,&trace})?;
    }
    Ok(())
}
