use std::env;
use std::error::Error;
use std::fs;
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
    let search_input: &str = &args[2].trim();
    let output = fs::read_to_string(snippets_path)?;
    let snippets = output.split("%~\n");
    for snip in snippets {
        if snip.contains(search_input) {
            print!("{}", snip);
            break;
        }
    }
    Ok(())
}
