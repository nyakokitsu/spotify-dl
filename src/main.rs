use clap::Parser;
use std::error::Error;
use downloader::download;

pub mod channel_sink;
pub mod track;
pub mod encoder;
pub mod downloader;
pub mod metadata;
pub mod auth;

/// Really simple spotify downloader
#[derive(Parser)]
#[command(name = "Spotify Downloader")]
#[command(about = "Downloads an audio track using an access token", long_about = None)]
struct Cli {
    /// Access token for authentication
    access_token: String,

    /// Output dir (optional)
    /// If not provided, the current directory will be used
    #[arg(short, long, default_value = "./")]
    output_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let session = auth::connect(&cli.access_token).await;
    let uris = std::fs::read_to_string("uris.txt")?
        .lines()
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    for (index, _uri) in uris.iter().enumerate() {
            println!("Downloading track {}/{}: {}", index + 1, uris.len(), _uri);
            let (audio_stream, artists, title, cover_id) = download(_uri, session.clone()).await;
            let file_name = format!("{}{} - {}.mp3", &cli.output_dir, artists, title);
            let cover_url = format!("https://i.scdn.co/image/{}", cover_id.unwrap_or_default());
            let cover_data = reqwest::get(&cover_url).await?.bytes().await?;
            
            let audio_stream_vec = audio_stream.as_vec().unwrap();
            let mut cloned_vec = audio_stream_vec.clone();
            let _ = metadata::MetadataTagger::add_title_tag(&mut cloned_vec, &title);
            let _ = metadata::MetadataTagger::add_author_tag(&mut cloned_vec, &artists);
            let _ = metadata::MetadataTagger::add_cover_tag(&mut cloned_vec, &cover_data, "image/jpeg");
            
            std::fs::write(&file_name, &cloned_vec)?;
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
    

    Ok(())
}
