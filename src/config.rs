use clap::{ArgMatches, App, Arg};

pub fn get_cli_config<'a>() -> ArgMatches<'a> {
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
        .get_matches()
}

