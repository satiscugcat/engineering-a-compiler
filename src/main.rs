mod scanner;
use scanner::regex_parser;
use scanner::thomspons_constructor;
fn main() {
    let instructions = regex_parser::regex_parser("a(b|c)*(c|d)*");
    let nfa = thomspons_constructor::thomspons_constructor(instructions);

    println!("{:?}", nfa);
}

