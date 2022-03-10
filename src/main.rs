use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod assembler;
use assembler::Assembly;

fn main() {
    let warrior = fs::read_to_string("warriors/dwarf.red").expect("Could not read input file");

    println!("{}", warrior);

    println!("====================");

    let assembly = Assembly::from(warrior);

    for instruction in assembly.code {
        println!("{:?}", instruction);
    }
}