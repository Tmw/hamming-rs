# Hamming coding

A (15, 11) [hamming coding](https://en.wikipedia.org/wiki/Hamming_code) implementation written in Rust as an exercise as inspired by [3Blue1Brown's video](https://www.youtube.com/watch?v=X8jsijhllIA) about the topic.

## Library
> tbd.

## Cli

Hamming is primarily a Cli tool that will take some data from either direct input or stdin and perform hamming encoding on it. By default it will print to stdout a base64 encoded string. However passing the `--raw` flag will print it without base64 encoding.

### Grab & build
```bash
git clone
cargo build --release
./target/release/hamming --help
```
### Run
**Main help**
```plaintext
Hamming 
Encode and decode bytes using Hamming
USAGE:
    hamming <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decode    Decode a given hamming value; reads from STDIN    encode    Encode a given value, either as argument or reading from STDIN
    help      Prints this message or the help of the given subcommand(s)
```

**Encode**
```plaintext
hamming-encode 
Encode a given value, either as argument or reading from STDIN
USAGE:    hamming encode [FLAGS] [input]
FLAGS:
    -h, --help       Prints help information
    -r, --raw        Writes raw output (no base64 encoding)
    -V, --version    Prints version information
ARGS:    <input>    Input string to encode
```

**Decode**
```plaintext
hamming-decode 
Decode a given hamming value; reads from STDIN
USAGE:
    hamming decode [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -r, --raw        Reads raw input (non base64 encoded string)
    -V, --version    Prints version information
```

### Example
Encoding it to base64 output
```bash
hamming encode "this is an example"
b6MMmiVmLAY5OXGBeC0coA4rVBglWnYGVLJwAA==
```

Back to back
```bash
hamming encode "this is an example"  | hamming decode
this is an example
```

## License
[MIT](./LICENSE)
