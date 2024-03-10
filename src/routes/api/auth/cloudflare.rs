use std::collections::HashMap;

use anyhow::anyhow;

use crate::{get_env, Result};

pub async fn verify_turnstitle(token: &str, ip: std::net::IpAddr) -> Result<()> {
    #[derive(serde::Deserialize)]
    struct Turnstitle {
        success: bool,
    }
    let ip = ip.to_string();
    let secret_key = get_env("TURNSTITLE_SECRET_KEY".into())?;
    let url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
    let client = reqwest::Client::new();
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("secret", &secret_key);
    params.insert("response", &token);
    params.insert("remoteip", &ip);
    let resp = client.post(url).form(&params).send().await?;
    let resp = resp.json::<Turnstitle>().await?;
    match resp.success {
        true => Ok(()),
        false => Err(anyhow!("failed to verify turnstitle").into()),
    }
}
