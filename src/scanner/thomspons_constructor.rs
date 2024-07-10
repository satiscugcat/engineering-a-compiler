use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::Hash;

use super::regex_parser::*;

type State = u32;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Character{
    Char(char),
    Epsilon
}

fn state_generator() -> impl FnMut() ->  State{
    let mut count: State= 0;
    move || -> State {count +=1; count}
}

#[derive(Debug)]
pub struct NFA{
    states: HashSet<State>,
    characters: HashSet<Character>,
    transition_function: HashMap<(State, Character), HashSet<State>>,
    start_state: State,
    accepting_state: State
}

impl NFA{
    fn new() -> NFA{
        NFA{
            states: HashSet::new(),
            characters: HashSet::new(),
            transition_function: HashMap::new(),
            start_state: 0,
            accepting_state: 0
        }
    }
}

pub fn thomspons_constructor(instructions: Vec<OpCode>) -> NFA{
    let mut f = state_generator();
    let mut nfa_stack: Vec<NFA> = Vec::new();
    for instruction in instructions.iter(){
        match instruction {
            OpCode::LoadCharacter(character) => make_primary_nfa(*character, &mut nfa_stack, &mut f),
            OpCode::Union => nfa_union(&mut nfa_stack, &mut f),
            OpCode::Concatenate => nfa_concatenate(&mut nfa_stack),
            OpCode::Kleene => nfa_kleene(&mut nfa_stack, &mut f)
        }
    }
    nfa_stack.pop().expect("thompsons_constructor: Expected to pop return value")
}





fn make_primary_nfa<F>(character: char, nfa_stack: &mut Vec<NFA>, f: &mut F) where F: FnMut() -> State{

    let mut new_nfa = NFA::new();
    let start_state = f();
    let accepting_state = f();

    new_nfa.states.insert(start_state);
    new_nfa.states.insert(accepting_state);

    new_nfa.characters.insert(Character::Char(character));

    new_nfa.transition_function.insert((start_state, Character::Char(character)), HashSet::from([accepting_state]));

    new_nfa.start_state = start_state;
    new_nfa.accepting_state = accepting_state;

    nfa_stack.push(new_nfa);
}

fn nfa_union<F>(nfa_stack: &mut Vec<NFA>, f: &mut F) where F: FnMut() -> State{
    
    let nfa_2 = nfa_stack.pop().expect("nfa_union: Can't pop when expected.");
    let nfa_1 = nfa_stack.pop().expect("nfa_union: Can't pop when expected.");

    let mut new_nfa = NFA::new();

    let new_start_state = f();
    let new_accepting_state = f();

    new_nfa.states = nfa_1.states.union(&nfa_2.states).copied().collect();
    new_nfa.states.insert(new_start_state);
    new_nfa.states.insert(new_accepting_state);

    let start_state_1 = nfa_1.start_state;
    let start_state_2 = nfa_2.start_state;

    new_nfa.characters = nfa_1.characters.union(&nfa_2.characters).copied().collect();
    let accepting_state_1 = nfa_1.accepting_state;
    let accepting_state_2 = nfa_2.accepting_state;

    for (k,v) in nfa_1.transition_function.into_iter(){
        new_nfa.transition_function.insert(k,v);
    }
    for (k,v) in nfa_2.transition_function.into_iter(){
        new_nfa.transition_function.insert(k,v);
    }


    new_nfa.transition_function.insert((new_start_state, Character::Epsilon), HashSet::from([start_state_1, start_state_2]));  
    new_nfa.transition_function.insert((accepting_state_1, Character::Epsilon), HashSet::from([new_accepting_state]));  
    new_nfa.transition_function.insert((accepting_state_2, Character::Epsilon), HashSet::from([new_accepting_state]));  

    new_nfa.start_state = new_start_state;
    new_nfa.accepting_state = new_accepting_state;

    nfa_stack.push(new_nfa);
      
}

fn nfa_concatenate(nfa_stack: &mut Vec<NFA>){
    let nfa_2 = nfa_stack.pop().expect("nfa_concatenate: Can't pop when expected.");
    let nfa_1 = nfa_stack.pop().expect("nfa_concatenate: Can't pop when expected.");

    let mut new_nfa = NFA::new();
    

    new_nfa.states = nfa_1.states.union(&nfa_2.states).copied().collect();

    let new_start_state = nfa_1.start_state;

    let new_accepting_state = nfa_2.accepting_state;

    new_nfa.characters = nfa_1.characters.union(&nfa_2.characters).copied().collect();

    for (k,v) in nfa_1.transition_function.into_iter(){
        new_nfa.transition_function.insert(k,v);
    }
    for (k,v) in nfa_2.transition_function.into_iter(){
        new_nfa.transition_function.insert(k,v);
    }

    
    new_nfa.transition_function.insert((nfa_1.accepting_state, Character::Epsilon), HashSet::from([nfa_2.start_state]));  

    new_nfa.start_state = new_start_state;
    new_nfa.accepting_state = new_accepting_state;

    nfa_stack.push(new_nfa);
}

fn nfa_kleene<F>(nfa_stack: &mut Vec<NFA>, f: &mut F) where F: FnMut() -> State{
    let old_nfa = nfa_stack.pop().expect("nfa_kleene: Can't pop when expected.");

    let mut new_nfa = NFA::new();

    let new_start_state = f();
    let new_accepting_state = f();

    new_nfa.characters = old_nfa.characters;
    new_nfa.states = old_nfa.states;
    new_nfa.states.insert(new_start_state);
    new_nfa.states.insert(new_accepting_state);

    new_nfa.transition_function = old_nfa.transition_function;

    new_nfa.transition_function.insert((new_start_state, Character::Epsilon), HashSet::from([old_nfa.start_state, new_accepting_state]));
    new_nfa.transition_function.insert((old_nfa.accepting_state, Character::Epsilon), HashSet::from([old_nfa.start_state, new_accepting_state]));

    new_nfa.start_state = new_start_state;
    new_nfa.accepting_state = new_accepting_state;

    nfa_stack.push(new_nfa);
}