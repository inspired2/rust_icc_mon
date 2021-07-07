#![allow(unused)]

extern crate exif;
use lcms2::*;
use std::fs::DirEntry;
use std::{fs, io};
use std::path::*;
use img_parts::{ImageICC, jpeg};
use super::MANAGEABLE_FILE_EXTENSIONS;

//TODO:
//create custom error struct instead & implement send for it to be able to use multithreading
pub type CustomErr = Box<dyn std::error::Error>;
struct Image {
    path: String,
    profile: Option<Profile>
}
impl Image {
    fn new(img_path: &str) -> Self {
        let profile = match get_iccp_bytes(&img_path) {
            Ok(bytes) => Some(iccp_from_bytes(&bytes)),

            Err(_) => None
        };
        Self {
            path: img_path.to_owned(),
            profile
        }
    }
    fn convert_to(mut self, profile: Profile) -> Result<(), CustomErr> {
        //iterate over pixel transforming every pixel using lcms transform struct
        unimplemented!()
    }
    fn image_bufer() -> Result<Vec<u8>, CustomErr> {
        //use image crate to get pixels iterator of an image
        unimplemented!()
    }
    fn profile_desc(&self) -> Option<IccpInfo> {
        match &self.profile {
            Some(profile) => Some(IccpInfo::new(&profile)),
            _=> None
        }
    }
}
#[derive(Debug)]
struct IccpInfo {
    description: Option<String>,
    model: Option<String>
}

impl IccpInfo {
    fn new(profile: &Profile) -> Self {
        Self {
            description: profile.info(InfoType::Description, Locale::none()),
            model: profile.info(InfoType::Description, Locale::none())
        }
    }
}

pub fn process_dir_inp(dir: &str, _recurse: bool) -> Result<(), CustomErr> {
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
    let images: Vec<DirEntry> = files
        .into_iter()
        .filter(|f| is_check_pending(&f))
        .collect();
    
    images
        .iter()
        .for_each(|entry| { process_file_inp(&entry.path()).unwrap(); });
    Ok(())
}

pub fn process_file_inp(path: &PathBuf) -> Result<(), CustomErr> {
    let image = Image::new(path.to_str().unwrap());
    match image.profile_desc() {
        Some(IccpInfo { description: d, model: m} ) => {
            //TODO:
            //analyze d & m
            //launch image.convert() if needed
            image.convert_to(default_iccp())?;
            Ok(())
        },
        None => {
            image.convert_to(default_iccp())?;
            Ok(())
        }
    }
}

fn get_exif(path: &str) -> Result<exif::Exif, CustomErr> {
    let file = fs::File::open(path)?;
    let mut buf_reader = io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut buf_reader)?;
    Ok(exif)
}

///get raw iccp with img_parts crate
fn get_iccp_bytes(path: &str) -> Result<img_parts::Bytes, CustomErr> {
    let file = fs::read(path)?;
    let icc = jpeg::Jpeg::from_bytes(file.into())?.icc_profile();
    match icc {
        Some(bytes) => Ok(bytes),
        None => Err(CustomErr::from(io::Error::new(io::ErrorKind::Other, "no icc profile found")))
    }
}

fn iccp_from_bytes(bytes: &img_parts::Bytes) -> Profile {
    lcms2::Profile::new_icc(&bytes).unwrap()
}

fn default_iccp() -> Profile {
    lcms2::Profile::new_srgb()
}

fn path_is_dir(input: &str) -> bool {
    Path::new(input).is_dir()
}
fn path_is_symlink(path: &PathBuf) -> bool {
    match fs::symlink_metadata(path) {
        Ok(meta) => meta.file_type().is_symlink(),
        _=> false
    }
}

fn is_check_pending(file: &DirEntry) -> bool {
    println!("{:?}",file);
    match file.path().extension() {
        Some(s) => if MANAGEABLE_FILE_EXTENSIONS.contains(&s.to_str().unwrap_or_default()) {
            return true
        } else { return false }
        _ => false
    }
}