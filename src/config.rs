use clap::{ArgMatches, App, Arg, SubCommand};

pub const SUB_COMMAND_NAME_COUNTS: &str = "counts";
pub const SUB_COMMAND_NAME_LIST_VARIANTS: &str = "list_variants";

pub const ARG_NAME_INPUT: &str = "input";
pub const ARG_NAME_OUTPUT: &str = "output";

fn create_input_arg<'a, 'b>() -> Arg<'a,'b> {
    Arg::with_name(ARG_NAME_INPUT)
        .short("i")
        .long("input")
        .value_name("INPUT")
        .help("The input file")
        .takes_value(true)
}

fn create_output_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(ARG_NAME_OUTPUT)
        .short("o")
        .long("output")
        .value_name("OUTPUT")
        .help("The output file")
        .takes_value(true)
}

pub fn get_cli_config<'a>() -> ArgMatches<'a> {
    App::new("Megadrile")
        .version("0.1.0")
        .author("Oliver Ruebenacker <oliverr@broadinstitute.org>")
        .about("Crunching genomic data")
        .subcommand(
            SubCommand::with_name(SUB_COMMAND_NAME_COUNTS)
                .about("Counting samples and records.")
                .arg(create_input_arg())
        )
        .subcommand(
            SubCommand::with_name(SUB_COMMAND_NAME_LIST_VARIANTS)
                .about("Create a list of variants.")
                .arg(create_input_arg())
                .arg(create_output_arg())
        )
        .get_matches()
}

