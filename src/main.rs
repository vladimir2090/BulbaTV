mod torrentparse;
//mod http;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use hex::encode;
use torrentparse::TorrentInfo;

#[derive(Deserialize)]

struct Config {
    path_torrent: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_bytes = tokio::fs::read("config.yaml").await?;
    let config: Config = serde_yaml_ng::from_slice(&config_bytes)
        .map_err(|e| anyhow!("[ERROR] failed to parse config.yaml: {}", e))?;

    let join_handle = tokio::task::spawn_blocking(move || {
        TorrentInfo::from_file(&config.path_torrent)
    });

    let info = join_handle
        .await
        .map_err(|e| anyhow!("[ERROR] spawn_blocking join error: {}", e))??;

    println!("[MSG] Announce URL: {}", info.announce);
    println!("[MSG] Info hash (hex): {}", encode(&info.info_hash));
    println!("[MSG] Name: {}", info.name);
    println!("[MSG] Total size: {} bytes", info.total_length);
    println!("[MSG] Piece length: {} bytes", info.piece_length);
    println!("[MSG] Number of pieces: {}", info.pieces_count);

    Ok(())
}