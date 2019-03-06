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
    Char(char),
    AnyChar,
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
>>>>>>> 8544b0a231ef55a608dc339f2db8efcd40512d37
    }

    // Level 1:
    // MaybeMulDiv  -> Atom MulDivOp?
    fn maybe_mul_div(&mut self) -> Result<Expr, String> {
        //calls atom acording to grammar and gets Expr if valid
        let mut x = self.atom()?;

        //check if next token is an operator and call mul_div_op if it is according to grammer
        //x is an Expr of the binop so pass as parameter to mul div op so it can create a binop
        if let Some(c) = self.peek_operator() {
            match c {
                '*' => x = self.mul_div_op(x)?,
                '/' => x = self.mul_div_op(x)?,
                _ => return Ok(x),
            }
        }

        Ok(x)
    }

    // MulDivOp     -> ('*'|'/') Atom
    /**
     * The lhs: Expr is passed in so that the syntax tree can grow "down" the lhs.
     */
    fn mul_div_op(&mut self, lhs: Expr) -> Result<Expr, String> {
        //consume the operator token
        let y = self.take_operator().unwrap();

        //get the next atom according to grammar
        let x = self.atom()?;

        //create new binop with the argument as the lhs, operator as the op and the returned atom
        //as the expr
        let mut b = binop(lhs, y, x);

        //if there an operator is the next token, pass the new binop as the lhs and recursively
        //call
        if let Some(c) = self.peek_operator() {
            b = self.mul_div_op(b)?;
        }

        Ok(b)
    }

    // Level 2: Does not add new rules, rather modifies Level 1's!

    // Level 3:
    // MaybeAddSub -> MaybeMulDiv AddSubOp?
    fn maybe_add_sub(&mut self) -> Result<Expr, String> {
        //calls maybe mul div according to grammar
        let mut x = self.maybe_mul_div()?;

        //if the next token is an + or -, pass the Expr to the add sub op as its LHS to make a new
        //binop
        if let Some(c) = self.peek_operator() {
            match c {
                '+' => x = self.add_sub_op(x)?,
                '-' => x = self.add_sub_op(x)?,
                //if the operator isn't a + or -, just return the expr
                _ => {
                    return Ok(x);
                }
            }
        }
        Ok(x)
    }

    fn add_sub_op(&mut self, lhs: Expr) -> Result<Expr, String> {
        //consume operator
        let x = self.take_operator().unwrap();

        //call maybe mul div according to grammar and get expr
        let y = self.maybe_mul_div()?;

        //create new binop with the expr returned and the operator
        let mut b = binop(lhs, x, y);

        //if the next token is an operator, recursively call this function and pass the new binop
        //as the Expr for the lhs
        if let Some(c) = self.peek_operator() {
            b = self.add_sub_op(b)?;
        }
        Ok(b)
    }

}


    }
}

/* Parser's Helper Methods to improve ergonomics of parsing */
impl<'tokens> Parser<'tokens> {
    /**
     * Static helper method used in unit tests to establish a
     * parser given a string.
     */
    fn from(input: &'tokens str) -> Parser<'tokens> {
        Parser {
            tokens: Tokenizer::new(input).peekable(),
        }
    }

    /**
     * When you expect another token and want to take it directly
     * or raise an error that you expected another token here but
     * found the end of input. Example usage:
     *
     * let t: Token = self.take_next_token()?;
     *
     * Notice the ? usage will automatically propagate the Err or
     * unwrap the value of Ok.
     */
    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    /**
     * When you want to peek for an operator this helper method
     * will optionally return the operator's character value to you
     * or it will return None.
     */
    fn peek_operator(&mut self) -> Option<char> {
        if let Some(Token::Operator(op)) = self.tokens.peek() {
            Some(*op)
        } else {
            None
        }
    }

    /**
     * When you know you want to take an Operator token, this helper
     * method will optionally take it and return it or result in an
     * Err. Example usage:
     *
     * let op: Token = self.take_operator()?;
     */
    fn take_operator(&mut self) -> Result<char, String> {
        let token = self.tokens.next();
        if let Some(Token::Operator(op)) = token {
            Ok(op)
        } else {
            Err(format!("Expected operator, found {:?}", token))
        }
    }

    /**
     * When there's a specific token you expect next in the grammar
     * use this helper method. It will raise an Err if there is no
     * next token or if it is not _exactly_ the Token you expected
     * next. If it is the token you expected, it will return Ok(Token).
     */
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
}

