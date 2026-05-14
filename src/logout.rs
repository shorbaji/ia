use std::error::Error;
use std::fs;

use crate::credentials::credentials_path;

pub fn run() -> Result<(), Box<dyn Error>> {
    let path = credentials_path()?;
    match fs::remove_file(&path) {
        Ok(()) => {
            println!("signed out. removed {}", path.display());
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("not signed in.");
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
