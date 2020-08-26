use rustyline::error::ReadlineError;
use rustyline::Editor;
use tablam_lang::ast::Expression;
use tablam_lang::eval::Program;

fn print_welcome() {
    print!(
        r#"TablaM: The practical relational language
    Home: http://www.tablam.org
    Version: {} (experimental)
    
    Type: "help", "exit" or write tablam code...
"#,
        env!("CARGO_PKG_VERSION")
    );
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    print_welcome();
    let program = Program::new();

    loop {
        let readline = rl.readline("\x1b[1;32m>\x1b[0m ");
        match readline {
            Ok(line) => match line.as_str() {
                "exit" => break,
                "help" => println!("Help & more info at http://www.tablam.org"),
                line => {
                    rl.add_history_entry(line);
                    dbg!(&line);
                    match program.execute_str(line) {
                        Ok(expr) => match expr {
                            Expression::Pass => continue,
                            Expression::Eof => {
                                break;
                            }
                            expr => println!("{}", expr),
                        },
                        Err(err) => eprintln!("Error: {}", err),
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                return;
            }
        }
    }
    println!("bye!");
    rl.save_history("history.txt").unwrap();
}
