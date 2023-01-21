use base64;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, env)]
    spotify_client_id: String,

    #[clap(long, env)]
    spotify_client_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let auth = format!("{}:{}", args.spotify_client_id, args.spotify_client_secret);
    let auth64 = base64::encode(auth);
    let basic = format!("Basic {}", auth64);
    let params = [("grant_type", "client_credentials")];
    let user = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", basic)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    println!("Hello, world!{:?}", user);
    Ok(())
}
