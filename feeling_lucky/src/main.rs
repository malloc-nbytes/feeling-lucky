use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;
use std::process::Command;

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

const MACROS: [&str; 9] = [
    "#include", "#define", "#if",
    "#else", "#elif", "#ifdef",
    "#ifndef", "#endif", "#defined",
];

const KEYWORDS: [&str; 20] = [
    "main", "for", "while",
    "return", "break", "switch",
    "do", "enum", "struct",
    "continue", "const", "goto",
    "extern", "if", "else",
    "register", "static", "typedef",
    "union", "volatile",
];

const TYPES: [&str; 15] = [
    "int", "char", "float",
    "long", "byte", "short",
    "double", "float", "bool",
    "void", "unsigned", "signed",
    "size_t", "uint32_t", "uint64_t",
];

const ODDS: u32 = 1000;

fn get_random(rng: &mut ThreadRng, upper: u32) -> (u32, u32) {
    (rng.gen_range(0..upper), rng.gen_range(0..upper))
}

fn reverse_token(token: String) -> String {
    token.chars().rev().collect()
}

fn duplicate_token(token: String) -> String {
    let mut new_token = String::from(&token);
    new_token += &token;
    new_token
}

fn delete_single_char(token: String, rng: &mut ThreadRng) -> String {
    let mut arr: Vec<char> = token.chars().collect();
    if token.len() > 0 {
        arr.remove(rng.gen_range(0..16) % token.len());
    }
    arr.into_iter().collect()
}

fn swap_words(words: &HashSet<&str>, rng: &mut ThreadRng) -> String {
    words
        .iter()
        .cloned()
        .collect::<Vec<_>>()
        .choose(rng)
        .unwrap()
        .to_string()
}

fn process_token(
    token: &str,
    rng: &mut ThreadRng,
    keywords: &HashSet<&str>,
    macros: &HashSet<&str>,
    types: &HashSet<&str>,
) -> String {

    let (mut x, mut y) = get_random(rng, ODDS);
    let mut hit = x == y;
    match token {
        token if keywords.contains(token) && hit => return swap_words(&keywords, rng),
        token if macros.contains(token) && hit => return swap_words(&macros, rng),
        token if types.contains(token) && hit => return swap_words(&types, rng),
        token if token != "\n" => {
            for i in 0..4 {
                (x, y) = get_random(rng, ODDS);
                hit = x == y;
                match i {
                    0 if hit => return reverse_token(token.to_string()),
                    1 if hit => return delete_single_char(token.to_string(), rng),
                    2 if hit => return duplicate_token(token.to_string()),
                    3 if hit => return String::new(),
                    _ => ()
                }
            }
        }
        _ => return token.to_string()
    }

    token.to_string()
}

fn iter_file(data: String) -> String {
    let (lines, mut rng) = (
        data.split(|c| c == '\n'),
        rand::thread_rng(),
    );

    // Remove file contents.
    let (x, y) = get_random(&mut rng, ODDS * 100 + 100000);
    if x == y {
        return String::new();
    }

    let keywords: HashSet<&str> = KEYWORDS.into_iter().collect();
    let macros: HashSet<&str> = MACROS.into_iter().collect();
    let types: HashSet<&str> = TYPES.into_iter().collect();

     let mut res = lines
        .into_iter()
        .map(|line| {
            let tokens = line.split(' ');
            let processed_tokens: String = tokens
                .map(|token| process_token(token, &mut rng, &keywords, &macros, &types))
                .collect::<Vec<String>>()
                .join(" ");
            format!("{}\n", processed_tokens)
        })
        .collect::<String>();
    res.pop();
    res
}

fn compile(filepath: &str, compiler: &str, arguments: &str) {
    let mut cmd = Command::new(compiler);
    cmd.arg(filepath);
    cmd.args(arguments.split_whitespace());
    let output = cmd
        .output()
        .expect("Could not compile code");
    if output.status.success() {
        println!("Congratulations! Your code compiles!");
    } else {
        let errmsg = String::from_utf8_lossy(&output.stderr);
        println!("Uh oh. Looks like it didn't compile D:\nReason: {}", errmsg);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("ERROR: usage: ./feeling_lucky <code.x> <compiler> <compiler args>");
        process::exit(1);
    }

    let (filepath, compiler, arguments) = (&args[1], &args[2], &args[3]);
    match fs::read_to_string(filepath) {
        Ok(data) => {
            let result = iter_file(data);
            fs::write(filepath, result)?;
            compile(filepath, compiler, arguments);
            Ok(())
        }
        Err(error) => {
            eprintln!("ERROR: {}", error);
            process::exit(1);
        }
    }
}
