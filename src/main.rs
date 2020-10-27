use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    // create rustyline editor with no completer
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("risp> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("error: {}", err);
            }
        }
    }
}
