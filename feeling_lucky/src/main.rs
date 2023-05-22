use std::env;
use std::fs;
use std::process;
use std::process::Command;
use std::collections::HashSet;

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

const TYPES: [&str; 14] = [
    "int", "char", "float",
    "long", "uint64_t", "short",
    "double", "float", "bool",
    "void", "unsigned", "signed",
    "size_t", "uint32_t",
];

const MSGS: [&str; 27] = [
    "[code has been marked as duplicate]",
    "pls learn to code",
    r#"
| |╷ 

|| |_
"#,
    "getoutofmyhead getoutofmyhead getoutofmyhead getoutofmyhead getoutofmyhead getoutofmyhead getoutofmyhead ",
    "UwU",
    "OwO",
    "i showed you my code, pls respond :'(",
    "Gone. Reduced to atoms... also rip atom",
    "This is so sad. F",
    "ayo?",
    "Actual speghetti code",
    "Emacs > Vim",
    r#"
Caught you in 8K UHD surround sound 16 Gigs ram, HDR GEFORCE RTX, TI-80 texas insturments, Triple A duracell battery ultrapower100 Cargador Compatible
iPhone 1A 5 W 1400 + Cable 100% 1 Metro Blanco Compatible iPhone 5 5 C 5S 6 SE 6S 7 8 X XR XS XS MAX GoPro hero 1 2 terrabyte xbox series x Dell
UltraSharp 49 Curved Monitor - U4919DW Sony HDC-3300R 2/3" CCD HD Super Motion Color Camera, 1080p Resolution Toshiba EM131A5C-SS Microwave Oven with
Smart Sensor, Easy Clean Interior, ECO Mode and Sound On/Off, 1.2 Cu. ft, Stainless Steel HP LaserJet Pro M404n Monochrome Laser Printer with Built-in
Ethernet (W1A52A) GE Voluson E10 Ultrasound Machine LG 23 Cu. Ft. Smart Wi-Fi Enabled InstaView Door-in-Door Counter-Depth Refrigerator with Craft Ice
Maker GFW850SPNRS GE 28" Front Load Steam Washer 5.0 Cu. Ft. with SmartDispense, WiFi, OdorBlock and Sanitize and Allergen - Royal Sapphire Kohler K-3589
Cimarron Comfort Height Two-Piece Elongated 1.6 GPF Toilet with AquaPiston Flush Technology., Quick Charge 30W Cargador 3.0 Cargador de Viaje Enchufe
Cargador USB Carga Rápida con 3 Puertos carga rápida Adaptador de Corriente para iPhone x 8 7 Xiaomi Pocophone F1 Mix 3 A1 Samsung S10 S9 S8AUKEY Quick
Charge 3.0 Cargador de Pared 39W Dual Puerto Cargador Móvil para Samsung Galaxy S8 / S8+/ Note 8, iPhone XS / XS Max / XR, iPad Pro / Air, HTC 10, LG G5
/ G6 AUKEY Quick Charge 3.0 Cargador USB 60W 6 Puerto Cargador Móvil para Samsung Galaxy S8 / S8+ / Note 8, LG G5 / G6, Nexus 5X / 6P, HTC 10, iPhone XS
/ XS Max / XR, iPad Pro/ Air, Moto G4 SAMSUNG 85-inch Class Crystal UHD TU-8000 Series - 4K UHD HDR Smart TV with Alexa Built-in (UN85TU8000FXZA, 2020
Model) GE 38846 Premium Slim LED Light Bar, 18 Inch Under Cabinet Fixture, Plug-In, Convertible to Direct Wire, Linkable 628 Lumens, 3000K Soft Warm
White, High/Off/Low, Easy to Install, 18 Ft Bissell Cleanview Swivel Pet Upright Bagless
"#,
    "Vim > Emacs",
    "VSCode moment frfr",
    "Waltuh, put the code away Waltuh",
    "YIPPE",
    "nah that's cap",
    "(ᗒᗣᗕ)՞ ... (ノಠൠಠ)ノ彡┻━┻ ... ಠ_ಠ ┻━┻ ... (ノಠ_ಠ)ノ ┻━┻ ... ┬─┬ノ(ಠ_ಠノ) ... (ಥ﹏ಥ) ",
    "AAAAHHHHHHHHHHHHHHHHHHHHHHHHHHHHH",
    "L L L L L L L L L L L L L L L L L",
    "All errors any% speedrun",
    "\"jUsT UsE pyTHoN bRo\"",
    "\n^^^^The code above is cringe^^^^^\n",
    "\n#include <stdio.h>\n\nint main(void) {\n  printf(\"Hello sadness!\");\n  return 0;\n}\n",
    "no code?",
r#"my honest reaction to that information:
@@@@@@@@@@@@@@@@@@@@@@@#+-:............:-=+#@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@*..-=================-..:+%@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@=.:*-::::::::::::::::::=++:.:#@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@+..#:::::::::::::::::::::::=*..=@@@@@@@@@
@@@@@@@@@@@@@@@@@@%..*=:::::=++++++++===-:::::::#:.-@@@@@@@@
@@@@@@@@@@@@@@@@@@=.:#::::*#**------:::-+*+::::::#..+@@@@@@@
@@@@@@@@@@@@@@@@@@..*-::::##***==---------*=:::::=+..@@@@@@@
@@@@@@@@@@@@@@@@@+..#::::::=*###*******#**=:::::::#..*@@@@@@
@@@@@@@@@@@@@@@@@..++::::::::::-=====--:::::::::::+-.:@@@@@@
@@@@@@@@@@@@@@@@*..%::::::::::::::::::::::::::::::-#..%@@@@@
@@@@@@@@@@@@@@@@-.-*:::::::::::::::::::::::::::::::%..+@@@@@
@@@@@@@@@@@@@@@@..*-:::::::::::::::::::::::::::::::*-.-@@@@@
@@@@@@@@@@@@@@@*..%::::::::::::::::::::::::::::::::=+ .@@@@@
@@@@@@@@@@@@@@@=.:*:::::::::::::::::::::::::::::::::#..%@@@@
@@@@@@@@@@@@@@@. +=:::::::::::::::::::::::::::::::::%..*@@@@
@@@@@@@@@@@@@@%..#-:::::::::::::::::::::::::::::::::#..+@@@@
@@@@@@@@@@@@@@*..%:::::::::-===========:::::::::::::*-.-@@@@
@@@@@@@@@@@@@@=.:#:::::::++:...:::::..:#::::::::::::+= -@@@@
@@@@@@#=:.......=+:::::::=*..*@@@@@@*..#::::::::::::=+ -@@@@
@@@@%:.:+++=====*-::::::::=+..@@@@@@#..#::::::::::::-* -@@@@
@@@@=.-*:::::::::::::::::::%..+*=:.....#::::::::::::-* -@@@@
@@@@=.=+::::::::::::::::::-#....========::::::::::::=* -@@@@
@@@@#..=+=:::::::::::::-=+=....#:::::::::::::::::::-*..=@@@@
@@@@@@+:.:-============:..-++..#-::::::::::::::-=++-..*@@@@@
@@@@@@@@@%##*********##%@@@@@#=---------------====+#%@@@@@@@"#
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

fn swap_words(words: &HashSet<&str>, rng: &mut ThreadRng) -> String {
    words
        .iter()
        .cloned()
        .collect::<Vec<_>>()
        .choose(rng)
        .unwrap()
        .to_string()
}

fn random_msg(rng: &mut ThreadRng) -> String {
    return MSGS[rng.gen_range(0..MSGS.len())].to_string();
}

fn process_token(
    token: &str,
    rng: &mut ThreadRng,
    keywords: &HashSet<&str>,
    macros: &HashSet<&str>,
    types: &HashSet<&str>,
    odds: u32
) -> String {

    let (x, y) = get_random(rng, odds);
    let hit = x == y;
    match token {
        token if keywords.contains(token) && hit => return swap_words(&keywords, rng),
        token if macros.contains(token) && hit => return swap_words(&macros, rng),
        token if types.contains(token) && hit => return swap_words(&types, rng),
        token if token != "\n" => {
            match rng.gen_range(0..5) {
                0 if hit => return reverse_token(token.to_string()),
                1 if hit => return delete_single_char(token.to_string(), rng),
                2 if hit => return duplicate_token(token.to_string()),
                3 if hit => return random_msg(rng),
                4 if hit => return String::new(),
                _ => ()
            }
        }
        _ => return token.to_string()
    }

    token.to_string()
}

fn iter_file(data: String, odds: u32) -> String {
    let (lines, mut rng) = (
        data.split(|c| c == '\n'),
        rand::thread_rng(),
    );

    // Remove file contents O_O.
    let (x, y) = get_random(&mut rng, 100000);
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
                .map(|token| process_token(token, &mut rng, &keywords, &macros, &types, odds))
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

fn usage() {
    eprintln!("ERROR: usage: ./feeling_lucky <code.x> <compiler> <compiler args> <optional odds>");
    process::exit(1);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut odds = 1000u32;

    if args.len() < 4 || args.len() > 5 {
        usage();
    }

    if args.len() == 5 {
        odds = args[4].parse::<u32>().unwrap();
    }

    let (filepath, compiler, arguments) = (&args[1], &args[2], &args[3]);
    match fs::read_to_string(filepath) {
        Ok(data) => {
            let result = iter_file(data, odds);
            println!("{result}");
            // fs::write(filepath, result)?;
            compile(filepath, compiler, arguments);
            Ok(())
        }
        Err(error) => {
            eprintln!("ERROR: {}", error);
            process::exit(1);
        }
    }
}
