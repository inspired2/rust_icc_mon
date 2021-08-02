#![allow(dead_code)]
use super::*;
use lcms2::*;

#[derive(Clone, Copy, Debug)]
pub enum IccpType {
    IECsRGB,
    AdobeRGB,
    GoogleSrgb,
    OtherSrgb,
    Other,
}

pub struct Iccp {
    pub data: Profile,
    desc: IccpType,
    len: usize,
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
            }
            _ => return None,
        }

        if let Some(profile) = data {
            match profile.info(InfoType::Description, Locale::none()) {
                Some(s) => {
                    let s = s.to_lowercase();
                    if s.contains("iec") && s.contains("srgb") && len > 3100 {
                        desc = IccpType::IECsRGB;
                    } else if s.contains("adobe") && s.contains("rgb") {
                        desc = IccpType::AdobeRGB;
                    } else if s.contains("srgb") && s.contains("google") {
                        desc = IccpType::GoogleSrgb;
                    } else if s.contains("srgb") {
                        desc = IccpType::OtherSrgb;
                    } else {
                        desc = IccpType::Other;
                    }
                    Some(Self {
                        desc,
                        len,
                        data: profile,
                    })
                }
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn profile_type(&self) -> IccpType {
        self.desc.to_owned()
    }
    // pub fn _data(self) -> Profile {
    //     self.data
    // }
    // pub fn _profile_size(&self) -> usize {
    //     self.len
    // }
    pub fn default() -> Self {
        Self {
            data: lcms2::Profile::new_srgb(),
            len: 3144,
            desc: IccpType::IECsRGB,
        }
    }
    pub fn into_bytes(self) -> Bytes {
        let profile = self.data;
        Bytes::from(profile.icc().unwrap())
    }
    pub fn from_file(path: &str) -> Result<Self, CustomErr> {
        let reader = std::fs::read(path)?;
        let len = reader.len();
        let profile = Profile::new_icc(&reader)?;
        let desc = qualify_profile(&profile).unwrap();
        Ok(Self {
            data: profile,
            desc,
            len,
        })
    }
    pub fn from_bytes(buf: &[u8]) -> Result<Self, CustomErr> {
        let data = lcms2::Profile::new_icc(buf)?;
        let desc = iccp::qualify_profile(&data).unwrap();
        let len = buf.len();
        Ok(Self { data, desc, len })
    }
}
pub fn qualify_profile(p: &Profile) -> Option<IccpType> {
    let desc: IccpType;
    match p.info(InfoType::Description, Locale::none()) {
        Some(s) => {
            let s = s.to_lowercase();
            if s.contains("iec") && s.contains("srgb") {
                desc = IccpType::IECsRGB;
            } else if s.contains("adobe") && s.contains("rgb") {
                desc = IccpType::AdobeRGB;
            } else if s.contains("srgb") && s.contains("google") {
                desc = IccpType::GoogleSrgb;
            } else if s.contains("srgb") {
                desc = IccpType::OtherSrgb;
            } else {
                desc = IccpType::Other;
            }
            Some(desc)
        }
        _ => None,
    }
}
