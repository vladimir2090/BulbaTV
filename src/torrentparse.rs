use lava_torrent::torrent::v1::Torrent;
use hex::encode;
use std::error::Error;

pub fn reader_torrent(path: &str) -> Result<(), Box<dyn Error>> {
    let torrent = Torrent::read_from_file(path)?;
    let announce = torrent.announce.as_ref().ok_or("[ERROR] announce not found")?;
    println!("[MSG] announce: {}", announce);

    let info_hash = torrent.info_hash();
    println!("[MSG] info hash (hex): {}", encode(info_hash));

    Ok(())
}