pub mod helpers;


// Starter code for PS06 - thegrep
// Add Honor Code Header and Collaborators Here

/**
 4  * thegrep - Tar Heel egrep
 5  *
 6  * Author(s): Sarah Bost, Shannon Goad
 7  * ONYEN(s): sbost99, sgoad13
 8  *
 9  * UNC Honor Pledge: I pledge I have received no unauthorized aid
10  * on this assignment. I further pledge not to distribute my solution
11  * to this code to anyone other than the course staff and partner.
12  */
extern crate rand; 

use self::State::*;
use super::parser::Parser;
use super::parser::AST;
use super::tokenizer::Tokenizer;
use std::iter::Peekable;
use std::ops::Add;
use rand::prelude::*;
use rand::distributions::Alphanumeric; 

/**
 * ===== Public API =====
 */

/**
 * An NFA is represented by an arena Vec of States
 * and a start state.
 */
#[derive(Debug)]
pub struct NFA {
    start: StateId,
    states: Vec<State>,
}

impl NFA {
    /**
     * Construct an NFA from a regular expression pattern.
     */
    pub fn from(regular_expression: &str) -> Result<NFA, String> {
        let mut nfa = NFA::new();

        let start = nfa.add_state(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add_state(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    pub fn random_regex(&self) -> String {
        let mut ran: String = "".to_string();  
        self.random_regex_traverse(self.start, ran)
    }

    pub fn random_regex_traverse( &self, position: StateId, mut ran: String) -> String {
        
        match &self.states[position] {
            Start(state_id) => {
                self.random_regex_traverse(state_id.unwrap(), ran) 

            }
            Split(lhs, rhs) => {
     let mut rng = rand::thread_rng();
     let direction: bool = rng.gen(); 
 

                if direction {
                    return self.random_regex_traverse(lhs.unwrap(), ran); 
                } else {
                    return self.random_regex_traverse(rhs.unwrap(), ran); 
                }
            }
            Match(character, state_id) => {
                match &character {
                    Char::Any => {
                        //let rand_string = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
                        let mut random = rand::thread_rng(); 
                        let ch: u32 = random.gen(); 
                        ran.push(ch); 
                    }, 
                    Char::Literal(ch)=> {
                        ran.push(*ch);
                    },
                }
                self.random_regex_traverse(state_id.unwrap(), ran) 
            }
            End => {
                return ran; 
            }
        }
    
    }

    /**
     * Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     */
    pub fn accepts(&self, input: &str) -> bool {
        // vector of chars formed from input so that we can iterate over them in accepts()
        let input_chars = input.chars().collect();
        // the final "false" here is a bool that will be useful later to see if the nfa has already
        // been started being read in the input
        
        if input.len() == 0 {
           return  self.traverse(&input_chars, 0, self.start, false);
        }
        
        let mut counter = 0;
        for inp in &input_chars {
            if self.traverse(&input_chars, counter, self.start, false) {
                return true;
            }
            counter = counter + 1;
        }
        return false;
    }

    pub fn traverse(
        &self,
        chars: &Vec<char>,
        chars_index: usize,
        start_state_id: StateId,
        has_started_nfa: bool,
    ) -> bool {
//        println!("{:?}", self.states); 
        // we are matching by what state the nfa (regex) is on in our traversal
        match &self.states[start_state_id] {
            Start(state_id) => {
                return self.traverse(&chars, chars_index, state_id.unwrap(), has_started_nfa)
            }
            // start state: just traverse to the next state by moving to next state_id
            Split(lhs, rhs) => {
                // split state: check both sides and traverse whichever one is necessary
                if self.traverse(&chars, chars_index, lhs.unwrap(), has_started_nfa) {
                    return true;
                }
                if self.traverse(&chars, chars_index, rhs.unwrap(), has_started_nfa) {
                    return true;
                }
                return false;
            }

            Match(character, state_id) => {
                // if we reach end of input chars before finding regex, return false
                if chars_index == chars.len() {
                    return false;
                }
                // see if the input char matches the NFA regex char that we expect
                match character {
                    Char::Literal(c) => {
                        if c == &chars[chars_index] {
                            return self.traverse(&chars, chars_index + 1, state_id.unwrap(), true);
                        } else {
                            if has_started_nfa {
                                return self.traverse(&chars, chars_index, self.start, false);
                            }
                            return self.traverse(
                                &chars,
                                chars_index + 1,
                                start_state_id,
                                has_started_nfa,
                            );
                        }
                    }
                    Char::Any => {
                        return self.traverse(
                            &chars,
                            chars_index + 1,
                            state_id.unwrap(),
                            has_started_nfa,
                        );
                    }
                };
            }
            End => {
                // end state: return true because it will only get here if it's a match!
                return true;
            }
        };
    }
}

impl Add for NFA {
    type Output = Self;

    fn add(self, rhs: NFA) -> NFA {
        let mut concat = NFA::new();
        let firstlength = self.states.len() - 1;
        let mut lhs_end = firstlength;
        let mut rhs_start = firstlength - 1;
        for i in 0..firstlength {
            match &self.states[i] {
                State::Start(n) => { 
                    let start = concat.add_state(Start(Some(n.unwrap()))); 
                    concat.start = start;
                },
                State::Match(c, n) => { concat.add_state(Match(*c, Some(n.unwrap()))); },
                State::Split(n, m) => { concat.add_state(Split(Some(n.unwrap()), Some(m.unwrap()))); },
                State::End => { lhs_end = i; },
            };
        }
        for s in &rhs.states {
            match s {
                State::Start(n) => { rhs_start = n.unwrap() + rhs_start - 1; },
                State::Match(c, n) => { concat.add_state(Match(*c, Some(n.unwrap() + firstlength - 1))); },
                State::Split(n, m) => { concat.add_state(Split(Some(n.unwrap() + firstlength - 1), Some(m.unwrap() + firstlength - 1))); },
                State::End => { concat.add_state(End); },
            };
        }
        for i in 0..concat.states.len() {
            match &concat.states[i] {
                State::Match(c, n) => { 
                    if n.unwrap() == lhs_end { concat.states[i] = Match(*c, Some(rhs_start + 1)); }
                },
                State::Split(n, m) => {
                    if n.unwrap() == lhs_end && m.unwrap() == lhs_end { concat.states[i] = Split(Some(rhs_start), Some(rhs_start)); }
                    // else if m.unwrap() == lhs_end { concat.states[i] = Split(*n, Some(rhs_start + 1)); }
                    // else if n.unwrap() == lhs_end { concat.states[i] = Split(Some(rhs_start + 1), *m); }
                },
                _ => { },
            }
        }
        println!("{:?}", concat.states);
        concat
    }
}   

/**
 * ===== Internal API =====
 */
type StateId = usize;

/**
 * States are the elements of our NFA Graph
 * - Start is starting state
 * - Match is a state with a single matching transition out
 * - Split is a state with two epsilon transitions out
 * - End is the final accepting state
 */
#[derive(Debug)]
enum State {
    Start(Option<StateId>),
    Match(Char, Option<StateId>),
    Split(Option<StateId>, Option<StateId>),
    End,
}

/**
 * Chars are the matching label of a non-epsilon edge in the
 * transition diagram representation of the NFA.
 */
#[derive(Debug, Copy, Clone)]
enum Char {
    Literal(char),
    Any,
}

/**
 * Internal representation of a fragment of an NFA being constructed
 * that keeps track of the start ID of the fragment as well as all of
 * its unjoined end states.
 */
#[derive(Debug)]
struct Fragment {
    start: StateId,
    ends: Vec<StateId>,
}

/**
 * Private methods of the NFA structure.
 */
impl NFA {
    /**
     * Constructor establishes an empty states Vec.
     */
    fn new() -> NFA {
        NFA {
            states: vec![],
            start: 0,
        }
    }

    /**
     * Add a state to the NFA and get its arena ID back.
     */
    fn add_state(&mut self, state: State) -> StateId {
        let idx = self.states.len();
        self.states.push(state);
        idx
    }

    /**
     * Given an AST node, this method returns a Fragment of the NFA
     * representing it and its children.
     */
    fn gen_fragment(&mut self, ast: &AST) -> Fragment {
        // creates fragments of an NFA based on what AST they are
        match ast {
            AST::AnyChar => {
                let state = self.add_state(Match(Char::Any, None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Char(c) => {
                let state = self.add_state(Match(Char::Literal(*c), None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Catenation(lhs, rhs) => {
                let fragment_lhs = self.gen_fragment(lhs);
                let fragment_rhs = self.gen_fragment(rhs);
                self.join_fragment(&fragment_lhs, fragment_rhs.start);
                self.join_fragment(&fragment_rhs, fragment_rhs.start);

                Fragment {
                    start: fragment_lhs.start,
                    ends: fragment_rhs.ends,
                }
            }
            AST::Alternation(one, two) => {
                let fragment_one = self.gen_fragment(one);
                let fragment_two = self.gen_fragment(two);
                let split_state =
                    self.add_state(Split(Some(fragment_one.start), Some(fragment_two.start)));
                let mut v = vec![];
                v.extend(fragment_one.ends);
                v.extend(fragment_two.ends);
                Fragment {
                    start: split_state,
                    ends: v,
                }
            }
            AST::Closure(ast) => {
                let fragment_ast = self.gen_fragment(ast);
                let split_state = self.add_state(Split(Some(fragment_ast.start), None));
                self.join(split_state, fragment_ast.start);
                self.join(fragment_ast.start, split_state);

                Fragment {
                    start: split_state,
                    ends: vec![split_state],
                }
            }
            AST::OneOrMore(ast) => {
                //gets first regex
                let fragment_ast = self.gen_fragment(&ast);
                ////gets second regex 
                //let mut fragment_repeat = self.gen_fragment(&ast);
                let split_state = self.add_state(Split(Some(fragment_ast.start), None));
                self.join_fragment(&fragment_ast, split_state);

                Fragment {
                    start: fragment_ast.start,
                    ends: vec![split_state],
                }
            }
        }
    }

    /**
     * Join all the loose ends of a fragment to another StateId.
     */
    fn join_fragment(&mut self, lhs: &Fragment, to: StateId) {
        for end in &lhs.ends {
            self.join(*end, to);
        }
    }

    /**
     * Join a loose end of one state to another by IDs.
     * Note in the Split case, only the 2nd ID (rhs) is being bound.
     * It is assumed when building an NFA with these constructs
     * that the lhs of an Split state will always be known and bound.
     */
    fn join(&mut self, from: StateId, to: StateId) {
        match self.states[from] {
            Start(ref mut next) => *next = Some(to),
            Match(_, ref mut next) => *next = Some(to),
            Split(_, ref mut next) => *next = Some(to),
            End => {}
        }
    }
}
#[cfg(test)]
mod public_api {
    use super::*;

    #[test]
    fn test0() {
        let nfa = NFA::from("a").unwrap();
        assert_eq!(nfa.accepts("a"), true);
    }

    #[test]
    fn test1() {
        let nfa = NFA::from("sarah").unwrap();
        assert_eq!(nfa.accepts("ra"), false);
    }

    #[test]
    fn test2() {
        let nfa = NFA::from("a.*").unwrap();
        assert_eq!(nfa.accepts("abb"), true);
    }
    #[test]
    fn test3() {
        let nfa = NFA::from("hello").unwrap();
        assert_eq!(nfa.accepts("no"), false);
    }
    #[test]
    fn test4() {
        let nfa = NFA::from("aut....a").unwrap();
        assert_eq!(nfa.accepts("automata"), true);
    }
    #[test]
    fn test5() {
        let nfa = NFA::from("aut....a").unwrap();
        assert_eq!(nfa.accepts("asdfasdf"), false);
    }
    #[test]
    fn test6() {
        let nfa = NFA::from("aut....a").unwrap();
        assert_eq!(nfa.accepts("chautanqua"), true);
    }
    #[test]
    fn test7() {
        let nfa = NFA::from("etion").unwrap();
        assert_eq!(nfa.accepts("deletion"), true);
    }
    #[test]
    fn test8() {
        let nfa = NFA::from("etion").unwrap();
        assert_eq!(nfa.accepts("completion"), true);
    }
    #[test]
    fn test9() {
        let nfa = NFA::from("tool").unwrap();
        assert_eq!(nfa.accepts("toadstools"), true);
    }
    #[test]
    fn test10() {
        let nfa = NFA::from("bl.*").unwrap();
        assert_eq!(nfa.accepts("blue"), true);
    }
    #[test]
    fn test11() {
        let nfa = NFA::from("bl.*").unwrap();
        assert_eq!(nfa.accepts("reblock"), true);
    }
    #[test]
    fn test12() {
        let nfa = NFA::from("bl.*").unwrap();
        assert_eq!(nfa.accepts("dog"), false);
    }
    #[test]
    fn test13() {
        let nfa = NFA::from("b|rag").unwrap();
        assert_eq!(nfa.accepts("rag"), true);
        assert_eq!(nfa.accepts("bag"), true);
        assert_eq!(nfa.accepts("hag"), false);
    }
    #[test]
    fn test14() {
        let nfa = NFA::from("h|yell.*").unwrap();
        assert_eq!(nfa.accepts("hello"), true);
        assert_eq!(nfa.accepts("hell"), true);
        assert_eq!(nfa.accepts("yellowed"), true);
        assert_eq!(nfa.accepts("yell"), true);
        assert_eq!(nfa.accepts("asdfasdfasdfhell"), true);
        assert_eq!(nfa.accepts("bellow"), false);
    }
    #[test]
    fn test15() {
        let nfa = NFA::from("..*fee").unwrap();
        assert_eq!(nfa.accepts("coffee"), true);
        assert_eq!(nfa.accepts("fee"), false);
        assert_eq!(nfa.accepts("blahcoffeeblah"), true);
        assert_eq!(nfa.accepts("teefee"), true);
        assert_eq!(nfa.accepts("eeeeeeeee"), false);
    }
    #[test]
    fn test16() {
        let nfa = NFA::from("a..b").unwrap();
        assert_eq!(nfa.accepts("aaaaaaaaaaaaaaaaab"), true);
        assert_eq!(nfa.accepts("aaab"), true);
        assert_eq!(nfa.accepts("aaaaaab"), true);
    }

    #[test]
    fn test20() {
        let nfa = NFA::from("aaab").unwrap();
        assert_eq!(nfa.accepts("aaaab"), true);
    }

    #[test]
    fn test21() {
        let nfa = NFA::from(".*").unwrap();
        assert_eq!(nfa.accepts("bca"), true);
        assert_eq!(nfa.accepts(""), true);
    }

    #[test]
    fn test17() {
        let nfa = NFA::from("(a|o)(p|r).*").unwrap();
        assert_eq!(nfa.accepts("orange"), true);
        assert_eq!(nfa.accepts("apple"), true);
        assert_eq!(nfa.accepts("opple"), true);
        assert_eq!(nfa.accepts("aaaaaa"), false);
        assert_eq!(nfa.accepts("o"), false);
        assert_eq!(nfa.accepts("aaaaaaaaaaaaaaaaaaapple"), true);
        assert_eq!(nfa.accepts("prprprprprp"), false);
        assert_eq!(nfa.accepts(""), false);
    }
}
#[cfg(test)]
mod op_overload_test {
    use super::*;

    #[test]
    fn optest0() {
        let nfa1 = NFA::from("a").unwrap();
        let nfa2 = NFA::from("b").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("ab"), true);
    }   

    #[test] 
    fn optest1() {
        let nfa1 = NFA::from("a").unwrap();
        let nfa2 = NFA::from("a").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("mmmmmmaaaaaa"), true);
        assert_eq!(nfa.accepts("jaja"), false);
        assert_eq!(nfa.accepts("ab"), false);
    }
    #[test]
    fn optest2() {
        let nfa1 = NFA::from("ri").unwrap();
        let nfa2 = NFA::from("ha.*").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("rihanna"), true);
    }
    #[test]
    fn optest3() {
        let nfa1 = NFA::from("a|b").unwrap();
        let nfa2 = NFA::from("c|d").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("ac"), true);
        assert_eq!(nfa.accepts("bd"), true);
    }
    #[test]
    fn optest4() {
        let nfa1 = NFA::from("a*").unwrap();
        let nfa2 = NFA::from("c").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("ac"), true);
        assert_eq!(nfa.accepts("aaaaaaac"), true);
        assert_eq!(nfa.accepts("c"), true);
        assert_eq!(nfa.accepts("bbbb"), false);
    }
    #[test]
    fn optest5() {
        let nfa1 = NFA::from(".").unwrap();
        let nfa2 = NFA::from("b").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("ab"), true);
        assert_eq!(nfa.accepts("bumblebee"), true);
        assert_eq!(nfa.accepts("bear"), false);
    }
    #[test]
    fn optest6() {
        let nfa1 = NFA::from("ab(c|d)").unwrap();
        let nfa2 = NFA::from("x(y|z)").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("abdxy"), true);
        assert_eq!(nfa.accepts("abcxy"), true);
        assert_eq!(nfa.accepts("abcxz"), true);
    }
    #[test]
    fn optest7() {
        let nfa1 = NFA::from("tar").unwrap();
        let nfa2 = NFA::from("heels").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("tarheels"), true);
        assert_eq!(nfa.accepts("ttttttttarheelsssssss"), true);
        assert_eq!(nfa.accepts("tarpoopheels"), false);
        assert_eq!(nfa.accepts("tar*heels"), false);
    }
    #[test]
    fn optest8() {
        let nfa1 = NFA::from("12.34").unwrap();
        let nfa2 = NFA::from("ugh").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("1234"), false);
        assert_eq!(nfa.accepts("12p34ugh"), true);
        assert_eq!(nfa.accepts("12.34poop ugh"), false);
    }
    #[test]
    fn optest9() {
        let nfa1 = NFA::from("(taa*r)|(hh*eelss*)").unwrap();
        let nfa2 = NFA::from("(poo*p)|(yee*s.)").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("taaaaaaaryeeeeees0"), true);
        assert_eq!(nfa.accepts("taaaaaaarpooooooop"), true);
        assert_eq!(nfa.accepts("hhhhheelssspooooop"), true);
        assert_eq!(nfa.accepts("heelspooooop"), true);
    }
    #[test]
    fn optest10() {
        let nfa1 = NFA::from(".*").unwrap();
        let nfa2 = NFA::from(".*").unwrap();
        let nfa = nfa1 + nfa2;
        assert_eq!(nfa.accepts("a"), true);
        assert_eq!(nfa.accepts(""), true);
    }
}
