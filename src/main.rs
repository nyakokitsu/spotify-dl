use clap::Parser;
use std::error::Error;
use downloader::download;

pub mod channel_sink;
pub mod track;
pub mod encoder;
pub mod downloader;

/// Really simple spotify downloader
#[derive(Parser)]
#[command(name = "Spotify Downloader")]
#[command(about = "Downloads an audio track using an access token", long_about = None)]
struct Cli {
    /// Access token for authentication
    access_token: String,

    /// Identifier of the track
    track: String,

    /// Output dir (optional)
    /// If not provided, the current directory will be used
    #[arg(short, long, default_value = "./")]
    output_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let (audio_stream, artists, title) = download(&cli.track, &cli.access_token).await;
    let file_name = format!("{}{} - {}.mp3", &cli.output_dir, artists, title);
    audio_stream.write_to_file(&file_name).await?;

    Ok(())
}
