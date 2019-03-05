use std::iter::Peekable;
use std::str::Chars;

/**
 * thbc - Tar Heel Basic Calculator
 *
 * Author: <Sarah Bost> and Shannon Goad
 * ONYEN: <sbost99> SHANNON PUT YOUR ONYEN
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/**
 * The tokens types of `thbc` are defined below.
 */
#[derive(Debug, PartialEq)]
pub enum Token {
    Char(char),
    UnionBar,
    AnyChar,
    KleeneStar,
    LParen,
    RParen,
}

/**
 * The internal state of a Tokenizer is maintained by a peekable character
 * iterator over a &str's Chars.
 */
pub struct Tokenizer<'str> {
    chars: Peekable<Chars<'str>>,
}

impl<'str> Tokenizer<'str> {
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

/**
 * The Iterator trait is implemented for Tokenizer. It will produce items of
 * type Token and has a `next` method that returns Option<Token>.
 */
impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

    /**
     * The `next` method ignores leading whitespace and returns the next
     * complete Some(Token) in the Tokenizer's input string or None at all.
     */
    fn next(&mut self) -> Option<Token> {
        self.lex_whitespace();
        if let Some(c) = self.chars.peek() {
            Some(match c {
                '|'=> self.lex_union_bar(),
                '*' => self.lex_kleene_star(),
                '.' => self.lex_any_char(),
                '(' | ')' => self.lex_paren(),
                //i think he said something about whitespace needing to be tokenized on piazza but
                //ill double check later 
  //              c.is_whitespace() => self.lex_whitespace(), 
                _ => self.lex_char(),
            })
        } else {
            None
        }
    }
}

/*
 * Helper methods of Tokenizer are follow. None are defined as pub
 * so these are internal methods only.
 */
impl<'str> Tokenizer<'str> {
    fn lex_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\n' => self.chars.next(),
                _ => break,
            };
        }
    }

    fn lex_union_bar(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '|' => Token::UnionBar,
            _ => panic!("unknown char"),
        }
    }

    fn lex_kleene_star(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '*' => Token::KleeneStar,
            _ => panic!("Unexpected char"),
        }
    }

    fn lex_paren(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => panic!("unknown register"),
        }
    }

    fn lex_char(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        Token::Char(c)
    }

    fn lex_any_char(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '.' => Token::AnyChar,
            _ => panic!("unknown char"),
        }
    }

        }
