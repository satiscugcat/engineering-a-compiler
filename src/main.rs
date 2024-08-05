mod scanner;
use scanner::*;

fn main() {
    let instructions = regex_parser::regex_parser("(a|b|c)*");
    let nfa = thomspons_constructor::thomspons_constructor(instructions);

    println!("{:?}", nfa);
}

