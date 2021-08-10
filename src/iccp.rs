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
        if profile_bytes.is_none() { return None }
        let profile_bytes = profile_bytes.unwrap();

        let len = profile_bytes.len();
        let data = iccp_from_bytes(profile_bytes);

        if data.is_none() { return None }
        let data = data.unwrap();

        let desc = qualify_profile(&data, len);
        Some(Self{ desc,len, data})
    }

    pub fn profile_type(&self) -> IccpType {
        self.desc.to_owned()
    }

    pub fn default() -> Self {
        Self {
            data: lcms2::Profile::new_icc(&SRGB_IEC).expect("unable to read static iec_srgb"),
            len: SRGB_IEC.len(),
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
        let desc = qualify_profile(&profile, len);
        Ok(Self {
            data: profile,
            desc,
            len,
        })
    }
    pub fn from_bytes(buf: &[u8]) -> Result<Self, CustomErr> {
        let data = lcms2::Profile::new_icc(buf)?;
        let len = buf.len();
        let desc = iccp::qualify_profile(&data, len);
        Ok(Self { data, desc, len })
    }
}
fn qualify_profile(p: &Profile, len: usize) -> IccpType {
    match p.info(InfoType::Description, Locale::none()) {
        Some(s) => {
            let s = s.to_lowercase();
            if s.contains("iec") && s.contains("srgb") && len != 3144 {
                IccpType::IECsRGB
            } else if s.contains("adobe") && s.contains("rgb") {
                IccpType::AdobeRGB
            } else if s.contains("srgb") && s.contains("google") {
                IccpType::GoogleSrgb
            } else if s.contains("srgb") {
                IccpType::OtherSrgb
            } else {
                IccpType::Other
            }
        },
        None => IccpType::Other
    }
}

fn iccp_from_bytes(profile_bytes: Bytes) -> Option<Profile> {
    Profile::new_icc(&profile_bytes[..]).ok()
}
