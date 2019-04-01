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
        // vector of chars formed from input so that we can iterate over them in accepts()
        let input_chars = input.chars().collect();
        // the final "false" here is a bool that will be useful later to see if the nfa has already
        // been started being read in the input
        self.traverse(&input_chars, 0, self.start, false)
    }

    pub fn traverse(
        &self,
        chars: &Vec<char>,
        chars_index: usize,
        start_state_id: StateId,
        has_started_nfa: bool,
    ) -> bool {
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
        // creates fragments of an NFA based on what AST they are
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
//        assert_eq!(nfa.accepts("aaaaaaaaaaaaaaaaab"), true);
        assert_eq!(nfa.accepts("aaab"), true);
        assert_eq!(nfa.accepts("aaaaaab"), true);
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
