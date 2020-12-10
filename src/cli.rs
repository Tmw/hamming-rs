use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("Hamming")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Encode and decode bytes using Hamming")
        .subcommand(encode_command())
        .subcommand(decode_command())
        .get_matches()
}

fn encode_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("encode")
        .about("Encode a given value, either as argument or reading from STDIN")
        .arg(Arg::with_name("input").help("Input string to encode"))
        .arg(
            Arg::with_name("raw")
                .short("r")
                .long("raw")
                .help("Writes raw output (no base64 encoding)"),
        )
}

fn decode_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("decode")
        .about("Decode a given hamming value; reads from STDIN")
        .arg(
            Arg::with_name("raw")
                .short("r")
                .long("raw")
                .help("Reads raw input (non base64 encoded string)"),
        )
}
