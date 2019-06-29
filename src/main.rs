mod cli;
mod sha2;


use cli::Cli;
use structopt::StructOpt;
use structopt::clap::{Error as ClapError, ErrorKind as ClapErrorKind};

fn main() {
    println!("Hello, world!");
    let cli = Cli::from_args();
    if let Err(e) = handle_cli(cli) {
        e.exit();
    }
}


fn handle_cli(cli: Cli) -> Result<(), ClapError> {
    match cli {
        Cli::First { preimage } => {

        }
    }

}
