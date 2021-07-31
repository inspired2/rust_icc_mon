#![allow(unused)]
use super::CustomErr;
use super::*;
use counter::Counter;
use image_meta::*;
use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

pub fn process_dir_inp(dir: &str, recurse: bool) -> Result<Counter, CustomErr> {
    let mut counter = Counter::new();
    if !path_is_dir(dir) {
        return Err(custom_err::from("input is not a dir"))
    }
    let mut files: Vec<DirEntry> = Vec::new();
    for entry in Path::new(dir).read_dir()? {
        match entry {
            Ok(e) => {
                if recurse && path_is_dir(e.path().to_str().unwrap()) {
                    counter = counter + process_dir_inp(e.path().to_str().unwrap(), true).unwrap();
                }

                if (&e).path().is_file()
                    && !fs::symlink_metadata(e.path())?.file_type().is_symlink()
                {
                    files.push(e)
                }
            }
            Err(_) => {}
        }
    }
    let images: Vec<Image> = files
        .into_iter()
        .filter_map(|e| {
            if let Ok(image) = Image::read(e.path()) {
                Some(image)
            } else {
                None
            }
        })
        .filter(|img| img.is_manageable())
        .collect();

    for image in images {
        let counter1 = process_image(image)?;
        counter = counter + counter1;
    }
    Ok(counter)
}
pub fn process_image(mut img: Image) -> Result<Counter, CustomErr> {
    println!("processing image {:?}", ImageInfo::new(&img));
    let old_path = img.path.to_owned();
    let mut extension_changed = false;
    let mut modified_flag = false;
    let mut counter = Counter::new();

    match img.img_format() {
        //if jpeg => no conversion needed
        Some(ImageFormat::Jpeg) => {}
        Some(_) => {
            img = img.convert_format()?;
            modified_flag = true;
            extension_changed = true;
        }
        None => return Err(custom_err::from("unknown image format")),
    };

    match img.iccp() {
        Some(profile) => {
            match &profile.profile_type() {
                //only QSS37 series & above can handle AdobeRGB correctly
                IccpType::AdobeRGB => {counter.adobe_rgb += 1;} //if ALLOW_ADOBE_RGB => do nothing, else convert
                IccpType::IECSrgb => {counter.iec_srgb += 1;} 
                IccpType::Other => {counter.other += 1;}//todo convert iccp
                //in this case need to set IEC_sRGB instead because 
                //those can't be interpreted correctly by the Noritsu QSS software:
                IccpType::OtherSrgb | IccpType::GoogleSrgb => {
                    counter.other_srgb += 1;
                    img = img.set_IECsRGB_profile()?;
                    modified_flag = true;
                }
            }
        }
        //if no profile present presume that profile is IECsRGB 
        None => {
            counter.no_profile += 1;
            img = img.set_IECsRGB_profile()?;
            modified_flag = true;
        } 
    }

    if modified_flag == true {
        img.save()?;
    }
    if extension_changed {
        fs::remove_file(*old_path)?;
    }
    Ok(counter)
}
pub fn process_file_inp(path: PathBuf) -> Result<(), CustomErr> {
    let image = Image::read(path)?;
    process_image(image)?;
    Ok(())
}

fn path_is_dir(input: &str) -> bool {
    Path::new(input).is_dir()
}
