use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "assembly_94.pest"]
struct AssemblyParser;

fn main() {
    let warrior = fs::read_to_string("warriors/imp.red").expect("Could not read input file");

    println!("{}", warrior);
    
    let mut warrior = AssemblyParser::parse(Rule::assembly_file, &warrior).unwrap_or_else(|e| panic!("{}", e));

    print_rules(warrior.next().unwrap(), 0);
}

fn print_rules(pair: Pair<Rule>, level: u32) {
    for _ in 0..level {
        print!(" ");
    }

    print!("{:?}", pair.as_rule());

    match pair.as_rule() {
        Rule::comment => println!(": {}", pair.as_str()),
        Rule::instruction => println!(": {}", pair.as_str()),
        _ => println!()
    }

    for inner in pair.into_inner() {
        print_rules(inner, level + 1);
    }
}