extern crate clap;
use clap::{Arg, App};

fn main() {
    let cli_config =
        App::new("Megadrile")
            .version("0.1.0")
            .author("Oliver Ruebenacker <oliverr@broadinstitute.org>")
            .about("Crunching genomic data")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT")
                .help("The input file")
                .takes_value(true))
            .get_matches();
    match cli_config.value_of("input") {
        Some(input) => println!("Input: {}", input),
        None => println!("No input specified.")
    }
    println!("Done!");
}
