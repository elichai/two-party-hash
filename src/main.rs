mod cli;
mod sha2;
mod utils;

use crate::sha2::Sha256;
use crate::utils::parse_hex64;
use cli::Cli;
use rand_os::{rand_core::RngCore, OsRng};
use rustc_hex::ToHex;
use std::convert::TryInto;
use structopt::clap::{Error as ClapError, ErrorKind as ClapErrorKind};
use structopt::StructOpt;

fn main() {
    println!("Hello, world!");
    let cli = Cli::from_args();
    if let Err(e) = handle_cli(cli) {
        e.exit();
    }
}

fn handle_cli(cli: Cli) -> Result<(), ClapError> {
    match cli {
        Cli::First => {
            let mut preimage = [0u8; 64];
            OsRng.fill_bytes(&mut preimage);
            let res = unsafe { Sha256::one_block_no_padding(preimage) };
            println!("Preimage: 0x{}", preimage.to_hex::<String>());
            println!("Non finalized hashed: 0x{}", res.to_hex::<String>());
        }
        Cli::Second { mid_hash } => {
            let mut hash = unsafe { Sha256::from_one_block(mid_hash) };
            let mut preimage = [0u8; 32];
            OsRng.fill_bytes(&mut preimage);

            hash.input(&preimage);
            let res = hash.finalize();
            println!("Preimage: 0x{}", preimage.to_hex::<String>());
            println!("Final hashed: 0x{}", res.to_hex::<String>());
        }
        Cli::Verify { first_preimage, second_preimage } => {
            let mut hash = Sha256::new();
            hash.input(&first_preimage);
            hash.input(&second_preimage);
            let res = hash.finalize();
            println!("Regular hash of both preimages: 0x{}", res.to_hex::<String>());
        }
    }
    Ok(())
}
