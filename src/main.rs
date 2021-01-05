use anyhow::{bail, Context};
use clap::ArgMatches;
use std::io::{stdin, stdout, Read, Write};

use hamming::blocks::Blocks;
mod cli;

fn main() -> anyhow::Result<()> {
    match cli::get_matches().subcommand() {
        ("encode", Some(args)) => encode(args),
        ("decode", Some(args)) => decode(args),
        _ => bail!("command not recognized"),
    }
}

fn stdin_input() -> anyhow::Result<Vec<u8>> {
    let mut buff: Vec<u8> = Vec::new();
    stdin().read_to_end(&mut buff)?;

    Ok(buff)
}

fn parse_input(args: &ArgMatches) -> anyhow::Result<Vec<u8>> {
    if let Some(input) = args.value_of("input") {
        return Ok(input.as_bytes().to_owned());
    }

    stdin_input()
}

fn encode(args: &ArgMatches) -> anyhow::Result<()> {
    let input = parse_input(args)?;
    let mut blocks: Blocks = Blocks::new(&input[..], false);
    blocks.prepare();
    let out = blocks.to_byte_vec();

    match args.is_present("raw") {
        true => stdout().write_all(out.as_slice())?,
        false => print!("{}", base64::encode(out.as_slice()))

    };

    Ok(())
}

fn decode(args: &ArgMatches) -> anyhow::Result<()> {
    let input = parse_input(args)?;

    let input = match args.is_present("raw") {
        true => input,
        false => base64::decode(input)
            .context("Unable to decode base64 input. Are you sure input is base64 encoded? Tip: Try passing --raw flag")?,
    };

    let mut blocks: Blocks = Blocks::new(&input[..], true);
    &blocks.repair();
    println!("{}", blocks.to_string());

    Ok(())
}
