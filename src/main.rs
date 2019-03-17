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


//const QUIT_STRING: &str = "quit\n";
//const EXIT_OK: i32 = 0;
//const EXIT_ERR: i32 = 1;

//use std::io;
//#[derive(Debug, StructOpt)]
#[structopt(name = "thegrepc", about = "Tar Heel Egrep")]

//setting up flags for parse and tokens
struct Opt {
    #[structopt(short = "p", long = "parse")]
    parse: bool,
    #[structopt(short = "t", long = "tokens")]
    tokens: bool,
    #[structopt(help = "FILES")]
    paths: Vec<String>,
}

use std::fs::File; 
use std::io::BufRead; 
use std::io; 

const EXIT_ERR: i32 = 1; 
 fn main() {
     let opt = Opt::from_args();

     let result = if opt.paths.len() > 0 {
         print_files(&opt)
     } else {
        print_stdin(&opt)
};
    if let Err(e) = result {
        eprintln!("{}", e);
    }
}

 fn print_stdin(opt: &Opt) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    print_lines(reader, opt);
    Ok(())
 }

fn print_files(opt: &Opt) -> io::Result<()> { 
    for path in opt.paths.iter() { 
        let reader = io::BufReader::new(;
        print_lines(reader, opt)?;
    }
    Ok(())
}

fn print_lines<R: BufRead>(reader: R, opt: &Opt) -> io::Result<()> {
    let  mut argument: String = "".to_string(); 
    for line_result in reader.lines() {        
        println!("{:?}", line_result); 
    }
    //eval(&argument, opt); 
    Ok(())

}

// importing tokenizer and parser to use in main
pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;


//calls appropriate function based on flags
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
        eval_tokens(input);
    }
}

// evaluate here if parse flag detected
fn eval_parser(input: &str) {
    // create a new parser and cycle through input, return Ok for each parse chain and Err if
    // parser detected an error
    match Parser::parse(Tokenizer::new(input)) {
        Ok(statement) => {
            println!("{:?}", statement);
        }
        Err(msg) => eprintln!("thegrep: {}", msg),
    }
    print!("\n");
}

// evaluate here if tokens flag detected
fn eval_tokens(input: &str) {
    // create a new tokenizer and cycle through tokens
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    print!("\n");
}

// this reads in input from command line args
fn read() -> String {
    match read_line() {
        Ok(line) => {
        //    if line == QUIT_STRING {
                // Exit the process with an Ok exit code.
        //        std::process::exit(EXIT_OK);
        //    } else {
                line
         //   }
        }
        Err(message) => {
            eprintln!("Err: {}", message);
           std::process::exit(EXIT_ERR);
        }
    }
}

// this reads a line of input
fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
