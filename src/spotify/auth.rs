use base64;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Auth {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String,
}

pub async fn get_token(id: String, secret: String) -> Result<Auth, reqwest::Error> {
    let auth = format!("{}:{}", id, secret);
    let auth64 = base64::encode(auth);
    let basic = format!("Basic {}", auth64);
    let params = [
        ("grant_type", "client_credentials"),
        ("scopes", "user-read-private user-read-email"),
    ];
    Ok(reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", basic)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?
        .json()
        .await?)
}
