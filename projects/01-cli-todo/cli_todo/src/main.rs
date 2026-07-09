use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("program: {}", args[0]);
    if let Err(msg) = run(&args[1..]) {
        eprintln!("error: {msg}");
        process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    println!("received: {:?}", args);
    Ok(())
}
