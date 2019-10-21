use std::env;
use std::error::Error;
use std::fs;
use toml::Value;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = run() {
        eprintln!("Error processing: {}", e);
        process::exit(1);
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let snippets_path: &str = &args[1];
    let lang: &str = &args[2];
    let hotkey: &str = &args[3];
    let contents =
        fs::read_to_string(snippets_path).expect("Something went wrong reading the file");
    let mut contents_toml = contents.parse::<Value>()?;
    let snippet = &mut contents_toml[lang][hotkey].to_string();
    snippet.pop();
    let snippet = snippet.replace("\\", "");
    print!("{}", &snippet[1..]);
    Ok(())
}

