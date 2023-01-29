use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls {
    pub spotify: String,
}
pub async fn get_user(token: String, user: String) -> Result<User, reqwest::Error> {
    let bearer = format!("Bearer {}", token);
    let user_info: User = reqwest::Client::new()
        .get(format!("https://api.spotify.com/v1/users/{}", user))
        .header("Authorization", bearer)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(user_info)
}
