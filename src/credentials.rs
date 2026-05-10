use std::error::Error;
use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

pub fn credentials_path() -> Result<PathBuf, Box<dyn Error>> {
    let base = dirs::config_dir().ok_or("no config dir")?;
    Ok(base.join("insaali").join("credentials"))
}

pub fn save_token(token: &str) -> Result<(), Box<dyn Error>> {
    let path = credentials_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&path)?;
    f.write_all(token.as_bytes())?;
    Ok(())
}

pub fn read_token() -> Result<String, Box<dyn Error>> {
    let path = credentials_path()?;
    if !path.exists() {
        return Err("not signed in; run `ia login`".into());
    }
    Ok(fs::read_to_string(&path)?.trim().to_string())
}
