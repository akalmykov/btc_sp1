use bitcoin::{
    hashes::{sha256, sha256d, Hash},
    merkle_tree, Txid,
};
use hex::prelude::*;
use sp1_sdk::{utils, ProverClient, SP1Stdin};
use std::{error::Error, str::FromStr};
// use bitcoin::{block::Header, consensus::deserialize};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
mod esplora_api;
use esplora_api::{get_block_header, get_block_tx};

// https://people.csail.mit.edu/silvio/Selected%20Scientific%20Papers/Zero%20Knowledge/Zero-Knowledge_Sets.pdf
//https://alinush.github.io/2023/02/05/Why-you-should-probably-never-sort-your-Merkle-trees-leaves.html
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bloch_hash = "00000000000000000002f7538b43249a3120f12c3aa4f14f433ce4a61d0b44e0";
    let raw_header = get_block_header(&bloch_hash).await?;

    let tx_json = get_block_tx(bloch_hash).await?;
    let mut hash_vec = vec![];
    for tx in tx_json.as_array().unwrap() {
        let mut hash_bytes = <[u8; 32]>::from_hex(tx.as_str().unwrap())?;
        hash_bytes.reverse();
        let btc_hash = sha256d::Hash::from_slice(&hash_bytes)?;
        hash_vec.push(btc_hash);
    }
    let root = merkle_tree::calculate_root(hash_vec.into_iter());
    match root {
        Some(root) => {
            println!("calculated merkle root2: {:?}", root);
        }
        None => {
            println!("could not calculate root");
        }
    }

    Ok(())

    // utils::setup_logger();
    // // Generate proof.
    // let mut stdin = SP1Stdin::new();
    // // let n = 186u32;
    // // stdin.write(&n);
    // let mut stdin = SP1Stdin::new();
    // stdin.write_slice(&raw_header);

    // let client = ProverClient::new();
    // let mut proof = client.prove(ELF, stdin).expect("proving failed");

    // // Read output.
    // let a = proof.public_values.read::<[u8; 32]>();
    // // let b = proof.public_values.read::<u128>();
    // let hex_string = a.to_lower_hex_string();
    // println!("a: {}", hex_string);
    // // println!("b: {}", b);

    // // Verify proof.
    // client.verify(ELF, &proof).expect("verification failed");

    // // Save proof.
    // proof
    //     .save("proof-with-io.json")
    //     .expect("saving proof failed");

    // println!("successfully generated and verified proof for the program!");
    // Ok(())
}
