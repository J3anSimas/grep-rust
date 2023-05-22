use std::{env::current_dir, path::PathBuf, process};

pub enum ConfigBuildError {
    Test,
}
pub struct Config {
    path: PathBuf,
    pattern: String,
    github: bool,
    limit_size: u32,
}
impl Config {
    pub fn build(args: Vec<String>) -> Result<Self, ConfigBuildError> {
        let mut path: PathBuf = PathBuf::new();
        let pattern: String = match args.get(1) {
            Some(value) => value.clone(),
            None => {
                eprintln!("No pattern to search for provided");
                process::exit(1);
            }
        };
        let mut github: bool = true;
        let mut limit_size = 50_000;
        for (index, arg) in args.iter().enumerate() {
            if arg == "-path" {
                if let Some(value) = args.get(index + 1) {
                    path.push(value);
                } else {
                    eprintln!("You tried to overwrite the path but didn't set a value");
                    process::exit(1);
                }
            } else if arg == "-gitignore" {
                github = false;
            } else if arg == "-limit-size" {
                if let Some(value) = args.get(index + 1) {
                    match value.parse() {
                        Ok(result) => limit_size = result,
                        Err(e) => {
                            eprintln!("The value you passed could not be parsed to a number");
                            return Err(ConfigBuildError::Test);
                        }
                    }
                } else {
                    eprintln!("You tried to overwrite the limit size but didn't set a value");
                    process::exit(1);
                }
            }
        }
        if path == PathBuf::new() {
            path = match current_dir() {
                Ok(res) => res,
                Err(_) => panic!(""),
            };
        }
        Ok(Config {
            path,
            pattern,
            github,
            limit_size,
        })
    }

    pub fn github(&self) -> bool {
        self.github
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn pattern(&self) -> &str {
        self.pattern.as_ref()
    }

    pub fn limit_size(&self) -> u32 {
        self.limit_size
    }
}
