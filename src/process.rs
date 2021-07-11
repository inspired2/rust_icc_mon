#![allow(unused)]
use image::*;
extern crate exif;
use lcms2::*;
use std::fs::DirEntry;
use std::iter::FromIterator;
use std::{fs, io};
use std::path::*;
use img_parts::{ImageICC, jpeg};
use super::MANAGEABLE_FILE_EXTENSIONS;


// impl FromIterator<&[u8]> for Vec<RGB> {
//     fn from_iter<I: Iterator<Item = RGB>> (iter: I) {
//     let output = Vec::new();
//     for i in iter {
//         output.push(i);
//     }
//     output
//     }
// }
//TODO:
//create custom error struct instead & implement send for it to be able to use multithreading
pub type CustomErr = Box<dyn std::error::Error>;
#[derive(Copy, Clone, Debug)]
struct RGB {
    r: u8,
    g: u8,
    b: u8
}

impl RGB {
    fn new(slice: &[u8]) -> Self {
        Self {
            r: slice[0],
            g: slice[1],
            b: slice[2]
        }
    }
}
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
    fn convert_to(mut self, output_profile: Profile) -> Result<(), CustomErr> {
        let input_profile = match self.profile {
            Some(profile) => profile,
            None => {
                self.profile = Some(default_iccp());
                self.profile.take().unwrap()
            }
        };
        let mut file = image::io::Reader::open(&self.path)?.with_guessed_format()?.decode()?;
        // let mut input = file
        // //.decode()?
        // .as_bytes()
        // //.to_vec()
        // .chunks(3)
        // .map(|rgb| {
        //     //let pix =rgb.to_vec();
        //     RGB::new(rgb)
        // })
        // .collect::<Vec<RGB>>();
        // //iterate over pixel transforming every pixel using lcms transform struct
        // let t //: Transform<Vec<u8>, Vec<u8>, GlobalContext, AllowCache> 
        // = Transform::new(&input_profile, PixelFormat::RGB_8, &input_profile, PixelFormat::RGB_8, Intent::RelativeColorimetric)?;
        // t.transform_in_place(&mut input);
        file.save(&self.path)?;
        Ok(())
    }
    fn image_buffer(path: &str) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, CustomErr> {
        //use image crate to get pixels iterator of an image
        //read image to 
        let mut image = image::io::Reader::open(path)?.decode()?;
        let buf = image.as_mut_rgb8().unwrap().to_owned();
        Ok(buf)
        //unimplemented!()
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
            model: profile.info(InfoType::Model, Locale::none())
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
            println!("file: {:?}, desc: {:?}, model: {:?}", path, d, m);
            //TODO:
            //analyze d & m
            //launch image.convert() if needed
            image.convert_to(default_iccp())?;
            Ok(())
        },
        None => {
            println!("no profile found");
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