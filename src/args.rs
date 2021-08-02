use crate::counter::Counter;

use super::*;

#[derive(Debug, Clone)]
pub struct ArgsInput {
    path_option: Option<String>,
    options: Option<Vec<String>>,
    exe_dir: Option<String>,
}
impl ArgsInput {
    pub fn new(mut args: env::Args) -> Self {
        let exe_dir = args.next().take();
        let path_option = args.next().take();
        let mut options = None;
        if args.len() > 0 {
            options = args.map(Some).collect();
        }
        Self {
            path_option,
            options,
            exe_dir,
        }
    }
    pub fn process(mut self) -> Result<Counter, CustomErr> {
        let output: Counter;
        match self.options {
            Some(opt) => {
                if opt.contains(&"adobe-allow".to_string()) {
                    unsafe {
                        ALLOW_ADOBE_RGB = true;
                    }
                }
                if opt.contains(&"--all".to_string()) {
                    output = process_dir_inp(&self.path_option.take().unwrap(), true)?;
                } else {
                    let path = Path::new(&self.path_option.take().unwrap()).canonicalize()?;
                    output = process_file_inp(path)?
                }
            }

            _ => {
                match self.exe_dir {
                    None => return Err(custom_err::from("cud not infere executable path")),
                    Some(s) => {
                        //delete filename from exe_dir path str:
                        let path = Path::new(&s).parent().unwrap().canonicalize()?;
                        output = process_dir_inp(path.to_str().unwrap(), true)?
                    }
                };
            }
        }
        Ok(output)
    }
}
