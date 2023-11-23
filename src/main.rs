use lispdm::{environment, interpreter::eval};
use rustyline::{error::ReadlineError, DefaultEditor};

const PROMPT: &str = "lispdm> ";

fn main() {
    let mut env = environment::new_root_env();

    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline(PROMPT);
        match readline {
            Ok(input) => {
                if input.eq("exit") {
                    break;
                }

                match eval(input.as_ref(), &mut env) {
                    Ok(expr) => println!("{}", expr),
                    Err(err) => println!("Error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
