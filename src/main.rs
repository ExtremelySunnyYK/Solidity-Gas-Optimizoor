mod lexer;

use crate::lexer::lexer::*;
use crate::lexer::printer::*;

fn main() {
    let struct_contract_source =
        include_str!("../examples/unoptimized_contracts/struct_packing.sol");

    let tokens = tokenize(struct_contract_source);
    let output = generate_output(tokens);
    println!("{}", output);
}
