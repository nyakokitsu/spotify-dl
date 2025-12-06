use librespot::{
    core::{
        SpotifyUri,
        session::Session,
    },
    playback::{
        config::{PlayerConfig},
        mixer::NoOpVolume,
        player::Player,
    },
};

use crate::channel_sink::ChannelSink;
use crate::encoder::Format;
use crate::encoder::Samples;
use crate::track::Track;
use crate::channel_sink::SinkEvent;
use crate::encoder::EncodedStream;




pub async fn download(trackid: &str, session: Session) -> (EncodedStream, String, String, Option<String>) {
    let trid = trackid;
    println!("Track id: {}", trid);
    let track = SpotifyUri::from_uri(&*trackid).unwrap();

    let player_config = PlayerConfig::default();

    let track_obj = Track::from_id(track.clone());
    let metadata = track_obj.metadata(&session).await.ok().unwrap();

    let cover_id = metadata.cover_url.clone();

    let artists = metadata
        .artists
        .iter()
        .map(|artist| artist.name.clone())
        .collect::<Vec<_>>()
        .join(", ");
    println!("Downloading track: {} - {}", artists, metadata.track_name);
    let track_name = metadata.track_name.clone();
    let (sink, mut sink_channel) = ChannelSink::new(metadata);

    let file_size = sink.get_approximate_size();
    println!("File size: {:.2} MB", file_size as f64 / (1024.0 * 1024.0 * 10.0));
    
    let player = Player::new(player_config, session, Box::new(NoOpVolume), move || Box::new(sink));
    

    player.load(track, true, 0);

    let mut samples = Vec::<i32>::new();
    
    tokio::spawn(async move {
            player.await_end_of_track().await;
            player.stop();
    });

    
    println!("Ahh shit, here we go again");
    println!("Downloading...");
    while let Some(event) = sink_channel.recv().await {
        match event {
            SinkEvent::Write { bytes: _, total: _, mut content } => {
                samples.append(&mut content);
            }
            SinkEvent::Finished => {
                println!("Downloaded!");
                break;
            }
        }
    }
    println!("Running encoding...");
    let samples = Samples::new(samples, 44100, 2, 16);
    let encoder = crate::encoder::get_encoder(Format::Mp3);
    let stream = encoder.encode(samples).await.ok().unwrap();
    return (stream, artists, track_name, cover_id);
}
