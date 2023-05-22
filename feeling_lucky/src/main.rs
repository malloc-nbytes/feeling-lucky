use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

const MACROS: [&str; 6] = ["#include", "#define", "#if", "#ifdef", "#ifndef", "#endif"];

const KEYWORDS: [&str; 4] = ["main", "for", "while", "return"];

const TYPES: [&str; 10] = [
    "int", "char", "float", "long", "byte", "short", "double", "float", "bool", "void",
];

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

fn swap_macros(macros: &HashSet<&str>, rng: &mut ThreadRng) -> String {
    macros
        .iter()
        .cloned()
        .collect::<Vec<_>>()
        .choose(rng)
        .unwrap()
        .to_string()
}

fn swap_types(types: &HashSet<&str>, rng: &mut ThreadRng) -> String {
    types
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

    let (mut x, mut y) = get_random(rng, 50);
    match token {
        token if keywords.contains(token) => todo!(),
        token if macros.contains(token) && x == y => return swap_macros(&macros, rng),
        token if types.contains(token) && x == y => return swap_types(&types, rng),
        token if token != "\n" => {
            for i in 0..4 {
                (x, y) = get_random(rng, 50);
                match i {
                    0 if x == y => return reverse_token(token.to_string()),
                    1 if x == y => return delete_single_char(token.to_string(), rng),
                    2 if x == y => return duplicate_token(token.to_string()),
                    3 if x == y => return String::new(),
                    _ => ()
                }
            }
        }
        _ => return token.to_string()
    }

    token.to_string()
}

fn iter_file(data: String) -> String {
    let (mut res, lines, mut rng) = (
        String::new(),
        data.split(|c| c == '\n'),
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("ERROR: usage: ./feeling_lucky <code.c>");
        process::exit(1);
    }

    let filepath = &args[1];

    match fs::read_to_string(filepath) {
        Ok(data) => {
            let _result = iter_file(data);
            unimplemented!();
            fs::write(filepath, _result)?;
            Ok(())
        }
        Err(error) => {
            eprintln!("ERROR: {}", error);
            process::exit(1);
        }
    }
}
