use super::*;
use lcms2::*;
pub enum IccpType {
    IECSrgb,
    AdobeRGB,
    GoogleSrgb,
    OtherSrgb,
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
                    if s.contains("iec") && s.contains("srgb")&& len > 3100 {
                        desc = IccpType::IECSrgb;
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