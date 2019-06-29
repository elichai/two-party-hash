use crate::utils::{parse_hex32, parse_hex64};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "TwoPartyHash", about = "A tool that let's 2 parties agree on one preimage of an hash without revealing secrets.")]
pub enum Cli {
    #[structopt(name = "first")]
    First,
    #[structopt(name = "second")]
    Second {
        #[structopt(parse(try_from_str = "parse_hex32"), raw(required = "true"))]
        mid_hash: [u8; 32],
    },
    #[structopt(name = "verify")]
    Verify {
        #[structopt(parse(try_from_str = "parse_hex64"), raw(required = "true"))]
        first_preimage: [u8; 64],
        #[structopt(parse(try_from_str = "parse_hex32"), raw(required = "true"))]
        second_preimage: [u8; 32],
    },
}
