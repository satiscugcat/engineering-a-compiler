#[derive(Debug)]
pub enum OpCode{
    LoadCharacter(char),
    Union,
    Concatenate,
    Kleene
}

type InputString<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub fn regex_parser(regex: &str) -> Vec<OpCode>{
    let mut instructions : Vec<OpCode> = Vec::new();
    let mut input_stream = regex.chars().peekable() as InputString;
    union(&mut instructions, &mut input_stream);
    
    instructions

}


fn union(instructions: &mut Vec<OpCode>, input_stream: &mut InputString){
    concatenation(instructions, input_stream);

    while match input_stream.peek() {
        Some(t) => *t == '|', 
        None => false
    } {
        input_stream.next();
        concatenation(instructions, input_stream);
        instructions.push(OpCode::Union);
    }
}

fn concatenation(instructions: &mut Vec<OpCode>, input_stream: &mut InputString){
    primary(instructions, input_stream);

    while match input_stream.peek() {
        Some(t) => *t != '|' && *t!=')', 
        None => false
    }{
        primary(instructions, input_stream);
        instructions.push(OpCode::Concatenate);
    }
}

fn primary(instructions: &mut Vec<OpCode>, input_stream: &mut InputString){
    if match input_stream.peek() {
        Some(t) => *t == '(', 
        None => return
    }{
        input_stream.next();
        grouping(instructions, input_stream);
    } else {
        instructions.push(OpCode::LoadCharacter(*(input_stream.peek().unwrap())));
        input_stream.next();
    }

    if match input_stream.peek() {
        Some(t) => *t == '*', 
        None => false
    }{
        input_stream.next();
        instructions.push(OpCode::Kleene);
    }
}

fn grouping(instructions: &mut Vec<OpCode>, input_stream: &mut InputString){
    union(instructions, input_stream);
    if !match input_stream.peek() {
        Some(t) => *t == ')', 
        None => false
    } {
        panic!("Expect closing ')' after '('");
    } else {
        input_stream.next();
    }
}