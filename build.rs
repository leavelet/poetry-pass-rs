use std::env;
use std::fs;
use std::path::Path;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=data/");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");
    let mut f = fs::File::create(&dest_path).unwrap();
    
    let poetry_content = fs::read_to_string("data/poetry.txt")
        .unwrap_or_else(|_| include_str!("data/poetry.txt").to_string());
    
    let poetry_lines: Vec<&str> = poetry_content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .collect();
    
    let words_content = fs::read_to_string("data/words.txt")
        .unwrap_or_else(|_| include_str!("data/words.txt").to_string());
    
    let words_lines: Vec<&str> = words_content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .collect();
    
    writeln!(f, "/// Auto generated from data files").unwrap();
    writeln!(f, "pub const POETRY: &[&str] = &[").unwrap();
    for line in poetry_lines {
        writeln!(f, "    \"{}\",", line.trim()).unwrap();
    }
    writeln!(f, "];").unwrap();
    
    writeln!(f, "\npub const WORDS: &[&str] = &[").unwrap();
    for line in words_lines {
        writeln!(f, "    \"{}\",", line.trim()).unwrap();
    }
    writeln!(f, "];").unwrap();
}