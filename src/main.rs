#[allow(unused)]
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
// allows us to use structopt crate for flags, etc
extern crate structopt;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "thegrepc", about = "Tar Heel Egrep")]

// setting up flags for parse and tokens
struct Opt {
    #[structopt(short = "p", long = "parse")]
    parse: bool,
    #[structopt(short = "t", long = "tokens")]
    tokens: bool,
    #[structopt(short = "d", long = "dot")]
    dot: bool,
    #[structopt(help = "FILES")]
    paths: Vec<String>,
    #[structopt(short = "g", long = "gen")]
    num: f64, 
    //    #[structopt(short = "g", long = "gen")]
    //    num: 64,
}



use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use rand::prelude::*; 

fn main() {
    let opt = Opt::from_args();
//    println!("{:?}", opt);
    // if arguments are passed in read from file/paths otherwise evaluate input from std::in
    if opt.parse {
        // makes a parse tree of input
        match Parser::parse(Tokenizer::new(&opt.paths[0])) {
            Ok(statement) => {
                println!("{:?}", statement);
            }
            Err(msg) => eprintln!("thegrep: {}", msg),
        }
        print!("\n");
    } else if opt.tokens {
        println!("yes");
        // create a new tokenizer and cycle through tokens
        let mut tokens = Tokenizer::new(&opt.paths[0]);
        while let Some(token) = tokens.next() {
            println!("{:?}", token);
        }
        print!("\n");
    } else if opt.dot {
        // push output to dot nfa representation
        let nfa = NFA::from(&opt.paths[0]).unwrap();
        println!("{}", nfa_dot(&nfa));
        std::process::exit(0);
    } else if opt.num > 0.0 { 
      //  println!("{:?}", opt.paths[0]);
         let nfa = NFA::from(&opt.paths[0]).unwrap();
        let mut expression_count = 0.0; 

        while expression_count < opt.num {
            println!("{}", nfa.random_regex()); 
             expression_count = expression_count + 1.0;
        }
            
         }
        


    let result = if opt.paths.len() > 1 {
        print_files(&opt)
    } else {
        print_stdin(&opt)
    };

    // print error if paths has error
    if let Err(e) = result {
        eprintln!("{}", e);
    }
}

// processes input and calls print function
fn print_stdin(opt: &Opt) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    print_lines(reader, opt)
}

// iterates through all paths/files and calls print function
fn print_files(opt: &Opt) -> io::Result<()> {
    let regex = opt.paths[0].to_string();
    //    Ok(eval(&regex, opt))
    for path in opt.paths.iter().skip(1) {
        println!("{:?}", path);
        // we skipped 1 because the first one is regex to match later, everything else is files
        let file = File::open(path)?;
        let mut reader = io::BufReader::new(file);
        print_lines(reader, opt)?;
    }
    Ok(())
}

// pushes all lines in a file onto string and calls eval function to call tokens/parser
fn print_lines<R: BufRead>(reader: R, opt: &Opt) -> io::Result<()> {
    //call eval function to process tokens/parser
    //rintln!("{}", opt.paths[0]);
    for line in reader.lines() {
        eval(&line?, opt);
    }
    Ok(())
}

// importing tokenizer and parser to use in main
pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;
pub mod nfa;
use self::nfa::helpers::nfa_dot;
use self::nfa::NFA;

fn eval(input: &str, options: &Opt) {
//    println!("{:?}", options);
    if options.parse {
        // makes a parse tree of input
        match Parser::parse(Tokenizer::new(input)) {
            Ok(statement) => {
                println!("{:?}", statement);
            }
            Err(msg) => eprintln!("thegrep: {}", msg),
        }
        print!("\n");
    } else if options.tokens {
  //      println!("yes");
        // create a new tokenizer and cycle through tokens
        let mut tokens = Tokenizer::new(input);
        while let Some(token) = tokens.next() {
            println!("{:?}", token);
        }
        print!("\n");
    } else if options.dot {
        // push output to dot nfa representation
        let nfa = NFA::from(input).unwrap();
        println!("{}", nfa_dot(&nfa));
        std::process::exit(0);
    }
    // if options.num {
    // this is for using that number to generate random strings that match the nfa
    // }
    else {
        // no matter what options are chosen, make the NFA out of the given regex and test the input
        // for accepts based on the NFA
        // let mut regex = ".*".to_string();
        //regex.push_str(&options.paths[0]);
        //regex.push_str(".*");
        let nfa = NFA::from(&options.paths[0]).unwrap();
        if nfa.accepts(input) {
            println!("{}", input);
        }
    }
}
