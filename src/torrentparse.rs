use lava_torrent::torrent::v1::Torrent;
use anyhow::Result;

pub struct TorrentInfo {
    pub name: String,
    pub total_length: i64,
    pub piece_length: i64,
    pub pieces_count: usize,
    pub info_hash: Vec<u8>,
    pub announce: String,
}

impl TorrentInfo {
    pub fn from_file(path: &str) -> Result<Self> {
        let torrent = Torrent::read_from_file(path)?;
        let name = torrent.name.clone();
        let total_length = torrent.files
            .as_ref()
            .map(|files| files.iter().map(|f| f.length).sum())
            .unwrap_or(torrent.length);
        let piece_length = torrent.piece_length;
        let pieces_count = torrent.pieces.len();
        let info_hash = torrent.info_hash_bytes().to_vec();
        let announce = torrent
            .announce
            .clone()
            .ok_or_else(|| anyhow::anyhow!("[ERROR] announce not found"))?;
        Ok(Self { name, total_length, piece_length, pieces_count, info_hash, announce })
    }
}