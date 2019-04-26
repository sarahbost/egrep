use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

/**
 * thbc - Tar Heel Egrep - Parser
 *
 * Author: Sarah Bost, Shannon Goad
 * ONYEN: sbost99, sgoad13
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

// elements in AST, things that can be an AST
#[derive(Debug, PartialEq)]
pub enum AST {
    Alternation(Box<AST>, Box<AST>),
    Catenation(Box<AST>, Box<AST>),
    Closure(Box<AST>),
    AnyChar,
    Char(char),
    OneOrMore(Box<AST>),
}

// Helper factory functions for building AST
pub fn build_alternation(left: AST, right: AST) -> AST {
    AST::Alternation(Box::new(left), Box::new(right))
}

pub fn build_char(value: char) -> AST {
    AST::Char(value)
}

pub fn build_catenation(first: AST, second: AST) -> AST {
    AST::Catenation(Box::new(first), Box::new(second))
}

pub fn build_one_or_more(ast: AST) -> AST {
    AST::OneOrMore(Box::new(ast))
}

pub fn build_closure(closure: AST) -> AST {
    AST::Closure(Box::new(closure))
}

pub fn build_anychar() -> AST {
    AST::AnyChar
}

pub struct Parser<'tokens> {
    // parser needs a tokenizer to process the elements of input
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<AST, String> {
        let mut parser = Parser {
            // create a peekable tokenizer to make tokens to parse
            tokens: tokenizer.peekable(),
        };

        //calls parser expression returns Result<Expr, String>
        let res = parser.regexpr();

        //If there are still tokens left in the parser, return an error
        if let Some(c) = parser.tokens.peek() {
            return Err(format!("Expected end of input, found {:?}", c));
        } else {
            //returns type Result<Expr,String> of parsed syntax stree or an error
            return res;
        }
    }
}

#[cfg(test)]
mod parsertests {
    use super::*;
    use crate::parser::AST::Char;
    use crate::parser::AST::*;

    mod basictests {

        use super::*;
        use crate::parser::AST::*;

        #[test]
        fn parse_char() {
            let res = Parser::parse(Tokenizer::new("a")).unwrap();
            assert_eq!(Char('a'), res);
        }

        #[test]
        fn parse_cat() {
            let res = Parser::parse(Tokenizer::new("ab")).unwrap();
            assert_eq!(Catenation(Box::new(Char('a')), Box::new(Char('b'))), res);
        }

        #[test]
        fn parse_alt() {
            let res = Parser::parse(Tokenizer::new("a|b")).unwrap();
            assert_eq!(Alternation(Box::new(Char('a')), Box::new(Char('b'))), res);
        }

        #[test]
        fn parse_closure() {
            let res = Parser::parse(Tokenizer::new("a*")).unwrap();
            assert_eq!(Closure(Box::new(Char('a'))), res);
        }

        #[test]
        fn parse_anychar() {
            let res = Parser::parse(Tokenizer::new(".")).unwrap();
            assert_eq!(AnyChar, res);
        }
    }

    mod intermediatetests {

        use super::*;
        use crate::parser::*;

        #[test]
        fn parse1() {
            let res = Parser::parse(Tokenizer::new("a.*")).unwrap();
            assert_eq!(
                Catenation(Box::new(Char('a')), Box::new(Closure(Box::new(AnyChar)))),
                res
            );
        }

        #[test]
        fn parse2() {
            let res = Parser::parse(Tokenizer::new("a|b|c")).unwrap();
            assert_eq!(
                Alternation(
                    Box::new(Char('a')),
                    Box::new(Alternation(Box::new(Char('b')), Box::new(Char('c'))))
                ),
                res
            );
        }

        #[test]
        fn parse3() {
            let res = Parser::parse(Tokenizer::new("(ab)*")).unwrap();
            assert_eq!(
                Closure(Box::new(Catenation(
                    Box::new(Char('a')),
                    Box::new(Char('b'))
                ))),
                res
            );
        }
    }

    //  mod hardtests {

    //  use super::*;
    //  use parser::*;

    //  #[test]
    //  fn challenge() {
    //      let res = Parser::parse(Tokenizer::new("b(oo*|a)m")).unwrap();
    //     assert_eq!(Catenation(Box::new(Char('b')), Box::new(Catenation(Box::new(Alternation(Box::new(Catenation(Box::new(Char('o')), Box::new(Closure(Box::new(Char('o')))), Box::new(Char('a'))))))), Box::new(Char('m')))), res);
    //    }
    //}
}

/**
 * Internal-only parser methods to process the grammar via recursive descent.
 */
impl<'tokens> Parser<'tokens> {
    // regexpr is our "base" function, i.e. it is the first place the input is mapped to
    fn regexpr(&mut self) -> Result<AST, String> {
        let regex = self.maybe_regex()?;
        Ok(regex)
    }
    //Atom -> lparen RegExpr rparen | AnyChar | Char  according to grammar
    fn atom(&mut self) -> Result<AST, String> {
        // atom is sent input by closure, so this is our "base case" of recursion, i.e. nothing is
        // smaller than an atom in our grammar
        //Take next token if there is one (and doesn't throw error)
        let t: Token = self.take_next_token()?;
        match t {
            //if the token is anychar, make a new AST and return
            Token::AnyChar => {
                return Ok(build_anychar());
            }
            //If token is an LParen, input should follow lparen AST RParen
            //Consume tokens in this order and return the AST
            Token::LParen => {
                // x is next ast or error
                let x = self.regexpr()?;
                //r should be rparen
                let r = self.consume_token(Token::RParen);
                if !r.is_ok() {
                    return Err(String::from("Unexpected end of input")); // unclosed parentheses case
                }
                // otherwise return x
                return Ok(x);
            }
            // token character should just return Ok(c)
            Token::Char(c) => {
                return Ok(build_char(c));
            }
            _ => {
                return Err("unexpected input".to_string());
            }
        }
    }

    fn closure(&mut self) -> Result<AST, String> {
        // closure receives input from cat()
        let first_term = self.atom()?;

        //checks for kleene star
        if self.peek_kleene_star().is_some() {
            let kleene_star = self.take_next_token();
            return Ok(build_closure(first_term)); // if there's a kleene star, make a closure
        }

        //checks for kleen plus and will take next token
        if self.peek_kleene_plus().is_some() {
            let kleene_plus = self.take_next_token();
            return Ok(build_one_or_more(first_term));
        }

        Ok(first_term) // if no kleene star, just return first term wrapped in result
    }

    fn cat(&mut self) -> Result<AST, String> {
        // this is somewhat the third stage of parsing, because maybe_regex maps here
        let first_term = self.closure()?; // see if first term is a closure AST
        let second_term = self.maybe_cat(); // check and see if second term is an AST also
        if second_term.is_ok() {
            return Ok(build_catenation(first_term, second_term.unwrap())); // catenate first and second terms
        }
        return Ok(first_term); // if there is no second term don't make a new catenation, just return the first term wrapped in result
    }

    fn maybe_cat(&mut self) -> Result<AST, String> {
        match self.tokens.peek() {
            // this match statement sends to cat if there is one, otherwise throws an error
            Some(Token::LParen) | Some(Token::AnyChar) => self.cat(),
            Some(Token::Char(c)) => self.cat(),
            _ => Err(String::from("this is an error we are checking")),
        }
    }

    fn maybe_regex(&mut self) -> Result<AST, String> {
        // this is the second stage of recursive parsing, because regexpr maps directly here
        let lhs = self.cat()?; // base term is a catenation
        if self.peek_union_bar().is_some() {
            // test for alternation
            let union_bar = self.take_next_token();
            let rhs = self.regexpr()?;
            return Ok(build_alternation(lhs, rhs)); // return an alternation wrapped in result
        }
        return Ok(lhs); // if there isn't a union bar just return lhs
    }
}
impl<'tokens> Parser<'tokens> {
    // take next token consumes the token and returns it wrapped in a result
    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    // consumes a token and returns it wrapped in a result, but this is for when you know which
    // token you expect
    fn consume_token(&mut self, expected: Token) -> Result<Token, String> {
        if let Some(next) = self.tokens.next() {
            if next != expected {
                Err(format!("Expected: {:?} - Found {:?}", expected, next))
            } else {
                Ok(next)
            }
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    // function to peek if the next char is a kleene star
    fn peek_kleene_star(&mut self) -> Option<char> {
        if let Some(Token::KleeneStar) = self.tokens.peek() {
            Some('*')
        } else {
            None
        }
    }

    //function to peek if the next char is a kleene plus
    fn peek_kleene_plus(&mut self) -> Option<char> {
        if let Some(Token::KleenePlus) = self.tokens.peek() {
            Some('+')
        } else {
            None
        }
    }

    // function to peek if the next char is a union bar
    fn peek_union_bar(&mut self) -> Option<char> {
        if let Some(Token::UnionBar) = self.tokens.peek() {
            Some('|')
        } else {
            None
        }
    }
}
