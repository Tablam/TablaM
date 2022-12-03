use ariadne::{sources, Color, ColorGenerator, Config, Label, Report, ReportKind};
use corelib::errors::Span;
use std::io;
use std::ops::Range;

use crate::errors::ErrorCode;
use parser::errors::ErrorParser;
use parser::files::FilesDb;

fn build_label(
    span: &Span,
    named: String,
    color: Color,
    msg: &str,
    with_color: bool,
) -> Label<(String, Range<usize>)> {
    let label = Label::new((named, span.range())).with_message(msg);

    if with_color {
        label.with_color(color)
    } else {
        label
    }
}

pub fn build_report(
    named: String,
    err: &ErrorParser,
    with_color: bool,
) -> Report<(String, Range<usize>)> {
    let mut colors = ColorGenerator::new();

    // Generate some colours for each of our elements
    let primary = colors.next();
    // let secondary = colors.next();
    let code = err.error_code();
    let config = Config::default().with_color(with_color);
    let diagnostic = Report::build(ReportKind::Error, named.clone(), 0)
        .with_code(code as usize)
        .with_config(config);

    match err {
        ErrorParser::ScalarParse { span, kind, msg } => diagnostic
            .with_message(msg)
            .with_label(build_label(span, named, primary, msg, with_color))
            .with_note(format!("Parsing value of type: {kind:?}"))
            .finish(),
        _ => {
            todo!()
        }
    }
}

pub fn print_diagnostic_to_str(src: &FilesDb, err: &ErrorCode) -> io::Result<String> {
    let name = src.get_root().name();
    match err {
        ErrorCode::Parser { error } => {
            let err = build_report(name, error, false);
            let mut c = Vec::new();
            err.write(
                sources(src.files().map(|x| (x.data.name(), x.data.source()))),
                &mut c,
            )?;
            let s = String::from_utf8(c).unwrap();
            Ok(s)
        }
    }
}

pub fn print_diagnostic(src: &FilesDb, err: &ErrorCode) -> io::Result<()> {
    let name = src.get_root().name();
    match err {
        ErrorCode::Parser { error } => {
            let err = build_report(name, error, true);
            err.print(sources(
                src.files().map(|x| (x.data.name(), x.data.source())),
            ))
        }
    }
}
