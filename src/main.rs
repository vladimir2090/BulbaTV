use lava_torrent::torrent::v1::Torrent;
use hex::encode;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Deserialize)]

struct Config {
    path_torrent: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("config.json")?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;

    println!("{}", config.path_torrent);

    if let Err(e) = reader_torrent(&config.path_torrent) {
        println!("[ERROR] {}", e);
    }

    Ok(())
}

// Теперь функция корректно принимает путь по ссылке
fn reader_torrent(path: &str) -> Result<(), Box<dyn Error>> {
    let torrent = Torrent::read_from_file(path)?;
    let announce = torrent.announce.as_ref().ok_or("[ERROR] announce not found")?;
    println!("[MSG] announce: {}", announce);

    let info_hash = torrent.info_hash();
    println!("[MSG] info hash (hex): {}", encode(info_hash));

    Ok(())
}