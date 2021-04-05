extern crate clap;

use megadrile::{stats, error};
use megadrile::config;

fn evaluate_args() -> Result<(), error::Error> {
    let arg_matches = config::get_cli_config();
    if let (sub_name, Some(sub_matches)) = arg_matches.subcommand() {
        match sub_name {
            config::SUB_COMMAND_NAME_COUNTS => {
                let counts = stats::try_get_counts(sub_matches)?;
                println!("Number of samples: {}.", counts.n_samples);
                println!("Number of records: {}.", counts.n_records);
                Ok(())
            }
            config::SUB_COMMAND_NAME_LIST_VARIANTS => {
                Err(error::Error::from("Listing variants - not yet implemented."))
            }
            &_ => {
                Err(error::Error::from(format!("Unknown subcommand {}.", sub_name)))
            }
        }
    } else {
        Err(error::Error::from("Missing subcommand."))
    }
}

fn main() {
    match evaluate_args() {
        Ok(_) => {
            println!("ok")
        }
        Err(error) => {
            println!("Error: {}", error.message())
        }
    }
    println!("Done!");
}
