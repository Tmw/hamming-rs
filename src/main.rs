use hamming::blocks::Blocks;

fn main() {
    let input = get_input();

    // initial loading data in Hamming code and assign parity
    let mut blocks: Blocks = Blocks::from(&input[..]);
    println!("Before noise:\t {}", &blocks.to_string());

    // introduce noise randomly to the blocks and see its output
    blocks.introduce_noise();
    println!("After noise:\t {}", &blocks.to_string());

    // then repair using the parity
    blocks.repair();
    println!("Repaired:\t {}", &blocks.to_string());
}

fn get_input() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or(String::from("This is a sample string"))
}
