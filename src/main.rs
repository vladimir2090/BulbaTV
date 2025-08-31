use lava_torrent::torrent::v1::Torrent;
use hex::encode;
use std::error::Error;

fn main() {
    if let Err(e) = reader_torrent() {
        println!("[ERROR] {}", e);
    }
}

fn reader_torrent() -> Result<(), Box<dyn Error>> {
    let torrent = Torrent::read_from_file(r"C:\Users\admin\Desktop\torrent_folder\yourfile.torrent")?;
    
    let announce = torrent.announce.as_ref().ok_or("[ERROR] announce not found")?;
    println!("[MSG] announce: {}", announce);

    let info_hash = torrent.info_hash();
    println!("[MSG] info hash (hex): {}", encode(info_hash));

    Ok(())
}