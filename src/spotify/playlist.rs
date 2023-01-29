use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlists {
    pub href: String,
    pub items: Vec<Item>,
    pub limit: Option<u32>,
    pub next: Option<u32>,
    pub offset: Option<u32>,
    pub previous: Option<u32>,
    pub total: Option<u32>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub collaborative: bool,
    pub description: String,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub owner: Owner,
    pub tracks: Tracks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    #[serde(rename = "display_name")]
    pub display_name: String,
    pub href: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracks {
    pub href: String,
    pub total: Option<u32>,
}

pub async fn get_user_playlists(token: String, user: String) -> Result<Playlists, reqwest::Error> {
    let bearer = format!("Bearer {}", token);
    let playlists: Playlists = reqwest::Client::new()
        .get(format!(
            "https://api.spotify.com/v1/users/{}/playlists",
            user
        ))
        .header("Authorization", bearer)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(playlists)
}
