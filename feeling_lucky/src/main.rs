use std::collections::HashSet;
use std::fs;
use std::env;
use std::process;

const MACROS: [&str; 6] = [
    "#include", "#define", "#if",
    "#ifdef", "#ifndef", "#endif",
];

const KEYWORDS: [&str; 10] = [
    "main",   "int",    "char",
    "float",  "double", "bool",
    "for",    "while",  "void",
    "return",
];

fn get_file_data() -> Result<String, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ERROR: usage: ./feeling_lucky <code.c>");
        process::exit(1);
    }
    let filepath = &args[1];
    Ok(fs::read_to_string(filepath)?)
}

fn main() {

    let mut keywords: HashSet<&str> = KEYWORDS
        .into_iter()
        .collect();

    let mut macros: HashSet<&str> = MACROS
        .into_iter()
        .collect();

    match get_file_data() {
        Ok(result) => {
            println!("Result: {}", result);
        }
        Err(error) => {
            eprintln!("ERROR: {}", error);
            process::exit(1);
        }
    }
}
