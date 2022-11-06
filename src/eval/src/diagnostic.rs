use ariadne::{sources, ColorGenerator, Label, Report, ReportKind};
use std::io;
use std::ops::Range;

use crate::errors::ErrorCode;
use parser::errors::ErrorParser;
use parser::files::FilesDb;

pub fn build_report(named: String, err: &ErrorParser) -> Report<(String, Range<usize>)> {
    let mut colors = ColorGenerator::new();

    // Generate some colours for each of our elements
    let primary = colors.next();
    // let secondary = colors.next();
    let code = err.error_code();
    let diagnostic = Report::build(ReportKind::Error, named.clone(), 0).with_code(code as usize);

    match err {
        ErrorParser::ScalarParse { span, kind, msg } => diagnostic
            .with_message(msg)
            .with_label(
                Label::new((named, span.range()))
                    .with_message(msg)
                    .with_color(primary),
            )
            .with_note(format!("Parsing value of type: {kind:?}"))
            .finish(),
        _ => {
            todo!()
        }
    }
}

pub fn print_diagnostic(src: &FilesDb, err: &ErrorCode) -> io::Result<()> {
    let name = src.get_root().name();
    match err {
        ErrorCode::Parser { error } => {
            let err = build_report(name, error);
            err.print(sources(
                src.files().map(|x| (x.data.name(), x.data.source())),
            ))
        }
    }
}
