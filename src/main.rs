use clap::{Parser, Subcommand};
use spotgenre::spotify::playlist::Playlists;
use spotgenre::spotify::user::{self, User};
use spotgenre::spotify::{auth, playlist};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Commands,

    #[clap(long, env)]
    spotify_client_id: String,

    #[clap(long, env)]
    spotify_client_secret: String,

    #[clap(short, long, env)]
    spotify_user: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Retrieves all playlists for given user
    Playlist,
    /// Returns information about given user
    User,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let auth = auth::get_token(args.spotify_client_id, args.spotify_client_secret).await?;
    match &args.commands {
        Commands::Playlist => {
            let playlists: Playlists =
                playlist::get_user_playlists(auth.access_token, args.spotify_user).await?;
            let all: Vec<String> = playlists.items.iter().map(|p| p.name.clone()).collect();
            println!("{:#?}", all);
            Ok(())
        }
        Commands::User => {
            let user: User = user::get_user(auth.access_token, args.spotify_user).await?;
            println!("user ID: {}, users name: {}", user.id, user.display_name);
            Ok(())
        }
    }
}
