//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2_v0_10_8::Digest;
use sha2_v0_10_8::Sha256;

pub fn main() {
    let m = sp1_zkvm::io::read_vec();
    let mut hasher = Sha256::new();
    hasher.update(m);
    let hash = hasher.finalize();
    hasher = Sha256::new();
    hasher.update(hash);
    let hash2 = hasher.finalize();
    let mut ret = [0u8; 32];
    ret.copy_from_slice(&hash2);

    sp1_zkvm::io::commit(&ret);
}
