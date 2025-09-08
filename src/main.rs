mod torrentparse;
mod http;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use serde_yaml::from_reader;
use torrentparse::TorrentInfo;
use hex::encode;

#[derive(Deserialize)]
#[warn(dead_code)]
struct Config {
    path_torrent: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("config.yaml")?;
    let reader = BufReader::new(file);
    let config: Config = from_reader(reader)?;
    let info = TorrentInfo::from_file(&config.path_torrent)?;

    println!("Announce URL: {}", info.announce);
    println!("Info hash (hex): {}", encode(&info.info_hash));
    println!("Name: {}", info.name);
    println!("Total size: {} bytes", info.total_length);
    println!("Piece length: {} bytes", info.piece_length);
    println!("Number of pieces: {}", info.pieces_count);

    Ok(())
}