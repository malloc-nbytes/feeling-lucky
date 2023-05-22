use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

use rand::rngs::ThreadRng;
use rand::Rng;

const MACROS: [&str; 6] = ["#include", "#define", "#if", "#ifdef", "#ifndef", "#endif"];

const KEYWORDS: [&str; 5] = ["main", "for", "while", "void", "return"];

const TYPES: [&str; 9] = [
    "int", "char", "float", "long", "byte", "short", "double", "float", "bool",
];

fn reverse_token(token: String) -> String {
    token.chars().rev().collect()
}

fn get_random(rng: &mut ThreadRng, upper: u32) -> (u32, u32) {
    (rng.gen_range(0..upper), rng.gen_range(0..upper))
}

fn delete_single_char(token: String, rng: &mut ThreadRng) -> String {
    let mut arr: Vec<char> = token.chars().collect();
    if token.len() > 0 {
        arr.remove(rng.gen_range(0..120) % token.len());
    }
    arr.into_iter().collect()
}

fn process_token(
    token: &str,
    rng: &mut ThreadRng,
    keywords: &HashSet<&str>,
    macros: &HashSet<&str>,
    types: &HashSet<&str>,
) -> String {
    let mut res = String::new();

    if keywords.contains(token) {
    } else if macros.contains(token) {
    } else if types.contains(token) {
    } else if token != " " {
        let (x, y) = get_random(rng, 10);
        if x == y {
            return reverse_token(token.to_string());
        }
        let (x, y) = get_random(rng, 50);
        if x == y {
            return delete_single_char(token.to_string(), rng);
        }
    } else {
        res += token;
    }

    // res
    token.to_string()
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn iter_file(data: String) -> String {
    let (mut res, lines, mut rng) = (
        String::new(),
        data.split(|c| c == '\n').filter(|s| !s.is_empty()),
        rand::thread_rng(),
    );

    let keywords: HashSet<&str> = KEYWORDS.into_iter().collect();
    let macros: HashSet<&str> = MACROS.into_iter().collect();
    let types: HashSet<&str> = TYPES.into_iter().collect();

    for line in lines {
        let tokens = line.split(' ');
        for token in tokens {
            res += &process_token(token, &mut rng, &keywords, &macros, &types);
            res += " ";
        }
        res += "\n";
    }

    println!("{res}");

    res
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("ERROR: usage: ./feeling_lucky <code.c>");
        process::exit(1);
    }

    let filepath = &args[1];

    match fs::read_to_string(filepath) {
        Ok(data) => {
            let result = iter_file(data);
            unimplemented!();
            fs::write(filepath, result)?;
            Ok(())
        }
        Err(error) => {
            eprintln!("ERROR: {}", error);
            process::exit(1);
        }
    }
}
