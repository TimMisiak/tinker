
#[rust_sitter::grammar("command")]
pub mod parse_tree {
    #[derive(Debug)]
    #[rust_sitter::language]
    pub enum ProgramExpr {
        Expression(Box<AddExpr>),
    }

    #[derive(Debug)]
    #[rust_sitter::language]
    pub enum AddExpr {
        #[rust_sitter::prec_left(1)]
        Add(
            Box<MultExpr>,
            #[rust_sitter::leaf(text = "+")] (),
            Box<AddExpr>,
        ),
        #[rust_sitter::prec_left(1)]
        Subtract(
            Box<MultExpr>,
            #[rust_sitter::leaf(text = "-")] (),
            Box<AddExpr>,
        ),
        MultExp(Box<MultExpr>)
    }

    #[derive(Debug)]
    #[rust_sitter::language]
    pub enum MultExpr {
        #[rust_sitter::prec_left(1)]
        Multiply(
            Box<BasicExpr>,
            #[rust_sitter::leaf(text = "*")] (),
            Box<MultExpr>,
        ),
        #[rust_sitter::prec_left(1)]
        Divide(
            Box<BasicExpr>,
            #[rust_sitter::leaf(text = "/")] (),
            Box<MultExpr>,
        ),
        BasicExp(Box<BasicExpr>)
    }

    #[derive(Debug)]
    #[rust_sitter::language]
    pub enum BasicExpr {
        Number(#[rust_sitter::leaf(pattern = r"(\d+|0x[0-9a-fA-F]+)", transform = parse_int)] u64),
        #[rust_sitter::prec_left(1)]
        Parens(
            #[rust_sitter::leaf(text = "(")] (),
            Box<AddExpr>,
            #[rust_sitter::leaf(text = ")")] (),
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
