use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

fn get_path(path: &Path) -> String {
    let result = fs::read_to_string(path);

    return result.unwrap();
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 1 {
        println!("Usage: {} <path>", args[0]);
        exit(1);
    }

    let path = Path::new(args[1].as_str());

    println!("{}", get_path(&path));
    exit(0);
}

