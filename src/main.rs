use std::env;
use std::process;

mod parser;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <file.jump>");
        process::exit(1);
    }

    let filename = &args[1];
    let grid = parser::parse(filename).unwrap_or_else(|e| {
        eprintln!("Error parsing {}: {}", filename, e);
        process::exit(1);
    });

    let mut vm = vm::VM::new(grid);
    vm.run();
}
