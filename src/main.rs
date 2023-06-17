mod blockchain;

fn main() {
    let mut chain = blockchain::Blockchain::new();

    // Add some blocks to the chain
    chain.add_block("Block 1".to_string());
    chain.add_block("Block 2".to_string());
    chain.add_block("Block 3".to_string());

    // Print the blockchain
    println!("{:#?}", chain);
}