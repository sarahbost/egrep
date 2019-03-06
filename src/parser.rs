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
    Alternation {
        left: Box<AST>,
        right: Box<AST>,
    },
    Catenation(Box<AST>, Box<AST>), 
    Closure(Box<AST>), 
}

/* Helper factory functions for building AST*/
pub fn build_alternation(left: AST, right: AST) -> AST {
    AST::Alternation {
        left: Box::new(left),
        right: Box::new(right),
    }
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
        let res = parser.ast();

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
    fn ast(&mut self) -> Result<AST, String> {

       return self.atom(); 
    }

    //Atom -> lparen RegExpr rparen | AnyChar | Char  according to grammar
    fn atom(&mut self) -> Result<AST, String> {
        //Take next toke if there is one (and doesn't throw error)
        let t: Token = self.take_next_token()?;
        match t {
            //if the token is anychar, make a new AST and return
            Token::AnyChar => {
                return Ok(build_anychar());
            },
            //If token is an LParen, input should follow lparen AST RParen
            //Consume tokens in this order and return the AST
            Token::LParen => {
                // x is next ast or error
                let x = self.ast()?;
                //r should be rparen
                let r = self.consume_token(Token::RParen)?;
                if !r.is_ok() {
                    return Err(String::from("Unexpected end of input"));
                }
                // otherwise return x
                return Ok(x);
            },
            // token character should just return Ok(c)
            Token::Char(c) => {
                return Ok(build_char(c));
            },
            //take next token in atom should always match with LParen or number according to
            //grammar
            _ => {
                return Err("unexpected input".to_string());
            }
        }
    }
}
