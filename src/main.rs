use megadrile::{commands, config, error::Error};

fn evaluate_args() -> megadrile::Result<()> {
    let arg_matches = config::get_cli_config();
    if let (sub_name, Some(sub_matches)) = arg_matches.subcommand() {
        match sub_name {
            config::SUB_COMMAND_NAME_COUNTS => commands::print_counts(sub_matches),
            config::SUB_COMMAND_NAME_LIST_VARIANTS => commands::write_list_of_variants(sub_matches),
            config::SUB_COMMAND_NAME_LIST_SAMPLES => commands::write_list_of_samples(sub_matches),
            config::SUB_COMMAND_NAME_MAF => commands::calculate_maf(sub_matches),
            &_ => {
                let message = format!("Unknown subcommand {}. Use '--help' to get list.", sub_name);
                Err(Error::from(message))
            }
        }
    } else {
        Err(Error::from("Missing subcommand. Use '--help' to get list."))
    }
}

fn main() {
    match evaluate_args() {
        Ok(_) => {
            println!("ok")
        }
        Err(error) => {
            println!("Error: {}", error)
        }
    }
    println!("Done!");
}
