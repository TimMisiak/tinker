use std::io::Write;

use codemap::CodeMap;
use codemap_diagnostic::{ColorConfig, Diagnostic, Emitter, Level, SpanLabel, SpanStyle};
use rust_sitter::errors::{ParseError, ParseErrorReason};

use crate::grammar::parse_tree;

pub fn read_command() -> parse_tree::ProgramExpr {
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if !input.is_empty() {
            let cmd = parse_tree::parse(&input);
            match cmd {
                Ok(c) => return c,
                Err(errs) => {
                    // This came from https://github.com/hydro-project/rust-sitter/blob/main/example/src/main.rs
                    let mut codemap = CodeMap::new();
                    let file_span = codemap.add_file("<input>".to_string(), input.to_string());
                    let mut diagnostics = vec![];
                    for error in errs {
                        convert_parse_error_to_diagnostics(
                            &file_span.span,
                            &error,
                            &mut diagnostics,
                        );
                    }

                    let mut emitter = Emitter::stderr(ColorConfig::Always, Some(&codemap));
                    emitter.emit(&diagnostics);
                }
            }
        }
    }
}


// This came from https://github.com/hydro-project/rust-sitter/blob/main/example/src/main.rs
fn convert_parse_error_to_diagnostics(
    file_span: &codemap::Span,
    error: &ParseError,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match &error.reason {
        ParseErrorReason::MissingToken(tok) => diagnostics.push(Diagnostic {
            level: Level::Error,
            message: format!("Missing token: \"{tok}\""),
            code: Some("S000".to_string()),
            spans: vec![SpanLabel {
                span: file_span.subspan(error.start as u64, error.end as u64),
                style: SpanStyle::Primary,
                label: Some(format!("missing \"{tok}\"")),
            }],
        }),

        ParseErrorReason::UnexpectedToken(tok) => diagnostics.push(Diagnostic {
            level: Level::Error,
            message: format!("Unexpected token: \"{tok}\""),
            code: Some("S000".to_string()),
            spans: vec![SpanLabel {
                span: file_span.subspan(error.start as u64, error.end as u64),
                style: SpanStyle::Primary,
                label: Some(format!("unexpected \"{tok}\"")),
            }],
        }),

        ParseErrorReason::FailedNode(errors) => {
            if errors.is_empty() {
                diagnostics.push(Diagnostic {
                    level: Level::Error,
                    message: "Failed to parse node".to_string(),
                    code: Some("S000".to_string()),
                    spans: vec![SpanLabel {
                        span: file_span.subspan(error.start as u64, error.end as u64),
                        style: SpanStyle::Primary,
                        label: Some("failed".to_string()),
                    }],
                })
            } else {
                for error in errors {
                    convert_parse_error_to_diagnostics(file_span, error, diagnostics);
                }
            }
        }
    }
}