pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod constants;
pub mod time;

use rustyline::{error::ReadlineError, DefaultEditor};

pub fn run(mut args: impl Iterator<Item=String>) {
    args.next();
    args.for_each(|s| println!("{}",s));
    nova_repl();
}


pub fn nova_repl() {
    let mut rl = DefaultEditor::new().unwrap();
    
    println!();
    println!("Welcome to Nova, a highly expressive, dynamically typed functional programming language.\nType an expression to get started.\n");
    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(inp) => {
                if inp.len()==0 {
                    continue;
                }
                
                rl.add_history_entry(inp.clone().trim()).unwrap();
                let lex=lexer::Lexer::new(inp);
                println!("Tokenized: {:?}", lex.to_vec());
            },
            
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("See you again!");
                break;            
            }
            _ => (),
        }
    }
}
