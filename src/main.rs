mod config;

extern crate clap;
use std::fs::File;

fn main() {
    let cli_config = config::get_cli_config();
    match cli_config.value_of("input") {
        Some(input) => {
            let result = File::open(input);
            match result {
                Ok(_) => println!("Got file"),
                Err(err) => println!("Got error trying to open file: {}", err)
            }
            println!("Input: {}", input)
        },
        None =>
            println!("No input specified.")
    }
    println!("Done!");
}
