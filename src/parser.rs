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

/* == Begin Syntax Tree Elements == */
#[derive(Debug, PartialEq)]
pub enum AST {
    Alternation(Box<AST>, Box<AST>),
    Catenation(Box<AST>, Box<AST>),
    Closure(Box<AST>),
    AnyChar,
    Char(char),
}

/* Helper factory functions for building AST*/
pub fn build_alternation(left: AST, right: AST) -> AST {
    AST::Alternation(Box::new(left), Box::new(right))
}

pub fn build_char(value: char) -> AST {
    AST::Char(value)
}

pub fn build_catenation(first: AST, second: AST) -> AST {
    AST::Catenation(Box::new(first), Box::new(second))
}

pub fn build_closure(closure: AST) -> AST {
    AST::Closure(Box::new(closure))
}

pub fn build_anychar() -> AST {
    AST::AnyChar
}

/* == End Syntax Tree Elements == */

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<AST, String> {
        let mut parser = Parser {
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

/**
 * Internal-only parser methods to process the grammar via recursive descent.
 */
impl<'tokens> Parser<'tokens> {
    fn regexpr(&mut self) -> Result<AST, String> {
        let regex = self.maybe_regex()?;
        Ok(regex)
    }
    //Atom -> lparen RegExpr rparen | AnyChar | Char  according to grammar
    fn atom(&mut self) -> Result<AST, String> {
        //Take next toke if there is one (and doesn't throw error)
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
                    return Err(String::from("Unexpected end of input"));
                }
                // otherwise return x
                return Ok(x);
            }
            // token character should just return Ok(c)
            Token::Char(c) => {
                return Ok(build_char(c));
            }
            //take next token in atom should always match with LParen or number according to
            //grammar
            _ => {
                return Err("unexpected input".to_string());
            }
        }
    }

    fn closure(&mut self) -> Result<AST, String> {
        let first_term = self.atom()?;
        if self.peek_kleene_star().is_some() {
            let kleene_star = self.take_next_token();
            return Ok(build_closure(first_term));
        }
        Ok(first_term)
    }

    fn cat(&mut self) -> Result<AST, String> {
        let first_term = self.closure()?;
        let second_term = self.maybe_cat();
        if second_term.is_ok() {
            return Ok(build_catenation(first_term, second_term.unwrap()));
        }
        return Ok(first_term);
    }

    fn maybe_cat(&mut self) -> Result<AST, String> {
        match self.tokens.peek() {
            Some(Token::LParen) | Some(Token::AnyChar) => self.cat(),
            Some(Token::Char(c)) => self.cat(),
            _ => Err(String::from("this is an error we are checking")),
        }
    }

    fn maybe_regex(&mut self) -> Result<AST, String> {
        let lhs = self.cat()?;
        if self.peek_union_bar().is_some() {
            let union_bar = self.take_next_token();
            let rhs = self.regexpr()?;
            return Ok(build_alternation(lhs, rhs));
        }
        return Ok(lhs);
    }
}
impl<'tokens> Parser<'tokens> {
    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

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

    fn peek_kleene_star(&mut self) -> Option<char> {
        if let Some(Token::KleeneStar) = self.tokens.peek() {
            Some('*')
        } else {
            None
        }
    }

    fn peek_union_bar(&mut self) -> Option<char> {
        if let Some(Token::UnionBar) = self.tokens.peek() {
            Some('|')
        } else {
            None
        }
    }
}




