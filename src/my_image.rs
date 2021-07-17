use std::path::PathBuf;
use super::*;
use img_parts::DynImage;
use img_parts::jpeg::Jpeg;

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
    pub fn convert(mut self) -> Result<Image, CustomErr> {
        unimplemented!()
    }
    //this is edge case because image crate doesnt decode webp properly,
    //so we use webp crate to decode.
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