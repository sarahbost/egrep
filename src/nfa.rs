pub mod helpers;

// Starter code for PS06 - thegrep
// Add Honor Code Header and Collaborators Here

use self::State::*;
use super::parser::Parser;
use super::parser::AST;
use super::tokenizer::Tokenizer;
use std::iter::Peekable;
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

        let start = nfa.add(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    /**
     * Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     */
    pub fn accepts(&self, input: &str) -> bool {
        //vector of chars
        let input_chars = input.chars().collect();
        self.traverse(&input_chars, 0, self.start)
    }

    pub fn traverse(&self, chars: &Vec<char>, chars_index: usize, start_state_id: StateId) -> bool {
        match &self.states[start_state_id] {
            Start(state_id) => return self.traverse(&chars, chars_index, state_id.unwrap() + 1),
            Split(lhs, rhs) => {
                if self.traverse(&chars, chars_index, lhs.unwrap() + 1) {
                    return true;
                }

                if self.traverse(&chars, chars_index, rhs.unwrap() + 1) {
                    return true;
                }
                return false;
            },
            Match(character, state_id) => {
                //if the char matches, keep going and increment index, if not it doesn't match and
                //return false
                let check_char = chars[chars_index];
                match character {
                    check_char => {
                        if chars_index == chars.len() {
                            return true;
                        }
                        return self.traverse(&chars, chars_index + 1, state_id.unwrap() + 1);
                    },
                    _ => { return false;
                        //not sure, i think you need to recursively call and start char index at 0
                    },
                };
            },
            End => {
                return false;
            },
        };
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
#[derive(Debug)]
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
    fn add(&mut self, state: State) -> StateId {
        let idx = self.states.len();
        self.states.push(state);
        idx
    }

    /**
     * Given an AST node, this method returns a Fragment of the NFA
     * representing it and its children.
     */
    fn gen_fragment(&mut self, ast: &AST) -> Fragment {
        match ast {
            AST::AnyChar => {
                let state = self.add(Match(Char::Any, None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Char(c) => {
                let state = self.add(Match(Char::Literal(*c), None));
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
                    self.add(Split(Some(fragment_one.start), Some(fragment_two.start)));
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
                let split_state = self.add(Split(Some(fragment_ast.start), None));
                self.join(split_state, fragment_ast.start);
                self.join(fragment_ast.start, split_state);

                Fragment {
                    start: split_state,
                    ends: vec![split_state],
                }
            }

            node => panic!("Unimplemented branch of gen_fragment: {:?}", node),
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
    fn lvl0test() {
        let nfa = NFA::from("a").unwrap();
        assert_eq!(nfa.accepts("a"), true);
    }

    #[test]
    fn empty() {
        let nfa = NFA::from("sarah").unwrap();
        assert_eq!(nfa.accepts("ra"), true);
    }

    #[test]
    fn test1() {
        let nfa = NFA::from("a.*").unwrap();
        assert_eq!(nfa.accepts("a.*"), true);
    }
    #[test]
    fn hello() {
        let nfa = NFA::from("hello").unwrap();
        assert_eq!(nfa.accepts("no"), false);
    }

}
