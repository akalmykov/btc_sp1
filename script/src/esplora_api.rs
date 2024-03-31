use hex::prelude::*;

// https://blockstream.info/api/blocks/837003
// https://github.com/Blockstream/esplora/blob/master/API.md
// https://blockstream.info/api/block/00000000000000000002f7538b43249a3120f12c3aa4f14f433ce4a61d0b44e0/header

pub async fn get_block_header(block_hash: &str) -> Result<[u8; 80], Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!(
        "https://blockstream.info/api/block/{block_hash}/header"
    ))
    .await?
    .text()
    .await?;
    println!("{}", resp);

    Ok(<[u8; 80]>::from_hex(resp.as_str())?)
}
