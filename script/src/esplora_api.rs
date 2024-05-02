use hex::prelude::*;
use serde_json::Value;

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

    Ok(<[u8; 80]>::from_hex(resp.as_str())?)
}

pub async fn get_block_tx(block_hash: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!(
        "https://blockstream.info/api/block/{block_hash}/txids"
    ))
    .await?
    .text()
    .await?;
    let j: Value = serde_json::from_str(&resp)?;
    Ok(j)
}

// https://blockstream.info/api/block/00000000000000000002f7538b43249a3120f12c3aa4f14f433ce4a61d0b44e0/txs/
