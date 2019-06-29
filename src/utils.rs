use rustc_hex::FromHex;
use structopt::clap::{Error as ClapError, ErrorKind as ClapErrorKind};

const NOT_VALID: ClapErrorKind = ClapErrorKind::ValueValidation;

pub fn parse_hex32(hex: &str) -> Result<[u8; 32], ClapError> {
    let hex = parse_hex(hex)?;
    let mut result = [0u8; 32];
    if hex.len() != result.len() {
        // TODO: formalize an error
        return Err(ClapError::with_description(
            &format!("Expected a hex with {} bytes, received {} bytes", result.len(), hex.len()),
            NOT_VALID,
        ));
    }
    result.copy_from_slice(&hex);
    Ok(result)
}

pub fn parse_hex64(hex: &str) -> Result<[u8; 64], ClapError> {
    let hex = parse_hex(hex)?;
    let mut result = [0u8; 64];
    if hex.len() != result.len() {
        // TODO: formalize an error
        return Err(ClapError::with_description(
            &format!("Expected a hex with {} bytes, received {} bytes", result.len(), hex.len()),
            NOT_VALID,
        ));
    }
    result.copy_from_slice(&hex);
    Ok(result)
}

pub fn parse_hex(hex: &str) -> Result<Vec<u8>, ClapError> {
    let hex = if "0x" == hex.chars().take(2).collect::<String>().as_str() { hex.chars().skip(2).collect() } else { hex.to_owned() };
    hex.from_hex().map_err(|e| ClapError::with_description(&format!("Failed parsing the value as hex, err: {}", e), NOT_VALID))
}
