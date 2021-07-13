use std::{fs::{self, DirEntry}, io, path::{Path, PathBuf}};

use super::*;
use image_meta::*;
use img_parts::ImageEXIF;

pub fn process_dir_inp<'a >(dir: &'a str, _recurse: bool) -> Result<(), CustomErr> {
    if !path_is_dir(dir) {
        return Err(CustomErr::from(io::Error::new(io::ErrorKind::Other, "input is not a dir")))
    }
    let mut files: Vec<DirEntry> = Vec::new();
    for entry in Path::new(dir).read_dir()? {
        match entry {
            Ok(e) => {
                if (&e).path().is_file() && !fs::symlink_metadata(e.path())?.file_type().is_symlink() {
                    files.push(e)
                }
            },
            Err(_) => {}
        }
    }
    files
        .into_iter()
        .filter_map(|e| 
            if let Ok(image) = image_meta::Image::new(e.path()) {
                Some(image)
            } else {
                None
            })
        .filter(|img| img.is_manageable())
        .for_each(|img| { process_image(img).unwrap_or(()) });
    
    Ok(())
}
pub fn process_image(img: Image) -> Result<(), CustomErr> {
    //check if there's any info about iccp


    let img_meta = img.metadata().unwrap();
    println!("name: {:?}", img.path.file_name().unwrap_or_default());
    println!("exif: {:?}", img_meta.has_exif());
    println!("exif.len() from img_parts: {:?}", img_parts::jpeg::Jpeg::from_bytes(img.bytes.clone()).unwrap().exif().unwrap_or_default().len());
    println!("emb icc.len(): {:?}", img.jpeg_embedded_profile_bytes().unwrap_or_default().len());
    println!("***********************************");
    //analyze metadata & convert if needed
    Ok(())
}
pub fn process_file_inp(path: PathBuf) -> Result<(), CustomErr> {
    let image = image_meta::Image::new(path)?;
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
