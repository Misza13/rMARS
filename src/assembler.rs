use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "assembly_94.pest"]
struct AssemblyParser;

#[path = "rvm.rs"]
#[allow(dead_code)]
mod rvm;
use rvm::Instruction;

pub struct Assembly {
    pub code: Vec<Instruction>
}

impl Assembly {
    pub fn assemble_from(code: String) -> Assembly {
        let assembly_file = AssemblyParser::parse(Rule::assembly_file, &code)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

        for line in assembly_file.into_inner() {
            process_line(line);
        }

        Assembly { code: vec!() }
    }
}

macro_rules! invalid_rule {
    ($rule: expr, $parent: literal) => {
        unreachable!("Invalid rule within [{}]: {:?}", $parent, $rule.as_rule())
    };
}


fn process_line(line: Pair<Rule>) {
    match line.as_rule() {
        Rule::instruction_line => process_instruction_line(line),
        Rule::label_line => process_label_line(line),
        Rule::comment_line => process_comment_line(line),
        Rule::indented_comment_line => {},
        Rule::empty_line => {},
        Rule::EOI => {},
        _ => invalid_rule!(line, "line")
    }
}

fn process_instruction_line(instruction_line: Pair<Rule>) {
    for inner in instruction_line.into_inner() {
        match inner.as_rule() {
            Rule::label => println!("Label: {}", inner.as_str()),
            Rule::instruction => println!("Instruction: {}", inner.as_str()),
            Rule::instruction_line => process_instruction_line(inner),
            Rule::comment => {},
            _ => invalid_rule!(inner, "instruction_line")
        }
    }
}

fn process_label_line(label_line: Pair<Rule>) {
    for inner in label_line.into_inner() {
        match inner.as_rule() {
            Rule::label => println!("Label: {}", inner.as_str()),
            _ => invalid_rule!(inner, "label_line")
        }
    }
}

fn process_comment_line(comment_line: Pair<Rule>) {
    println!("{}", comment_line.as_str());
}