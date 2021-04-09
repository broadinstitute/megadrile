extern crate clap;

use megadrile::{error, commands};
use megadrile::config;

fn evaluate_args() -> Result<(), error::Error> {
    let arg_matches = config::get_cli_config();
    if let (sub_name, Some(sub_matches)) = arg_matches.subcommand() {
        match sub_name {
            config::SUB_COMMAND_NAME_COUNTS =>
                commands::print_counts(sub_matches),
            config::SUB_COMMAND_NAME_LIST_VARIANTS =>
                commands::write_list_of_variants(sub_matches),
            config::SUB_COMMAND_NAME_LIST_SAMPLES =>
                commands::write_list_of_samples(sub_matches),
            &_ => {
                let message =
                    format!("Unknown subcommand {}. Use '--help' to get list.", sub_name);
                Err(error::Error::from(message))
            }
        }
    } else {
        Err(error::Error::from("Missing subcommand. Use '--help' to get list."))
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
