use lispdm::Engine;
use rustyline::history::MemHistory;
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

    let mut rl = Editor::with_history(
        rustyline::config::Config::builder()
            .auto_add_history(true)
            .completion_type(CompletionType::List)
            .build(),
        MemHistory::default(),
    )
    .expect("Failed to instantiate REPL!");
    rl.set_helper(Some(InputValidator::default()));

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(input) => {
                if input.trim().is_empty() {
                    continue;
                }

                match engine.eval(&input) {
                    Ok(expr) => {
                        println!("{}", expr);
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
