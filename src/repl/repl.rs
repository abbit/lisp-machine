use std::ops::Deref;
use std::path::Path;

use lispdm::{Engine, Expr};
use rustyline::history::FileHistory;
use rustyline::{error::ReadlineError, validate::MatchingBracketValidator};
use rustyline::{highlight::MatchingBracketHighlighter, Editor};
use rustyline::{Completer, CompletionType, Helper, Highlighter, Hinter, Validator};

#[derive(Default, Validator, Helper, Completer, Hinter, Highlighter)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,
}

pub fn start(mut engine: Engine) {
    println!("LispDM v0.0.1");
    println!("Use (exit), or Ctrl-D to exit REPL");

    let expanded_path = shellexpand::tilde("~/.lispdm_history");
    let history_path = Path::new(expanded_path.deref());
    if !history_path.exists() {
        std::fs::File::create(history_path).expect("Failed to create history file");
    }
    let mut rl = Editor::with_history(
        rustyline::config::Config::builder()
            .auto_add_history(true)
            .completion_type(CompletionType::List)
            .build(),
        FileHistory::new(),
    )
    .expect("Failed to instantiate REPL!");
    rl.load_history(&history_path)
        .expect("Failed to load history");
    rl.set_helper(Some(InputValidator::default()));

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(input) => {
                if input.trim().is_empty() {
                    continue;
                }
                rl.save_history(&history_path)
                    .expect("Failed to save history");
                match engine.eval::<Expr>(&input) {
                    Ok(expr) => {
                        println!("{}", expr.unwrap());
                    }
                    Err(err) => println!("Error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                continue;
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
