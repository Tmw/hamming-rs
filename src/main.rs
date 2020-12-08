use anyhow::bail;
use clap::ArgMatches;
use std::io::{Read, Write, stdin, stdout};

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

    let blocks: Blocks = Blocks::new(&input[..], false);
    let out = blocks.to_byte_vec();
    stdout().write_all(out.as_slice())?;
    Ok(())
}

fn decode(args: &ArgMatches) -> anyhow::Result<()> {
    let input = parse_input(args)?;
    let mut blocks: Blocks = Blocks::new(&input[..], true);
    &blocks.repair();
    println!("{}", blocks.to_string());

    Ok(())
}
