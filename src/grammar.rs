use codemap::CodeMap;
use codemap_diagnostic::{ColorConfig, Diagnostic, Emitter, Level, SpanLabel, SpanStyle};
use rust_sitter::errors::{ParseError, ParseErrorReason};

#[rust_sitter::grammar("command")]
pub mod ast {
    #[rust_sitter::language]
    pub enum ProgramExpr {
        Expression(Box<EvalExpr>),
    }

    #[rust_sitter::language]
    pub enum EvalExpr {
        Number(#[rust_sitter::leaf(pattern = r"(\d+|0x[0-9a-fA-F]+)", transform = parse_int)] u64),
        #[rust_sitter::prec_left(1)]
        Add(
            Box<EvalExpr>,
            #[rust_sitter::leaf(text = "+")] (),
            Box<EvalExpr>,
        ),
    }

    #[rust_sitter::extra]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }

    fn parse_int(text: &str) -> u64 {
        let text = text.trim();
        if text.starts_with("0x") {
            let text = text.split_at(2).1;
            u64::from_str_radix(text, 16).unwrap()
        } else {
            text.parse().unwrap()
        }
    }
}
