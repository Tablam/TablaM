use std::env;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use seahorse::{App, Command, Context};

use std::fs::File;
use std::path::PathBuf;
use tablam::stdlib::io::read_file_to_string;
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

fn run_file(c: &Context) {
    if let Some(f) = c.args.iter().next() {
        let path = PathBuf::from(f);
        match File::open(path) {
            Ok(mut f) => {
                let code = read_file_to_string(&mut f).expect("Fail to read file");
                let program = Program::new();
                match program.execute_str(code.as_str()) {
                    Ok(expr) => match expr {
                        Expression::Pass => (),
                        Expression::Eof => (),
                        expr => println!("{}", expr),
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    } else {
        eprintln!("Error: Need a file name");
    }
}

fn file_command() -> Command {
    Command::new("--file")
        .alias("-f")
        .usage("--file name.tbl")
        .action(run_file)
}

fn run_repl(c: &Context) {
    if c.args.len() > 0 {
        c.help();
        return;
    }
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("tablam")
        .author("https://www.tablam.org")
        .version(env!("CARGO_PKG_VERSION"))
        .usage("tablam [command]")
        .action(run_repl)
        .command(file_command());

    app.run(args);
}
