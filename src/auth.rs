
use std::process::exit;
use librespot::{
    core::{
        authentication::Credentials,
        session::Session,
        config::SessionConfig,
    }
};

pub async fn connect(token: &str) -> Session {
    println!("Connecting...");
    let session_config = SessionConfig::default();
    let session = Session::new(session_config, None);

    let credentials = Credentials::with_access_token(token);

    if let Err(e) = session.connect(credentials, true).await {
        println!("Error connecting: {}", e);
        exit(1);
    }
    println!("Connected!");
    return session;
}