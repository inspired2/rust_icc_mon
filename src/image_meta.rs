use super::MANAGEABLE_FILE_EXTENSIONS;
use image::{EncodableLayout, ImageFormat, ImageOutputFormat};
use img_parts::DynImage;
use img_parts::jpeg::Jpeg;
use lcms2::{InfoType, Locale, Profile};
use qcms;
use rexiv2::{self, Metadata};
use webp;
use std::marker::{Send, Sync};

use std::fs;
use std::path::*;
pub use img_parts::{Bytes, ImageICC};

pub enum ImageType {
    Jpeg,
    Webp,
    Tiff,
    Bmp,
    Other,
}
pub enum IccpType {
    IECSrgb,
    AdobeRGB,
    Other
}
pub struct Iccp {
    data: Profile,
    desc: IccpType,
    len: usize
}
impl Iccp {
    pub fn new(image: &Image) -> Option<Self> {
        let profile_bytes = image.embedded_profile_bytes();
        let desc: IccpType;
        let len: usize;
        let data: Option<Profile>;
        match profile_bytes {
            Some(bytes) => {
                data = Profile::new_icc(&bytes[..]).ok();
                len = bytes.len();    
            },
            _=> return None
        }
        if let Some(profile) = data {
            match profile.info(InfoType::Description, Locale::none()) {
                Some(s) => {
                    let s = s.to_lowercase();
                    if s.contains("iec") && s.contains("srgb") {
                        desc = IccpType::IECSrgb;
                    } else if 
                        s.contains("adobe") && s.contains("rgb") {
                            desc = IccpType::AdobeRGB;
                        }
                     else {
                        desc = IccpType::Other;
                    }
                    Some(Self {
                        desc, len, data: profile
                    })

                },
                _=> return None
            }
        } else {
            return None
        }

    }
    pub fn profile_type(&self) -> &IccpType {
        &self.desc
    }
    pub fn data (self) -> Profile {
        self.data
    }
    pub fn profile_size(&self) -> usize {
        self.len
    }
}

pub type CustomErr = Box<dyn std::error::Error + Send + Sync>;
// trait MyMarker {}
// impl MyMarker for CustomErr {}
// unsafe impl<M: MyMarker + ?Sized> Send for M {}
pub struct Image {
    pub decoded: img_parts::DynImage,
    pub path: Box<PathBuf>,
    //now storing raw undecoded buffer in "bytes" field. 
    //maybe should store enum with concrete image type 
    //& decoded data inside appropriate enum variant?
    pub bytes: img_parts::Bytes,
    //pub decoded: Jpeg
}
unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl Image {
    pub fn new(path: PathBuf) -> Result<Self, CustomErr> {
        let buffer = read_to_buf(&path).unwrap();
        let bytes = Bytes::from(buffer);
        if let Some(decoded) = DynImage::from_bytes(bytes.to_owned()).unwrap() {
        //let decoded = decode_jpeg(bytes.to_owned()).unwrap();
        let out = Self {
            path: Box::new(path),
            bytes,
            decoded
        };
        Ok(out) } else {return Err(CustomErr::from(std::io::Error::new(std::io::ErrorKind::Other, "not an image")))}
    }
    pub fn save(self) -> Result<(), CustomErr> {
        let file = std::fs::File::create(*self.path)?;
        DynImage::encoder(self.decoded).write_to(file)?;
        Ok(())
    }
    pub fn convert_webp_to_jpeg(mut self) -> Result<Image, CustomErr> {
        let dyn_img = 
            webp::Decoder::new(self.bytes.as_bytes())
                .decode().unwrap()
                .to_image();
                
                //.to_vec();
        let mut path = self.path.to_owned();
        path.set_extension("jpeg");
        //std::fs::remove_file(&*path)?;
        let mut bytes = Vec::new();
        println!("dyn image: {:?}, path: {:?}", self.decoded, self.path.file_name().unwrap());
        dyn_img.write_to(&mut bytes,ImageOutputFormat::Jpeg(100))?;
        self = Image {
            decoded: DynImage::Jpeg(Jpeg::from_bytes(Bytes::from(bytes.to_owned()))?),
            path,
            bytes: Bytes::from(bytes)
        };
        Ok(self)
//         self.path.set_extension("jpeg");
//         self.bytes = Bytes::from(buffer);
//         self.decoded = DynImage::Jpeg(Jpeg::from_bytes(self.bytes.clone())?);
//         Ok(self)
    }
}

pub trait Meta {
    fn metadata(&self) -> Option<rexiv2::Metadata>;
    fn embedded_profile_bytes(&self) -> Option<img_parts::Bytes>;
    fn is_manageable(&self) -> bool;
    fn iccp(&self) -> Option<Iccp>;
    fn img_format(&self) -> Option<ImageFormat>;

}
impl Meta for Image {
    fn metadata(&self) -> Option<Metadata> {
        if let Ok(meta) = rexiv2::Metadata::new_from_buffer(&self.bytes) {
            return Some(meta)
        }
        None
    }
    
    fn embedded_profile_bytes(&self) -> Option<img_parts::Bytes> {
        self.decoded.icc_profile()
    }
    fn is_manageable(&self) -> bool {
        match self.path.extension() {
            Some(path) => {
                MANAGEABLE_FILE_EXTENSIONS.contains(&path.to_str().unwrap())
            },
            _=> false
        }
    }
    fn iccp(&self) -> Option<Iccp> {
        Iccp::new(self)
    }
    fn img_format(&self) -> Option<image::ImageFormat> {
        image::guess_format(self.bytes.as_bytes()).ok()
    }
}

fn read_to_buf(path: &PathBuf) -> Result<Vec<u8>, CustomErr> {
    let buffer = fs::read(&**path)?;
    Ok(buffer)
}
// pub fn print_all_exif_tags(meta: &Metadata) {
//     match meta.get_exif_tags() {
//         Ok(tags_vec) => tags_vec.iter().for_each(|tag| println!("tag: {}", tag)),
//         _=>{}

//     }
// }
