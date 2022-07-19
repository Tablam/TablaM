use std::env;
use std::fs::File;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use seahorse::{App, Command, Context};
use syntect::easy::HighlightLines;

use corelib::prelude::VERSION;
use eval::ast::Expr;
use eval::program::Program;

fn print_welcome() {
    print!(
        r#"TablaM: The practical relational language
    Home: http://www.tablam.org
    Version: {} (experimental)
    
    Type: "help", "exit" or write tablam code...
"#,
        VERSION
    );
}

fn run_file(c: &Context) {
    if let Some(f) = c.args.first() {
        let path = PathBuf::from(f);
        match File::open(path) {
            Ok(mut f) => {
                //TODO: Fill this
                //let code = read_file_to_string(&mut f).expect("Fail to read file");
                let code = String::new();
                let mut program = Program::new(&code);
                match program.execute_str(code.as_str()) {
                    Ok(expr) => match expr {
                        Expr::Pass(_) => (),
                        Expr::Eof(_) => (),
                        expr => println!("{:?}", expr),
                    },
                    Err(err) => eprintln!("{:?}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    } else {
        eprintln!("Error: Need a file name");
    }
}

use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_file;
use syntect::parsing::SyntaxSet;

fn run_repl(c: &Context) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    if !c.args.is_empty() {
        c.help();
        return;
    }
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(".history.txt").is_err() {
        println!("No previous history.");
    }
    print_welcome();
    let mut program = Program::new("");

    loop {
        let readline = rl.readline("\x1b[1;32m>\x1b[0m ");
        match readline {
            Ok(line) => match line.as_str().trim() {
                "" => continue,
                "exit" => break,
                "help" => println!("Help & more info at http://www.tablam.org"),
                line => {
                    rl.add_history_entry(line);
                    //dbg!(&line);
                    match program.execute_str(line) {
                        Ok(expr) => match expr {
                            Expr::Pass(_) => continue,
                            Expr::Eof(_) => {
                                break;
                            }
                            expr => println!("{:?}", expr),
                        },
                        Err(err) => eprintln!("Error: {:?}", err),
                    }
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                return;
            }
        }
    }
    println!("bye!");
    rl.save_history(".history.txt").unwrap();
}

fn file_command() -> Command {
    Command::new("--file")
        .alias("-f")
        .usage("--file name.tbl")
        .action(run_file)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("tablam")
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("tablam [command]")
        .action(run_repl)
        .command(file_command());

    app.run(args);

    // use reedline::{DefaultPrompt, Reedline, Signal};
    //
    // let mut line_editor = Reedline::create();
    // let prompt = DefaultPrompt::default();
    //
    // loop {
    //     let sig = line_editor.read_line(&prompt);
    //     match sig {
    //         Ok(Signal::Success(buffer)) => {
    //             println!("We processed: {}", buffer);
    //         }
    //         Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
    //             println!("\nAborted!");
    //             break;
    //         }
    //         x => {
    //             println!("Event: {:?}", x);
    //         }
    //     }
    // }
}
