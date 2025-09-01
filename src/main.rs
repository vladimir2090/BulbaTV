mod torrentparse;
mod http;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use serde_json::from_reader;

#[derive(Deserialize)]

struct Config {
    path_torrent: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("config.json")?;
    let reader = BufReader::new(file);
    let config: Config = from_reader(reader)?;

    println!("{}", config.path_torrent);

    if let Err(e) = torrentparse::reader_torrent(&config.path_torrent) {
        println!("[ERROR] {}", e);
    }

    Ok(())
}