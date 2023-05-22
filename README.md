# feeling-lucky

### This project is a joke. Do not use it in any serious manner. This ___will___ modify the file you provide. Make sure a backup exists.
Feeling lucky? Then this is for you! Just provide a filepath to some code (c is most supported but any language works), provide the compiler/interpreter, any arguments for the compiler/interpreter, and (optionally) the odds of this program messing with the code.

## Prerequisites
Before running this project, make sure you have the following prerequisites installed on your system:

* Rust: You can install Rust by following the instructions at https://www.rust-lang.org/tools/install.

## Getting Started
To get started with this project, follow these steps:

1. Clone the repository to your local machine:
```git clone https://github.com/malloc-nbytes/feeling-lucky.git
```
2. Navigate to the project directory:
```cd feeling-lucky
```
3. Build the project using Cargo, the package manager and build tool for Rust:
```cargo build
```
This will compile the project and its dependencies.

## Running the Project
To run the project, use the following command:
```cargo run -- <path/to/source/code> <compiler> <compiler args> <(optional) odds>
```

### Example
Lets say I have a file called `~/dev/code.c`. I want to compile it using the GNU Compiler Collection (GCC) with the arguments `-o main -Wall -Wextra`.
To run it, it would look like this:
```cargo run -- ~/dev/code.c gcc "-o main -Wall -Wextra"
```
If you want to increase/decrease the odds of it modifying the file (By default, it is set to 1000), provide a number as the _last_ argument. For example:
```cargo run -- ~/dev/code.c gcc "-o main -Wall -Wextra" 100
```
At the moment, there are 9 different modifications that the program can make to each word. This means that each word will have a 1/100 chance to be changed per modification option, or 1 - (99/100)^9 = ~8.66%. Set the odds to 1 for extra fun.

Also, there is a 1/100000 chance to just remove everything in the file. This cannot be changed.
