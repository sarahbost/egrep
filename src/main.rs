#![allow(unused)]

/**
 * thegrep - Tar Heel egrep
 *
 * Author(s): Sarah Bost, Shannon Goad
 * ONYEN(s): sbost99, sgoad13
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff and partner.
 */
//allows us to use structopt crate for flags, etc
extern crate structopt;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "thegrepc", about = "Tar Heel Egrep")]

//setting up flags for parse and tokens
struct Opt {
    #[structopt(short = "p", long = "parse")]
    parse: bool,
    #[structopt(short = "t", long = "tokens")]
    tokens: bool,
    #[structopt(help = "FILES")]
    paths: Vec<String>,
    #[structopt(help = "PATTERN")] 
    pattern: String, 
    #[structopt(short = "d", long = "dot")]
    dot: bool,
}

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;

fn main() {
    let opt = Opt::from_args();

    //if arguments are passed in read from file/paths otherwise evaluate input from std::in
    let result = if opt.paths.len() > 0 {
        print_files(&opt)
    } else {
        print_stdin(&opt)
    };

    //print error if paths has error
    if let Err(e) = result {
        eprintln!("{}", e);
    }
}

//processes input and calls print function
fn print_stdin(opt: &Opt) -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let reader = stdin.lock().read_to_string(&mut buffer)?;
    print_lines(reader.to_string(), opt);
    Ok(())
}

//iterates through all paths/files and calls print function
fn print_files(opt: &Opt) -> io::Result<()> {
    for path in opt.paths.iter() {
        print_lines(path.to_string(), opt)?;
    }
    Ok(())
}

//pushes all lines in a file onto string and calls eval function to call tokens/parser
fn print_lines(reader: String, opt: &Opt) -> io::Result<()> {
    //call eval function to process tokens/parser
    eval(&reader, opt);
    Ok(())
}

// fn nfa_egrep(

// importing tokenizer and parser to use in main
pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;
pub mod nfa;
use self::nfa::NFA;
use self::nfa::helpers::nfa_dot;

//creates parser/ tokenzer to parse or tokenize input
fn eval(input: &str, options: &Opt) {
    if options.parse {
        match Parser::parse(Tokenizer::new(input)) {
            Ok(statement) => {
                println!("{:?}", statement);
            }
            Err(msg) => eprintln!("thegrep: {}", msg),
        }
        print!("\n");
    }
    if options.tokens {
        // create a new tokenizer and cycle through tokens
        let mut tokens = Tokenizer::new(input);
        while let Some(token) = tokens.next() {
            println!("{:?}", token);
        }
        print!("\n");
    }
    if options.dot {
        // push output to dot nfa representation
        let nfa = NFA::from(input).unwrap();
        println!("{}", nfa_dot(&nfa));
        std::process::exit(0);
    }
}
