use std::{
    env,
    path::Path,
    process::{self},
};

mod pgn;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <file_path> <output_path>", args[0]);
        process::exit(1);
    }

    match args[1].as_str() {
        "extract" => {
            let input = Path::new(&args[2]);
            let output = Path::new(&args[3]);

            pgn::extract_pgn(input, output);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Usage: {} extract <input_file> <output_file>", args[0]);
            process::exit(1);
        }
    }
}
