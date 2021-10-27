/*
The standard (std) library does not exist in Kaos. A new standard library will need to be written for Kaos for this shell to work. 
The standard library is a collection of functions that are interfaced into the OS to perform normal functions.
*/
use std::{env, process::{Command, Stdio, Child}, io::{self, Write}, path::Path};

fn main() {
    loop {
        println!("> ");
        let flush = io::stdout().flush();
        if let Err(_err) = flush {
            println!("Error flushing stdout");
        }
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let mut command_set = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = command_set.next() {

            let mut parts = command.trim().split_whitespace();
            let action = parts.next().unwrap();
            let args = parts;

            match action {
                "exit" => return,
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        println!("{}", e);
                    }
                    previous_command = None;
                },
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));
                    let stdout = if command_set.peek().is_some(){
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };
                    let output = Command::new(command).args(args).stdin(stdin).stdout(stdout).spawn();
                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        },
                        Err(e) => {
                            println!("{}", e);
                            previous_command = None;
                        }
                    }
                }    
            }
        }
        if let Some(mut final_command) = previous_command {
            let status = final_command.wait();
            match status {
                Ok(status) => {
                    println!("{}", status);
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}