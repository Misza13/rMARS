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
    pub code: Vec<Instruction>,
    pub origin: u32
}

#[derive(Debug)]
struct PreprocessorLine {
    instruction_rule: Option<Rule>,
    labels: Vec<String>
}

struct Preprocessor {
    lines: Vec<PreprocessorLine>,
    origin: u32,
    index: usize
}

impl Assembly {
    pub fn from(code: String) -> Assembly {
        let assembly_file = AssemblyParser::parse(Rule::assembly_file, &code)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

        let mut preprocessor = Preprocessor::new();

        for line in assembly_file.into_inner() {
            process_line(line, &mut preprocessor);
        }

        println!("====================");

        for line in preprocessor.lines {
            println!("{:?}", line);
        }

        Assembly { code: vec!(), origin: 0 }
    }
}

impl PreprocessorLine {
    fn new() -> PreprocessorLine {
        PreprocessorLine {
            instruction_rule: None,
            labels: vec!()
        }
    }

    fn push_label(&mut self, label: String) {
        self.labels.push(label);
    }
}

impl Preprocessor {
    fn new() -> Preprocessor {
        Preprocessor {
            lines: vec!(),
            origin: 0,
            index: 0
        }
    }

    fn process_label(&mut self, label: String) {
        self.ensure_line();
        self.lines.last_mut().unwrap().push_label(label.to_string());
    }

    fn process_instruction(&mut self, rule: Rule) {
        self.ensure_line();
        self.lines.last_mut().unwrap().instruction_rule = Some(rule);
        self.index += 1;
        //TODO: ORG, END, EQU
    }

    fn ensure_line(&mut self) {
        if self.index == self.lines.len() {
            self.lines.push(PreprocessorLine::new());
        }
    }
}

macro_rules! invalid_rule {
    ($rule: expr, $parent: literal) => {
        unreachable!("Invalid rule within [{}]: {:?}", $parent, $rule.as_rule())
    };
}

fn process_line(line: Pair<Rule>, preprocessor: &mut Preprocessor) {
    match line.as_rule() {
        Rule::instruction_line => process_instruction_line(line, preprocessor),
        Rule::label_line => process_label_line(line, preprocessor),
        Rule::comment_line => process_comment_line(line, preprocessor),
        Rule::indented_comment_line => {},
        Rule::empty_line => {},
        Rule::EOI => {},
        _ => invalid_rule!(line, "line")
    }
}

fn process_instruction_line(instruction_line: Pair<Rule>, preprocessor: &mut Preprocessor) {
    for inner in instruction_line.into_inner() {
        match inner.as_rule() {
            Rule::label => {
                preprocessor.process_label(inner.as_str().to_string());
                println!("Label: {}", inner.as_str());
            },
            Rule::instruction => {
                preprocessor.process_instruction(inner.as_rule());
                println!("Instruction: {}", inner.as_str());
            },
            Rule::instruction_line => process_instruction_line(inner, preprocessor),
            Rule::comment => {},
            _ => invalid_rule!(inner, "instruction_line")
        }
    }
}

fn process_label_line(label_line: Pair<Rule>, preprocessor: &mut Preprocessor) {
    for inner in label_line.into_inner() {
        match inner.as_rule() {
            Rule::label => {
                preprocessor.process_label(inner.as_str().to_string());
                println!("Label: {}", inner.as_str());
            },
            _ => invalid_rule!(inner, "label_line")
        }
    }
}

fn process_comment_line(comment_line: Pair<Rule>, preprocessor: &mut Preprocessor) {
    println!("{}", comment_line.as_str());
}