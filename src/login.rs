use std::error::Error;
use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use tiny_http::{Header, Response, Server};

const BASE_URL_DEFAULT: &str = "https://insaali.com";

pub fn run() -> Result<(), Box<dyn Error>> {
    let base = std::env::var("INSAALI_BASE_URL").unwrap_or_else(|_| BASE_URL_DEFAULT.to_string());
    let state = random_state()?;

    // Bind a local listener on a free port.
    let server = Server::http("127.0.0.1:0").map_err(|e| format!("local listener: {e}"))?;
    let port = server
        .server_addr()
        .to_ip()
        .ok_or("no local port assigned")?
        .port();

    let url = format!("{base}/cli/login?state={state}&port={port}");
    println!("Opening {url} ...");
    if let Err(e) = webbrowser::open(&url) {
        println!("Could not open browser automatically ({e}).");
        println!("Open this URL manually: {url}");
    }

    // Wait for the first request to /callback. Anything else gets a 404
    // (occasional preflight from the browser, favicons, etc.).
    loop {
        let request = server.recv()?;
        let parsed: url::Url = format!("http://localhost{}", request.url()).parse()?;
        if parsed.path() != "/callback" {
            request.respond(Response::from_string("Not Found").with_status_code(404))?;
            continue;
        }

        let mut token = None;
        let mut received_state = None;
        for (k, v) in parsed.query_pairs() {
            match k.as_ref() {
                "token" => token = Some(v.into_owned()),
                "state" => received_state = Some(v.into_owned()),
                _ => {}
            }
        }

        let token = token.ok_or("callback missing token")?;
        let got_state = received_state.ok_or("callback missing state")?;
        if got_state != state {
            request.respond(Response::from_string("state mismatch").with_status_code(400))?;
            return Err("state mismatch".into());
        }

        save_token(&token)?;

        let body = "<!doctype html><html><body><h1>signed in</h1><p>you can close this tab.</p></body></html>";
        let response = Response::from_string(body).with_header(
            Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap(),
        );
        request.respond(response)?;

        println!(
            "signed in. token saved to {}",
            credentials_path()?.display()
        );
        return Ok(());
    }
}

fn random_state() -> Result<String, Box<dyn Error>> {
    let mut buf = [0u8; 16];
    getrandom::getrandom(&mut buf).map_err(|e| format!("getrandom: {e}"))?;
    Ok(buf.iter().map(|b| format!("{:02x}", b)).collect())
}

pub fn credentials_path() -> Result<PathBuf, Box<dyn Error>> {
    let base = dirs::config_dir().ok_or("no config dir")?;
    Ok(base.join("insaali").join("credentials"))
}

fn save_token(token: &str) -> Result<(), Box<dyn Error>> {
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
