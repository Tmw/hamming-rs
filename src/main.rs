use hamming::blocks::Blocks;
use std::io::{Read, Write};
mod cli;

fn main() -> Result<(), std::io::Error> {
    match cli::setup().subcommand() {
        ("encode", Some(args)) => CLI::new(args).map(|cli| cli.encode()),
        ("decode", Some(args)) => CLI::new(args).map(|cli| cli.decode()),
        _ => None,
    };

    Ok(())
}

struct CLI {
    input: Vec<u8>,
}

// TODO: Have proper error handling here?
impl CLI {
    fn new(args: &clap::ArgMatches) -> Option<Self> {
        if args.is_present("input") {
            let input = args
                .value_of("input")
                .expect("expected input to have a value");

            return Some(CLI {
                input: input.as_bytes().to_owned(),
            });
        }

        // input was not passed, attempt to read from stdin
        let mut buff: Vec<u8> = Vec::new();
        std::io::stdin()
            .read_to_end(&mut buff)
            .expect("Error reading from STDIN");

        Some(CLI { input: buff })
    }

    fn encode(&self) {
        let blocks: Blocks = Blocks::new(&self.input[..], false);
        let out = blocks.to_byte_vec();
        std::io::stdout()
            .write_all(out.as_slice())
            .expect("Error writing to STDOUT")
    }

    fn decode(&self) {
        let mut blocks: Blocks = Blocks::new(&self.input[..], true);
        &blocks.repair();
        println!("{}", blocks.to_string());
    }
}
