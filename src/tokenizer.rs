use std::iter::Peekable;
use std::str::Chars;

/**
 * thegrep - Tar Heel egrep
 *
 * Author: Sarah Bost, Shannon Goad
 * ONYEN: sbost99, sgoad13
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

// this creates a new tokenizer from input given as argument, this is what is called in main.rs
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
                '|' => self.lex_union_bar(),
                '*' => self.lex_kleene_star(),
                '.' => self.lex_any_char(),
                '(' | ')' => self.lex_paren(),
                _ => self.lex_char(),
                // these match options should allow whitespace to be recognized as a char token
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
    // consumes whitespace
    fn lex_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\n' => self.chars.next(),
                _ => break,
            };
        }
    }
    // consumes char, which will be union bar, and returns a unionbar token
    fn lex_union_bar(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '|' => Token::UnionBar,
            _ => panic!("unknown char"),
        }
    }

    // consumes char, which will be kleene, and returns a kleenestar token
    fn lex_kleene_star(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '*' => Token::KleeneStar,
            _ => panic!("Unexpected char"),
        }
    }

    // consumes char, which will be paren, and returns a paren token
    fn lex_paren(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => panic!("unknown register"),
        }
    }

    // consumes char and returns a char token
    fn lex_char(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        Token::Char(c)
    }

    // consumes char, which will be anychar, and returns an anychar token
    fn lex_any_char(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '.' => Token::AnyChar,
            _ => panic!("unknown char"),
        }
    }
}

/**
 * Unit Tests for the `next` method.
 */
#[cfg(test)]
mod iterator {
    use super::*;

    #[test]
    fn empty() {
        // tests for empty string
        let mut tokens = Tokenizer::new("");
        assert_eq!(tokens.next(), None);
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn parenthesis() {
        // tests just parentheses
        let mut tokens = Tokenizer::new("()");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::RParen));
    }

    #[test]
    fn union() {
        // tests the unionbar token
        let mut tokens = Tokenizer::new("|");
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn kleene() {
        // tests the kleene star token
        let mut tokens = Tokenizer::new("*");
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn anychar() {
        // tests the anychar token
        let mut tokens = Tokenizer::new(".");
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn char() {
        // tests the char token
        let mut tokens = Tokenizer::new("a");
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), None);
    }

    //checks each kind of token next can generate
    #[test]
    fn alltokens() {
        let mut tokens = Tokenizer::new("a|*().");
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), None);
    }

}
