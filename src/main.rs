#![feature(backtrace)]
extern crate lettre;

mod args;
mod counter;
mod custom_err;
mod iccp;
mod image_meta;
mod mailer;
mod my_image;
mod process;
mod static_iecsrgb;

use args::*;
use custom_err::*;
use iccp::*;
use image_meta::*;
use my_image::*;
use process::*;
use static_iecsrgb::SRGB_IEC;
use std::env;
use std::io::stdin;
use std::path::Path;

pub static JPEG_QUALITY: u8 = 90;
pub static MANAGEABLE_FILE_EXTENSIONS: [&str; 5] = ["jpg", "tiff", "jpeg", "webp", "tif"];
pub static EMAIL_TO: &str = "astrafotovl@yandex.ru";
pub static EMAIL_FROM: &str = "inspired2@yandex.ru";
pub static mut ALLOW_ADOBE_RGB: bool = false;

fn main() -> Result<(), CustomErr> {
    let args = ArgsInput::new(env::args());
    match args.clone().process() {
        Err(e) => {
            let err_trace = e.source().map(|e| e.backtrace().unwrap());
            println!("an error occured; sending report to {:?}", EMAIL_TO);
            mailer::send_email(format! {"an error occured while processing image\n
            arguments: {:?}. Backtrace: {:?}", args ,&err_trace})?;
        }
        Ok(c) => {
            println!("Processing complete.\nHere are the results: {:#?}", c);
        }
    }
    println!("нажмите любую клавишу, чтобы выйти");
    let buf = &mut String::new();
    loop {
        let _input = stdin().read_line(buf);
        if buf.len() > 0 {break}
    }
    Ok(())
}
