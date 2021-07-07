mod process;
use process::*;
use std::{env::*, path::Path};

pub static MANAGEABLE_FILE_EXTENSIONS: [&str; 4] = ["jpg", "tiff", "jpeg", "webp"];
#[derive(Debug)]
struct ArgsInput {
    path: Option<String>,
    options: Option<Vec<String>>
}
impl ArgsInput {
    fn new(mut args: Args) -> Self {
        //args.nth(0).take();
        let path = args.nth(1).take();
        let mut options = None;
        if args.len() > 1 {
            options = args.map(|s| Some(s)).collect();
        }
        Self {
            path, options
        }
    }
    fn process(mut self) ->Result<(), CustomErr> {
        match self.options {
            Some (opt) if opt.contains(&"--all".to_string()) => {
                process_dir_inp(&self.path.take().unwrap(), false)?;
                //process all files in dir
            },
            _ => {
                let path = Path::new(&self.path.take().unwrap()).canonicalize()?;
                process_file_inp(&path)?
            }
    
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ArgsInput::new(args());
    args.process()?;
    Ok(())
}