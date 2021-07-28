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
                //println!("cud not create image from: {:?}", e);
                None
            }
        })
        .filter(|img| img.is_manageable())
        .collect();
    //.for_each(|img| { process_image(img).unwrap_or(()) });
    for image in images {
        let counter1 = process_image(image)?;
        counter = counter + counter1;
    }
    Ok(counter)
}
pub fn process_image(mut img: Image) -> Result<Counter, CustomErr> {
    println!("processing image {:?}", ImageInfo::new(&img));
    let old_path = img.path.to_owned();
    let mut ext_changed = false;
    let mut modified_flag = false;
    let counter = Counter::new();

    match img.img_format() {
        None => return Err(custom_err::from("unknown image format")),
        Some(ImageFormat::Jpeg) => {} //proceed
        Some(_) => {
            img = img.convert_format()?;
            modified_flag = true;
            ext_changed = true;
        }
    };

    unsafe {
        let working_profile_type = match ALLOW_ADOBE_RGB {
            true => IccpType::AdobeRGB,
            false => IccpType::IECSrgb,
        };
    }

    match img.iccp() {
        Some(profile) => {
            match &profile.profile_type() {
                IccpType::AdobeRGB => {} //if ALLOW_ADOBE_RGB => do nothing, else convert
                IccpType::IECSrgb => {}  //do nothing
                IccpType::OtherSrgb | IccpType::Other | IccpType::GoogleSrgb => {
                    img = img.set_IECsRGB_profile()?;
                    modified_flag = true;
                    //img.convert_iccp_and_save(&profile.data(), &Iccp::default())?;
                    //return Ok(counter);
                } //convert
            }
        }
        None => {
            img = img.set_IECsRGB_profile()?;
            modified_flag = true;
        } //
    }
    // match iccp {
    //     Some(profile) => {
    //         let desc = profile.info(InfoType::Description, Locale::none());
    //         match desc {
    //             Some(s) if s.to_lowercase().contains("iec") && iccp_size > 3100 => {
    //                 println!("image: {:?}, profile: {:?}, profile_size: {:?}",img.path, s, iccp_size);
    //                 counter.total_iecsrgb_profiles += 1;
    //             },
    //             _=> {
    //                 println!("image: {:?}, profile: {:?}, profile_size: {:?}",img.path, desc.unwrap(), iccp_size);
    //                 counter.total_srgb_profiles += 1;
    //             }
    //         };
    //     },
    //     None => {
    //         counter.total_no_emb_profiles += 1;
    //     }
    // }
    //match on

    //img.save()?;
    //img.convert_we_to_jpeg()?.save()?;
    if modified_flag == true {
        img.save()?;
    }
    if ext_changed {
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
