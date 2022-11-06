use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{env, fs, io};

use corelib::errors::Span;
use corelib::prelude::{Scalar, VERSION};
use eval::code::Code;
use eval::diagnostic::print_diagnostic;
use eval::errors::ErrorCode;
use eval::program::{create_file, read_file_to_string, Program};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use seahorse::{App, Command, Context};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

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

enum Execute {
    Pass,
    Halt((ErrorCode, Span)),
    Value(Scalar),
    Eof,
}

fn run_code(p: &Program) -> Execute {
    match p.eval() {
        Code::Root => Execute::Pass,
        Code::Scalar { val, .. } => Execute::Value(val),
        Code::If { .. } => Execute::Pass,
        Code::Halt { error, span } => Execute::Halt((error, span)),
        Code::Eof => Execute::Eof,
    }
}

fn run_file(c: &Context) {
    if let Some(f) = c.args.first() {
        let path = PathBuf::from(f);
        match File::open(path.clone()) {
            Ok(mut f) => {
                //TODO: Fill this
                let code = read_file_to_string(&mut f).expect("Fail to read file");
                let file = create_file(path, &code);

                let program = Program::from_file(file);
                match run_code(&program) {
                    Execute::Pass => {}
                    Execute::Halt((err, span)) => eprintln!("{:?}", err),
                    Execute::Value(_) => {}
                    Execute::Eof => {}
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    } else {
        eprintln!("Error: Need a file name");
    }
}

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
    let mut program = Program::new();

    loop {
        let readline = rl.readline("\x1b[1;32m>\x1b[0m ");
        match readline {
            Ok(line) => match line.as_str().trim() {
                "" => continue,
                "exit" => break,
                "help" => println!("Help & more info at https://www.tablam.org"),
                line => {
                    rl.add_history_entry(line);
                    //dbg!(&line);
                    match program.append_from_src(line) {
                        Ok(_) => match run_code(&program) {
                            Execute::Pass => continue,
                            Execute::Halt((err, span)) => eprintln!("{:?}", err),
                            Execute::Value(x) => {
                                println!("{}", x)
                            }
                            Execute::Eof => break,
                        },
                        Err(err) => print_diagnostic(&program.files, &err)
                            .expect("Fail to report diagnostics"),
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
