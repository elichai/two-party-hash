use rustc_hex::{FromHex, FromHexError};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "TwoPartyHash",
    about = "A tool that let's 2 parties agree on one preimage of an hash without revealing secrets."
)]
pub enum Cli {
    #[structopt(name = "first")]
    First { preimage: String },
    #[structopt(name = "Second")]
    Second {
        #[structopt(parse(try_from_str = "parse_hex32"), raw(required = "true"))]
        current_hash: [u8; 32],
        preimage: String,
    },
    #[structopt(name = "verify")]
    Verify {
        preimage: String,
        #[structopt(parse(try_from_str = "parse_hex32"), raw(required = "true"))]
        hash: [u8; 32],
    },
}

fn parse_hex32(hex: &str) -> Result<[u8; 32], FromHexError> {
    let hex = parse_hex(hex)?;
    if hex.len() != 32 {
        // TODO: formalize an error
        return Err(FromHexError::InvalidHexLength);
    }
    let mut result = [0u8; 32];
    result.copy_from_slice(&hex);
    Ok(result)
}

fn parse_hex(hex: &str) -> Result<Vec<u8>, FromHexError> {
    let hex = if "0x" == hex.chars().take(2).collect::<String>().as_str() {
        hex.chars().skip(2).collect()
    } else {
        hex.to_owned()
    };
    hex.from_hex()
}
