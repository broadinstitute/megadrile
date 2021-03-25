extern crate clap;
use megadrile::get_cli_config;

fn main() {
    let cli_config = get_cli_config();
    match cli_config.value_of("input") {
        Some(input) => println!("Input: {}", input),
        None => println!("No input specified.")
    }
    println!("Done!");
}
