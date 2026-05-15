use std::error::Error;

use reqwest::blocking::Client;

use crate::api::{api_url, CreateRunRequest, LogsResponse, Run};
use crate::credentials::read_token;

fn auth() -> Result<(Client, String, String), Box<dyn Error>> {
    let token = read_token()?;
    Ok((Client::new(), token, api_url()))
}

pub fn simeval(simulator: &str, policy_ref: &str, max_steps: u32) -> Result<(), Box<dyn Error>> {
    let (client, token, base) = auth()?;
    let resp = client
        .post(format!("{base}/runs"))
        .bearer_auth(&token)
        .json(&CreateRunRequest {
            simulator,
            policy_ref,
            max_steps,
        })
        .send()?
        .error_for_status()?;
    let run: Run = resp.json()?;
    println!("{}", run.id);
    println!("status: {}", run.status);
    Ok(())
}

pub fn status(run_id: &str) -> Result<(), Box<dyn Error>> {
    let (client, token, base) = auth()?;
    let resp = client
        .get(format!("{base}/runs/{run_id}"))
        .bearer_auth(&token)
        .send()?
        .error_for_status()?;
    let run: Run = resp.json()?;
    println!("id:        {}", run.id);
    println!("simulator: {}", run.simulator);
    println!("policy:    {}", run.policy_ref);
    println!("status:    {}", run.status);
    if let Some(err) = run.error_message {
        println!("error:     {err}");
    }
    Ok(())
}

pub fn logs(run_id: &str) -> Result<(), Box<dyn Error>> {
    let (client, token, base) = auth()?;
    let resp = client
        .get(format!("{base}/runs/{run_id}/logs"))
        .bearer_auth(&token)
        .send()?
        .error_for_status()?;
    let body: LogsResponse = resp.json()?;
    print!("{}", body.logs);
    Ok(())
}
