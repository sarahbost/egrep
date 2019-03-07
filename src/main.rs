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

//copied and pasted this from the starter code in previous assignments
extern crate structopt;
use structopt::StructOpt;

const QUIT_STRING: &str = "quit\n"; 
const EXIT_OK: i32 = 0; 
const EXIT_ERR: i32 = 1; 

use std::io; 
#[derive(Debug, StructOpt)]
#[structopt(name = "thegrepc", about = "Tar Heel Egrep")]

//potentially need to account for the '-' before the flags?
struct Opt {
    #[structopt(short = "h", long = "help")]
    help: bool, 
    #[structopt(short = "p", long = "parse")]
    parse: bool,
     #[structopt(short = "t", long = "tokens")]
    tokens: bool,
    #[structopt(short = "V", long = "version")]
    version: bool,
}

// importing tokenizer and parser to use in main
pub mod tokenizer; 
use self::tokenizer::Tokenizer; 
pub mod parser; 
use self::parser::Parser; 

fn main() {
    let opt = Opt::from_args();
//    println!("{:?}", opt); 
    
    loop {
        eval(&read(), &opt); 
    }
}


//calls function based on flags 
fn eval(input: &str, options: &Opt) {

    if options.parse {
        eval_parser(input); 
    } 

    if options.tokens {
        eval_tokens(input); 
    }

    if options.help {
        eval_tokens(input); 
    }



}


fn eval_parser(input: &str) {
    match Parser::parse(Tokenizer::new(input)) {
        Ok(statement) => {
            println!("{:?}", statement); 
        }
        //need to initalize msg?
        // no need to initialize msg, this msg is what is returned if parser throws an error
        Err(msg) => eprintln!("thegrep: {}", msg), 
    }
    print!("\n"); 
}

fn eval_tokens(input: &str) {
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    print!("\n");
}

fn eval_help(input: &str) {
    println!("thegrep 1.0.0");
    println!("Tar Heel egrep");
    println!("/n");
    println!("USAGE:");
    println!("/t thegrep {}", input);
    println!("/n");
    println!("FLAGS:");
    println!("/t -h, --help /t Prints help information");
    println!("/t -p, --parse /t Show parsed AST");
    println!("/t -t, --tokens /t Show Tokens");
    println!("/t -V, --version /t Prints version information");
    println!("ARGS:");
    println!(" /t uh fill in pattern here /t Regular Expression Pattern");
}

fn eval_version(input: &str) {
    println!("thegrep version 1.0.0");
}


//copied from thbc
//
fn read() -> String {
    match read_line() {
        Ok(line) => {
            if line == QUIT_STRING {
                // Exit the process with an Ok exit code.
                std::process::exit(EXIT_OK);
            } else {
                line
            }
        }
        Err(message) => {
            eprintln!("Err: {}", message);
            std::process::exit(EXIT_ERR);
        }
    }
}

/**
 * Helper function to read a line of input from stdin.
 */
fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
