mod repl;

use lispdm::{Engine, Expr};

fn print_help() {
    println!("LispDM {}", env!("CARGO_PKG_VERSION"));
    println!("Trying to be Scheme R7RS interpreter");
    println!("Usage: lispdm [FLAGS] [FILE]");
    println!("If no file is given, starts REPL");
    println!("Flags:");
    println!("    -h, --help     Prints this help message");
    println!("    -e, --eval     Evaluates given string");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut engine = Engine::default();

    if args.is_empty() {
        // no args - start REPL
        repl::start(engine);
    } else {
        // check for flags
        match args[0].as_str() {
            // print help
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            // evaluate given string
            "-e" | "--eval" => {
                let src = match args.get(1) {
                    Some(src) => src,
                    None => {
                        println!("Expected string to evaluate for -e flag");
                        std::process::exit(1);
                    }
                };
                match engine.eval::<Expr>(src).unwrap() {
                    Ok(val) => {
                        println!("{}", val);
                        std::process::exit(0);
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        std::process::exit(1);
                    }
                }
            }
            // no flags - interpret file
            _ => {
                let filename = &args[0];
                let src = match std::fs::read_to_string(filename) {
                    Ok(src) => src,
                    Err(err) => {
                        println!("Error: failed to read a file '{}': {}", filename, err);
                        std::process::exit(1);
                    }
                };
                if let Err(err) = engine.eval::<()>(&src) {
                    println!("Error: {}", err);
                    std::process::exit(1);
                }
            }
        }
    }
}
