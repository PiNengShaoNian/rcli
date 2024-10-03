use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Result;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}
