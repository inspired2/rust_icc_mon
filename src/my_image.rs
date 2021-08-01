use super::*;
use img_parts::jpeg::Jpeg;
use img_parts::DynImage;
use lcms2::{Intent, PixelFormat, Profile};
use rgb::*;
use std::path::PathBuf;

pub struct Image {
    pub decoded: img_parts::DynImage,
    pub path: Box<PathBuf>,
    pub bytes: img_parts::Bytes,
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl Image {
    pub fn from_raw(buffer: Vec<u8>, path: PathBuf) -> Result<Self, CustomErr> {
        let bytes = Bytes::from(buffer);
        let dynamic = img_parts::DynImage::from_bytes(bytes.to_owned())?;
        match dynamic {
            Some(img) => Ok(Self {
                decoded: img,
                bytes,
                path: Box::new(path),
            }),
            None => Err(custom_err::from("not an image")),
        }
    }

    pub fn read(path: PathBuf) -> Result<Self, CustomErr> {
        let buffer = read_to_buf(&path).unwrap();
        let bytes = Bytes::from(buffer);
        if let Some(decoded) = DynImage::from_bytes(bytes.to_owned()).unwrap() {
            //let decoded = decode_jpeg(bytes.to_owned()).unwrap();
            let out = Self {
                path: Box::new(path),
                bytes,
                decoded,
            };
            Ok(out)
        } else {
            return Err(custom_err::from("not an image"));
        }
    }

    pub fn save(self) -> Result<(), CustomErr> {
        let file = std::fs::File::create(*self.path)?;
        DynImage::encoder(self.decoded).write_to(file)?;
        Ok(())
    }
    //need separate func to handle webp
    pub fn convert_webp_to_jpeg(mut self) -> Result<Image, CustomErr> {
        let dyn_img = webp::Decoder::new(self.bytes.as_bytes())
            .decode()
            .unwrap()
            .to_image();

        let mut path = self.path.to_owned();
        path.set_extension("jpeg");
        let mut bytes = Vec::new();
        println!(
            "dyn image: {:?}, path: {:?}",
            self.decoded,
            self.path.file_name().unwrap()
        );
        dyn_img.write_to(&mut bytes, ImageOutputFormat::Jpeg(JPEG_QUALITY))?;
        self = Image {
            decoded: DynImage::Jpeg(Jpeg::from_bytes(Bytes::from(bytes.to_owned()))?),
            path,
            bytes: Bytes::from(bytes),
        };
        Ok(self)
    }
    pub fn convert_format(self) -> Result<Image, CustomErr> {
        if let Some(ImageFormat::WebP) = self.img_format() {
            return Ok(self.convert_webp_to_jpeg()?);
        }
        Ok(self.convert_to_jpeg()?)
    }
    pub fn convert_to_jpeg(self) -> Result<Image, CustomErr> {
        let image_dynamic = //safe to unwrap here as we checked for none earlier from calling fn
        image::load_from_memory_with_format(&self.bytes, self.img_format().unwrap())?;
        let mut write_buffer: Vec<u8> = Vec::with_capacity(self.bytes.len());
        image_dynamic.write_to(&mut write_buffer, ImageOutputFormat::Jpeg(JPEG_QUALITY))?;
        Ok(Image::from_raw(write_buffer, *self.path)?)
    }
    pub fn convert_iccp(self, from: &Profile, into: &Profile) -> Result<Image, CustomErr> {
        //by now self is a JPEG image!
        let mut dynamic =
            image::load_from_memory_with_format(&self.bytes, image::ImageFormat::Jpeg)?;
        let pixels = dynamic.as_mut_rgb8().unwrap().as_mut().as_rgb_mut();
        let t = lcms2::Transform::new(
            from,
            PixelFormat::RGB_8,
            into,
            PixelFormat::RGB_8,
            Intent::RelativeColorimetric,
        )?;
        t.transform_in_place(pixels);
        let mut image_bytes = Vec::new();
        dynamic.write_to(&mut image_bytes, ImageFormat::Jpeg)?;
        let img = Image::from_raw(image_bytes, *self.path)?; //save_with_format(*self.path, ImageFormat::Jpeg)?;
        Ok(img)
    }
    #[allow(non_snake_case)]
    pub fn set_IECsRGB_profile(mut self) -> Result<Image, CustomErr> {
        self.decoded
            .set_icc_profile(Some(Bytes::from_static(&SRGB_IEC)));
        Ok(self)
    }
}

#[derive(Debug)]
pub struct ImageInfo {
    embedded_profile: Option<IccpType>,
    file_name: String,
}
impl ImageInfo {
    pub fn new(img: &Image) -> Self {
        let embedded_profile = match img.iccp() {
            Some(p) => Some(p.profile_type()),
            None => None,
        };
        let file_name = img.path.file_name().unwrap().to_str().unwrap().to_string();
        Self {
            embedded_profile,
            file_name,
        }
    }
}
