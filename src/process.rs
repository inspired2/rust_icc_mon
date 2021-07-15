use std::{fs::{self, DirEntry}, io, path::{Path, PathBuf}};

use super::*;
use image_meta::*;
use img_parts::ImageEXIF;
use lcms2::{InfoType, Locale, Profile};
use counter::Counter;


pub fn process_dir_inp(dir: &str, recurse: bool) -> Result<Counter, CustomErr> {

    if !path_is_dir(dir) {
        return Err(CustomErr::from(io::Error::new(io::ErrorKind::Other, "input is not a dir")))
    }
    let mut files: Vec<DirEntry> = Vec::new();
    for entry in Path::new(dir).read_dir()? {
        match entry {
            Ok(e) => {
                if recurse && path_is_dir(e.path().to_str().unwrap()) {
                    return process_dir_inp(e.path().to_str().unwrap(), true);
                }

                if (&e).path().is_file() && !fs::symlink_metadata(e.path())?.file_type().is_symlink() {
                    files.push(e)
                }
            },
            Err(_) => {}
        }
    }
    let images: Vec<JpegImage> = files
        .into_iter()
        .filter_map(|e| 
            if let Ok(image) = image_meta::JpegImage::new(e.path()) {
                Some(image)
            } else {
                println!("cud not create image from: {:?}", e);
                None
            })
        .filter(|img| img.is_manageable())
        .collect();
        //.for_each(|img| { process_image(img).unwrap_or(()) });
    let mut counter = Counter::new();
        for image in images {
            let counter1 = process_image(image)?;
            counter = counter + counter1;
    }
    Ok(counter)
}
pub fn process_image(img: JpegImage) -> Result<Counter, CustomErr> {
    //check if there's any info about iccp
    let mut counter = Counter::new();
    let iccp_size = img.embedded_profile_bytes().unwrap_or_default().len();
    let iccp = img.iccp();
    match &iccp {
        Some(profile) => {
            let desc = profile.info(InfoType::Description, Locale::none());
            match desc {
                Some(s) if s.to_lowercase().contains("iec") => {
                    counter.total_iecsrgb_profiles += 1;
                },
                _=> counter.total_srgb_profiles += 1
            };
        },
        None => {
            counter.total_no_emb_profiles += 1;
        }
    }
    let img_meta = img.metadata().unwrap();
    println!("name: {:?}", img.path.file_name().unwrap_or_default());
    println!("exif: {:?}", img_meta.has_exif());
    println!("emb icc.len(): {:?}, icc desc: {:?}", img.embedded_profile_bytes().unwrap_or_default().len(), iccp);
    println!("***********************************");
    //analyze metadata & convert if needed
    //img.save()?;
    Ok(counter)
}
pub fn process_file_inp(path: PathBuf) -> Result<(), CustomErr> {
    let image = image_meta::JpegImage::new(path)?;
    process_image(image)?;
    Ok(())
}

fn path_is_dir(input: &str) -> bool {
    Path::new(input).is_dir()
}
// fn path_is_symlink(path: &PathBuf) -> bool {
//     match fs::symlink_metadata(path) {
//         Ok(meta) => meta.file_type().is_symlink(),
//         _=> false
//     }
// }
