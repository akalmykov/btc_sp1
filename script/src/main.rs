use hex::prelude::*;
use sp1_sdk::{utils, ProverClient, SP1Stdin};
use std::error::Error;
// use bitcoin::{block::Header, consensus::deserialize};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
mod esplora_api;
use esplora_api::get_block_header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let raw_header =
        get_block_header("00000000000000000002f7538b43249a3120f12c3aa4f14f433ce4a61d0b44e0")
            .await?;
    // let some_header: Header = deserialize(&header_hex)?;
    // let s = "006000208b91a6f87fb412bbdf95b67f944fca42313eaab55c370100000000000000000004efe0609fd39b308e80975743f32dd023ca7a1117d0e744b629412b0046c23e9f650866d36203177d0fa204";
    // let n = <[u8; 80]>::from_hex(s).unwrap();
    utils::setup_logger();
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    // let n = 186u32;
    // stdin.write(&n);
    let mut stdin = SP1Stdin::new();
    stdin.write_slice(&raw_header);

    let client = ProverClient::new();
    let mut proof = client.prove(ELF, stdin).expect("proving failed");

    // Read output.
    let a = proof.public_values.read::<[u8; 32]>();
    // let b = proof.public_values.read::<u128>();
    let hex_string = a.to_lower_hex_string();
    println!("a: {}", hex_string);
    // println!("b: {}", b);

    // Verify proof.
    client.verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!");
    Ok(())
}

// //! A simple script to generate and verify the proof of a given program.

// use hex::prelude::*;
// use std::error::Error;

// // use bitcoin::{block::Header, consensus::deserialize};
// use sp1_sdk::{SP1Prover, SP1Stdin, SP1Verifier};
// const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
// mod esplora_api;
// use esplora_api::get_block_header;

// // #[tokio::main]
// //async fn main() -> Result<(), Box<dyn Error>> {
// fn main() {
//     // let raw_header =
//     //     get_block_header("00000000000000000002f7538b43249a3120f12c3aa4f14f433ce4a61d0b44e0")
//     //         .await?;
//     // let some_header: Header = deserialize(&header_hex)?;
//     // let s = "006000208b91a6f87fb412bbdf95b67f944fca42313eaab55c370100000000000000000004efe0609fd39b308e80975743f32dd023ca7a1117d0e744b629412b0046c23e9f650866d36203177d0fa204";
//     // let n = <[u8; 80]>::from_hex(s).unwrap();
//     // // Generate proof.
//     let mut stdin = SP1Stdin::new();
//     let n = [1, 2, 3];
//     stdin.write_slice(&n);
//     let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

//     // Read output.
//     let a = proof.stdout.read::<u128>();
//     let b = proof.stdout.read::<u128>();
//     println!("a: {}", a);
//     println!("b: {}", b);

//     // Verify proof.
//     SP1Verifier::verify(ELF, &proof).expect("verification failed");

//     // Save proof.
//     proof
//         .save("proof-with-io.json")
//         .expect("saving proof failed");

//     println!("successfully generated and verified proof for the program!");
//     // Ok(())
// }
