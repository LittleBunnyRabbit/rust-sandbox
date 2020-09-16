mod examples {
    pub mod print;
    pub mod vars;
    pub mod types;
    pub mod strings;
    pub mod touples;
    pub mod arrays;
    pub mod vectors;
    pub mod conditionals;
    pub mod loops;
    pub mod functions;
    pub mod pointer_ref;
    pub mod structs;
    pub mod enums;
    pub mod cli;
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: String = if args.len() > 1 { args[1].clone() } else { String::from("") };

    let selected_example = match &command[..] {
        "print" => examples::print::run,
        "vars" => examples::vars::run,
        "types" => examples::types::run,
        "strings" => examples::strings::run,
        "touples" => examples::touples::run,
        "arrays" => examples::arrays::run,
        "vectors" => examples::vectors::run,
        "conditionals" => examples::conditionals::run,
        "loops" => examples::loops::run,
        "functions" => examples::functions::run,
        "pointer_ref" => examples::pointer_ref::run,
        "structs" => examples::structs::run,
        "enums" => examples::enums::run,
        "cli" => examples::cli::run,
        _ => || println!("That function doesnt exist")
    };

    selected_example();
}
