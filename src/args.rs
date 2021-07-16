use  super::*;

#[derive(Debug)]
pub struct ArgsInput {
    path: Option<String>,
    options: Option<Vec<String>>
}
impl ArgsInput {
    pub fn new(mut args: env::Args) -> Self {
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
    pub fn process(mut self) ->Result<(), CustomErr> {
        match self.options {
            Some (opt) if opt.contains(&"--all".to_string()) => {
                let counter  = process_dir_inp(&self.path.take().unwrap(), true)?;
                println!("Counter: {:?}", counter);
                //process all files in dir
            },
            _ => {
                let path = Path::new(&self.path.take().unwrap()).canonicalize()?;
                process_file_inp(path)?
            }
    
        }
        Ok(())
    }
}
