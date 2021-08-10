//#![allow(unused)]
use super::*;
use counter::Counter;
use image_meta::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

pub fn process_dir_inp(dir: &str, recurse: bool) -> Result<Counter, CustomErr> {
    let mut counter = Counter::new();
    if !path_is_dir(dir) {
        return Err(custom_err::from("input is not a dir"));
    }
    let mut files: Vec<DirEntry> = Vec::new();
    for e in Path::new(dir).read_dir()?.flatten() {
        if recurse && path_is_dir(e.path().to_str().unwrap()) {
            counter = counter + process_dir_inp(e.path().to_str().unwrap(), true).unwrap();
        }

        if (&e).path().is_file() && !fs::symlink_metadata(e.path())?.file_type().is_symlink() {
            files.push(e)
        }
    }

    let images: Vec<Image> = files
        .into_iter()
        .filter_map(|e| {
            if let Ok(image) = Image::read(e.path()) {
                Some(image)
            } else {
                println!{"cannot parse image from: {:?}", e.path()};
                None
            }
        })
        .filter(Image::is_manageable)
        .collect();
    //using rayon par_iterator for multithreaded processing of images
    let results: Vec<Result<Counter, CustomErr>> =
        images.into_par_iter().map(process_image).collect();

    let (counters, errors): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);

    let mut counter = counters
        .into_iter()
        .map(Result::unwrap)
        .fold(counter, |c1, c2| c1 + c2);

    errors
        .into_iter()
        .map(Result::unwrap_err)
        .for_each(|err| counter.errors.push(err));

    Ok(counter)
}
pub fn process_image(mut img: Image) -> Result<Counter, CustomErr> {
    println!("processing image {:?}", ImageInfo::new(&img));
    let old_path = img.path.to_owned();
    let mut extension_changed = false;
    let mut was_modified = false;
    let mut counter = Counter::new();
    let old_exif = img.decoded.exif();

    match img.img_format() {
        //if jpeg || tiff => no conversion needed
        Some(ImageFormat::Jpeg) | Some(ImageFormat::Tiff) => {}
        Some(_) => {
            img = img.convert_format()?;
            was_modified = true;
            extension_changed = true;
        }
        None => return Err(custom_err::from("unknown image format")),
    };

    match img.iccp() {
        Some(profile) => {
            match &profile.profile_type() {
                //only QSS37 series & above can handle AdobeRGB correctly
                IccpType::AdobeRGB => {
                    //#safety
                    //allow_adobe_rgb static is mutated only once at startup
                    if unsafe { !ALLOW_ADOBE_RGB } {
                        img = img
                            .convert_iccp(
                                &profile.data,
                                &Iccp::from_bytes(&static_iecsrgb::SRGB_IEC)?.data,
                            )?
                            .set_IECsRGB_profile()?;
                            was_modified = true;
                    }
                    counter.adobe_rgb += 1;
                }
                IccpType::IECsRGB => {
                    counter.iec_srgb += 1;
                }
                IccpType::Other => {
                    counter.other += 1;
                    img = img
                        .convert_iccp(
                            &profile.data,
                            &Iccp::from_bytes(&static_iecsrgb::SRGB_IEC)?.data,
                        )?
                        .set_IECsRGB_profile()?;
                    was_modified = true;
                }
                //in this case need to set IEC_sRGB instead because
                //those can't be interpreted correctly by the Noritsu QSS software:
                //no conversion needed as they are basically identical
                IccpType::OtherSrgb | IccpType::GoogleSrgb => {
                    counter.other_srgb += 1;
                    img = img.set_IECsRGB_profile()?;
                    was_modified = true;
                }
            }
        }
        //if no color profile present presume that profile is IECsRGB
        None => {
            counter.no_profile += 1;
            img = img.set_IECsRGB_profile()?;
            was_modified = true;
        }
    }

    if was_modified {
        img.decoded.set_exif(old_exif);
        //todo fix errors on write protection
        img.save()?;
    }
    if extension_changed {
        fs::remove_file(*old_path)?;
    }
    Ok(counter)
}
pub fn process_file_inp(path: PathBuf) -> Result<Counter, CustomErr> {
    let image = Image::read(path)?;
    let counter = process_image(image)?;
    Ok(counter)
}

fn path_is_dir(input: &str) -> bool {
    Path::new(input).is_dir()
}
